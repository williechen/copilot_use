//! Windows handle-relative NT adapter. No reparse traversal, shared writers, overwrite or purge.
//! https://learn.microsoft.com/windows/win32/api/winternl/nf-winternl-ntcreatefile
//! https://learn.microsoft.com/windows-hardware/drivers/ddi/ntifs/nf-ntifs-ntsetinformationfile
//! https://learn.microsoft.com/windows/win32/api/aclapi/nf-aclapi-getsecurityinfo
use crate::{
    CleanupError, CleanupReceipt, FilePreview, RestoreReceipt, TrashEntry, modified_nanos,
    validate_name,
};
use std::{
    fs::File,
    io::{self, Read, Write},
    os::windows::io::{AsRawHandle, FromRawHandle},
    path::{Path, PathBuf},
};
use uuid::Uuid;

use windows_sys::Win32::Security::ACE_HEADER;
use windows_sys::{
    Wdk::{Foundation::OBJECT_ATTRIBUTES, Storage::FileSystem::*},
    Win32::{
        Foundation::*,
        Security::{Authorization::*, *},
        Storage::FileSystem::{
            BY_HANDLE_FILE_INFORMATION, CreateFileW, FILE_ADD_FILE, FILE_APPEND_DATA,
            FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_NORMAL, FILE_ATTRIBUTE_REPARSE_POINT,
            FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT, FILE_READ_ATTRIBUTES,
            FILE_READ_DATA, FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE, FILE_WRITE_DATA,
            GetFileInformationByHandle, GetFinalPathNameByHandleW, OPEN_EXISTING,
        },
        System::{
            IO::IO_STATUS_BLOCK,
            Threading::{GetCurrentProcess, OpenProcessToken},
        },
    },
};
pub struct PinnedRoot {
    directory: File,
}
struct Local(*mut core::ffi::c_void);
impl Drop for Local {
    fn drop(&mut self) {
        unsafe {
            LocalFree(self.0);
        }
    }
}
fn nt(status: i32) -> Result<(), CleanupError> {
    if status < 0 {
        Err(io::Error::from_raw_os_error(unsafe { RtlNtStatusToDosError(status) } as i32).into())
    } else {
        Ok(())
    }
}
fn user() -> Result<Vec<usize>, CleanupError> {
    let mut token = std::ptr::null_mut();
    if unsafe { OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) } == 0 {
        return Err(io::Error::last_os_error().into());
    }
    let token = unsafe { File::from_raw_handle(token) };
    let mut bytes = 0;
    unsafe {
        GetTokenInformation(
            token.as_raw_handle(),
            TokenUser,
            std::ptr::null_mut(),
            0,
            &mut bytes,
        );
    }
    if bytes == 0 || bytes > 65536 {
        return Err(CleanupError::UnsafeItem);
    }
    let mut data = vec![0usize; (bytes as usize).div_ceil(std::mem::size_of::<usize>())];
    if unsafe {
        GetTokenInformation(
            token.as_raw_handle(),
            TokenUser,
            data.as_mut_ptr().cast(),
            bytes,
            &mut bytes,
        )
    } == 0
    {
        return Err(io::Error::last_os_error().into());
    }
    Ok(data)
}
fn user_sid(data: &[usize]) -> PSID {
    unsafe { (*(data.as_ptr().cast::<TOKEN_USER>())).User.Sid }
}
fn trusted(sid: PSID, current: PSID) -> bool {
    unsafe {
        EqualSid(sid, current) != 0
            || IsWellKnownSid(sid, WinLocalSystemSid) != 0
            || IsWellKnownSid(sid, WinBuiltinAdministratorsSid) != 0
    }
}
fn secure(file: &File, private: bool) -> Result<(), CleanupError> {
    let mut owner = std::ptr::null_mut();
    let mut acl = std::ptr::null_mut();
    let mut descriptor = std::ptr::null_mut();
    let code = unsafe {
        GetSecurityInfo(
            file.as_raw_handle(),
            SE_FILE_OBJECT,
            OWNER_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION,
            &mut owner,
            std::ptr::null_mut(),
            &mut acl,
            std::ptr::null_mut(),
            &mut descriptor,
        )
    };
    if code != 0 {
        return Err(io::Error::from_raw_os_error(code as i32).into());
    }
    let _descriptor = Local(descriptor);
    let token = user()?;
    let current = user_sid(&token);
    if owner.is_null() || acl.is_null() || !trusted(owner, current) {
        eprintln!("security_check_failed: missing_or_untrusted_owner_or_acl");
        return Err(CleanupError::UnsafeItem);
    }
    for i in 0..unsafe { (*acl).AceCount } {
        let mut ace = std::ptr::null_mut();
        if unsafe { GetAce(acl, u32::from(i), &mut ace) } == 0 {
            return Err(CleanupError::UnsafeItem);
        }
        let header = unsafe { &*ace.cast::<ACE_HEADER>() };
        if u32::from(header.AceFlags) & INHERIT_ONLY_ACE != 0 {
            continue;
        }
        match header.AceType {
            0 => {
                let allowed = unsafe { &*ace.cast::<ACCESS_ALLOWED_ACE>() };
                let sid = std::ptr::addr_of!(allowed.SidStart).cast_mut().cast();
                let writes = 0x40000000
                    | 0x10000000
                    | 0x00010000
                    | 0x00040000
                    | 0x00080000
                    | 0x2
                    | 0x4
                    | 0x10
                    | 0x40
                    | 0x100;
                // OW applies only to the already-validated object owner. Python
                // 3.13+ uses it for mkdir(mode=0700); it never grants another user.
                // Keep ownership validation above strict; only this ACE is recognized.
                let owner_rights = unsafe { IsWellKnownSid(sid, WinCreatorOwnerRightsSid) } != 0;
                if !trusted(sid, current)
                    && !owner_rights
                    && ((private && allowed.Mask != 0) || allowed.Mask & writes != 0)
                {
                    eprintln!(
                        "security_check_failed: untrusted_allow_mask={:#x}, private={private}",
                        allowed.Mask
                    );
                    return Err(CleanupError::UnsafeItem);
                }
            }
            1 => {}
            _ => {
                eprintln!(
                    "security_check_failed: unsupported_ace_type={}",
                    header.AceType
                );
                return Err(CleanupError::UnsafeItem);
            }
        }
    }
    Ok(())
}
pub(super) fn make_private(file: &File) -> Result<(), CleanupError> {
    set_descriptor(file, &private_descriptor()?)
}
fn private_descriptor() -> Result<Local, CleanupError> {
    let token = user()?;
    let mut string = std::ptr::null_mut();
    if unsafe { ConvertSidToStringSidW(user_sid(&token), &mut string) } == 0 {
        return Err(io::Error::last_os_error().into());
    }
    let _string = Local(string.cast());
    let mut length = 0;
    while unsafe { *string.add(length) } != 0 && length < 256 {
        length += 1
    }
    if length == 256 {
        return Err(CleanupError::UnsafeItem);
    }
    let sid = String::from_utf16(unsafe { std::slice::from_raw_parts(string, length) })
        .map_err(|_| CleanupError::UnsafeItem)?;
    let sddl = format!("O:{sid}D:P(A;;FA;;;{sid})(A;;FA;;;SY)(A;;FA;;;BA)")
        .encode_utf16()
        .chain(Some(0))
        .collect::<Vec<_>>();
    let mut descriptor = std::ptr::null_mut();
    if unsafe {
        ConvertStringSecurityDescriptorToSecurityDescriptorW(
            sddl.as_ptr(),
            1,
            &mut descriptor,
            std::ptr::null_mut(),
        )
    } == 0
    {
        return Err(io::Error::last_os_error().into());
    }
    Ok(Local(descriptor))
}
fn security_text(file: &File) -> Result<String, CleanupError> {
    let mut descriptor = std::ptr::null_mut();
    let code = unsafe {
        GetSecurityInfo(
            file.as_raw_handle(),
            SE_FILE_OBJECT,
            DACL_SECURITY_INFORMATION,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut descriptor,
        )
    };
    if code != 0 {
        return Err(io::Error::from_raw_os_error(code as i32).into());
    }
    let _descriptor = Local(descriptor);
    let mut text = std::ptr::null_mut();
    let mut length = 0;
    if unsafe {
        ConvertSecurityDescriptorToStringSecurityDescriptorW(
            descriptor,
            1,
            DACL_SECURITY_INFORMATION,
            &mut text,
            &mut length,
        )
    } == 0
    {
        return Err(io::Error::last_os_error().into());
    }
    let _text = Local(text.cast());
    if length == 0 || length > 8192 {
        return Err(CleanupError::UnsafeItem);
    }
    // Windows reports buffer capacity, not string length. Stop at the first NUL;
    // reading/serializing the unused tail can include extra NULs or uninitialized data.
    // https://learn.microsoft.com/windows/win32/api/sddl/nf-sddl-convertsecuritydescriptortostringsecuritydescriptorw
    let used = (0..length as usize)
        .find(|&index| unsafe { *text.add(index) } == 0)
        .ok_or(CleanupError::UnsafeItem)?;
    let units = unsafe { std::slice::from_raw_parts(text, used) };
    String::from_utf16(units).map_err(|_| CleanupError::UnsafeItem)
}
fn set_descriptor(file: &File, descriptor: &Local) -> Result<(), CleanupError> {
    let mut present = 0;
    let mut defaulted = 0;
    let mut acl = std::ptr::null_mut();
    let mut control = 0;
    let mut revision = 0;
    if unsafe { GetSecurityDescriptorDacl(descriptor.0, &mut present, &mut acl, &mut defaulted) }
        == 0
        || present == 0
        || acl.is_null()
        || unsafe { GetSecurityDescriptorControl(descriptor.0, &mut control, &mut revision) } == 0
    {
        eprintln!(
            "security_descriptor_failed: present={present}, null_acl={}",
            acl.is_null()
        );
        return Err(CleanupError::UnsafeItem);
    }
    let flags = DACL_SECURITY_INFORMATION
        | if control & 0x1000 != 0 {
            PROTECTED_DACL_SECURITY_INFORMATION
        } else {
            UNPROTECTED_DACL_SECURITY_INFORMATION
        };
    let code = unsafe {
        SetSecurityInfo(
            file.as_raw_handle(),
            SE_FILE_OBJECT,
            flags,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            acl,
            std::ptr::null(),
        )
    };
    if code != 0 {
        return Err(io::Error::from_raw_os_error(code as i32).into());
    }
    Ok(())
}
fn restore_security(file: &File, text: &str) -> Result<(), CleanupError> {
    if text.len() > 16384 || text.contains('\0') {
        return Err(CleanupError::UnsafeItem);
    }
    let text = text.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
    let mut descriptor = std::ptr::null_mut();
    if unsafe {
        ConvertStringSecurityDescriptorToSecurityDescriptorW(
            text.as_ptr(),
            1,
            &mut descriptor,
            std::ptr::null_mut(),
        )
    } == 0
    {
        return Err(io::Error::last_os_error().into());
    }
    let descriptor = Local(descriptor);
    set_descriptor(file, &descriptor)?;
    secure(file, false)
}
fn open_at(
    directory: &File,
    name: &str,
    folder: bool,
    create: bool,
    write: bool,
    exclusive: bool,
) -> Result<File, CleanupError> {
    if name.contains(['\\', '/', '\0', ':']) || name.is_empty() {
        return Err(CleanupError::InvalidName);
    }
    let mut name = name.encode_utf16().collect::<Vec<_>>();
    let length = u16::try_from(name.len() * 2).map_err(|_| CleanupError::InvalidName)?;
    let mut unicode = UNICODE_STRING {
        Length: length,
        MaximumLength: length,
        Buffer: name.as_mut_ptr(),
    };
    let descriptor = if create {
        Some(private_descriptor()?)
    } else {
        None
    };
    let attributes = OBJECT_ATTRIBUTES {
        Length: std::mem::size_of::<OBJECT_ATTRIBUTES>() as u32,
        RootDirectory: directory.as_raw_handle(),
        ObjectName: &mut unicode,
        Attributes: 0x40,
        SecurityDescriptor: descriptor.as_ref().map_or(std::ptr::null(), |d| d.0.cast()),
        SecurityQualityOfService: std::ptr::null_mut(),
    };
    let mut handle = std::ptr::null_mut();
    let mut status: IO_STATUS_BLOCK = unsafe { std::mem::zeroed() };
    let access = FILE_READ_DATA
        | FILE_READ_ATTRIBUTES
        | 0x00020000
        | 0x00100000
        | if write {
            FILE_WRITE_DATA | FILE_APPEND_DATA | 0x00010000 | 0x00040000
        } else {
            0
        };
    nt(unsafe {
        NtCreateFile(
            &mut handle,
            access,
            &attributes,
            &mut status,
            std::ptr::null(),
            FILE_ATTRIBUTE_NORMAL,
            if exclusive {
                0
            } else if folder {
                FILE_SHARE_READ | FILE_SHARE_WRITE
            } else {
                FILE_SHARE_READ
            },
            if create {
                if folder
                    || name.as_slice()
                        == ".operation-lock"
                            .encode_utf16()
                            .collect::<Vec<_>>()
                            .as_slice()
                {
                    FILE_OPEN_IF
                } else {
                    FILE_CREATE
                }
            } else {
                FILE_OPEN
            },
            FILE_OPEN_REPARSE_POINT
                | FILE_SYNCHRONOUS_IO_NONALERT
                | if folder {
                    FILE_DIRECTORY_FILE
                } else {
                    FILE_NON_DIRECTORY_FILE
                },
            std::ptr::null(),
            0,
        )
    })?;
    let file = unsafe { File::from_raw_handle(handle) };
    check_kind(&file, folder)?;
    secure(&file, create || name.first() == Some(&('.' as u16)))?;
    Ok(file)
}
fn information(file: &File) -> Result<BY_HANDLE_FILE_INFORMATION, CleanupError> {
    let mut info = unsafe { std::mem::zeroed() };
    if unsafe { GetFileInformationByHandle(file.as_raw_handle(), &mut info) } == 0 {
        return Err(io::Error::last_os_error().into());
    }
    Ok(info)
}
fn check_kind(file: &File, directory: bool) -> Result<(), CleanupError> {
    let info = information(file)?;
    if info.dwFileAttributes & FILE_ATTRIBUTE_REPARSE_POINT != 0
        || (info.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY != 0) != directory
        || (!directory && info.nNumberOfLinks != 1)
    {
        return Err(CleanupError::UnsafeItem);
    }
    Ok(())
}
fn preview(file: &File, name: &str) -> Result<FilePreview, CleanupError> {
    check_kind(file, false)?;
    secure(file, false)?;
    let info = information(file)?;
    let meta = file.metadata()?;
    Ok(FilePreview {
        file_name: name.into(),
        size_bytes: meta.len(),
        last_modified_nanos: modified_nanos(&meta)?,
        identity: Some([
            u64::from(info.dwVolumeSerialNumber),
            (u64::from(info.nFileIndexHigh) << 32) | u64::from(info.nFileIndexLow),
            (u64::from(info.ftCreationTime.dwHighDateTime) << 32)
                | u64::from(info.ftCreationTime.dwLowDateTime),
        ]),
    })
}
fn names(directory: &File) -> Result<Vec<String>, CleanupError> {
    let mut path = vec![0u16; 32768];
    let length = unsafe {
        GetFinalPathNameByHandleW(
            directory.as_raw_handle(),
            path.as_mut_ptr(),
            path.len() as u32,
            0,
        )
    } as usize;
    if length == 0 || length >= path.len() {
        return Err(io::Error::last_os_error().into());
    }
    let path =
        PathBuf::from(String::from_utf16(&path[..length]).map_err(|_| CleanupError::InvalidName)?);
    Ok(std::fs::read_dir(path)?
        .take(1000)
        .filter_map(|item| item.ok()?.file_name().into_string().ok())
        .collect())
}
fn rename(file: &File, to: &File, name: &str) -> Result<(), CleanupError> {
    let name = name.encode_utf16().collect::<Vec<_>>();
    let offset = std::mem::offset_of!(FILE_RENAME_INFORMATION, FileName);
    let length = offset + name.len() * 2;
    let mut buffer = vec![0usize; length.div_ceil(std::mem::size_of::<usize>())];
    let info = buffer.as_mut_ptr().cast::<FILE_RENAME_INFORMATION>();
    unsafe {
        (*info).Anonymous.ReplaceIfExists = false;
        (*info).RootDirectory = to.as_raw_handle();
        (*info).FileNameLength = (name.len() * 2) as u32;
        std::ptr::copy_nonoverlapping(
            name.as_ptr(),
            std::ptr::addr_of_mut!((*info).FileName).cast::<u16>(),
            name.len(),
        );
    }
    let mut status = unsafe { std::mem::zeroed() };
    nt(unsafe {
        NtSetInformationFile(
            file.as_raw_handle(),
            &mut status,
            info.cast(),
            length as u32,
            FileRenameInformation,
        )
    })
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Journal {
    operation_id: Uuid,
    preview: FilePreview,
    quarantined_file: String,
    original_security: String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Marker {
    restore_id: Uuid,
}
fn journal(directory: &File, id: Uuid) -> Result<(Journal, Option<Uuid>), CleanupError> {
    if id.is_nil() {
        return Err(CleanupError::InvalidName);
    }
    let mut file = open_at(
        directory,
        &format!("{id}.journal"),
        false,
        false,
        false,
        false,
    )?;
    secure(&file, true)?;
    let mut bytes = Vec::new();
    (&mut file).take(8193).read_to_end(&mut bytes)?;
    if bytes.len() > 8192 {
        return Err(CleanupError::UnsafeItem);
    }
    let mut lines = bytes.split(|b| *b == b'\n');
    let journal: Journal = serde_json::from_slice(lines.next().ok_or(CleanupError::UnsafeItem)?)
        .map_err(|_| CleanupError::UnsafeItem)?;
    validate_name(&journal.preview.file_name)?;
    if journal.operation_id != id
        || journal.quarantined_file != format!("{id}.item")
        || journal.preview.identity.is_none()
    {
        return Err(CleanupError::UnsafeItem);
    }
    let mut restored = None;
    for line in lines.filter(|s| !s.is_empty()) {
        let marker: Marker = serde_json::from_slice(line).map_err(|_| CleanupError::UnsafeItem)?;
        if marker.restore_id.is_nil() || restored.is_some_and(|r| r != marker.restore_id) {
            return Err(CleanupError::UnsafeItem);
        }
        restored = Some(marker.restore_id);
    }
    Ok((journal, restored))
}
impl PinnedRoot {
    pub fn open(path: &Path) -> Result<Self, CleanupError> {
        use std::os::windows::ffi::OsStrExt;
        let path = path
            .as_os_str()
            .encode_wide()
            .chain(Some(0))
            .collect::<Vec<_>>();
        let handle = unsafe {
            CreateFileW(
                path.as_ptr(),
                FILE_READ_DATA | FILE_ADD_FILE | FILE_READ_ATTRIBUTES | 0x00020000 | 0x00100000,
                FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
                std::ptr::null(),
                OPEN_EXISTING,
                FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT,
                std::ptr::null_mut(),
            )
        };
        if handle == INVALID_HANDLE_VALUE {
            return Err(io::Error::last_os_error().into());
        }
        let directory = unsafe { File::from_raw_handle(handle) };
        check_kind(&directory, true)?;
        secure(&directory, false)?;
        Ok(Self { directory })
    }
    fn trash_directory(&self) -> Result<File, CleanupError> {
        open_at(&self.directory, ".pet-quarantine", true, true, true, false)
    }
    fn lock(&self, directory: &File) -> Result<File, CleanupError> {
        let until = std::time::Instant::now() + std::time::Duration::from_millis(500);
        loop {
            match open_at(directory, ".operation-lock", false, true, true, true) {
                Err(CleanupError::Io(e))
                    if e.raw_os_error() == Some(32) && std::time::Instant::now() < until =>
                {
                    std::thread::sleep(std::time::Duration::from_millis(2))
                }
                result => return result,
            }
        }
    }
    pub fn preview(&self, name: &str) -> Result<FilePreview, CleanupError> {
        validate_name(name)?;
        preview(
            &open_at(&self.directory, name, false, false, false, false)?,
            name,
        )
    }
    pub fn scan(&self) -> Result<Vec<FilePreview>, CleanupError> {
        Ok(names(&self.directory)?
            .into_iter()
            .filter_map(|name| self.preview(&name).ok())
            .collect())
    }
    pub fn quarantine(&self, expected: &FilePreview) -> Result<CleanupReceipt, CleanupError> {
        validate_name(&expected.file_name)?;
        let directory = self.trash_directory()?;
        let _lock = self.lock(&directory)?;
        let file = open_at(
            &self.directory,
            &expected.file_name,
            false,
            false,
            true,
            false,
        )?;
        if preview(&file, &expected.file_name)? != *expected {
            return Err(CleanupError::Changed);
        }
        let id = Uuid::new_v4();
        let name = format!("{id}.item");
        let record = Journal {
            operation_id: id,
            preview: expected.clone(),
            quarantined_file: name.clone(),
            original_security: security_text(&file)?,
        };
        let mut log = open_at(
            &directory,
            &format!("{id}.journal"),
            false,
            true,
            true,
            true,
        )?;
        let encoded = serde_json::to_vec(&record).map_err(|_| CleanupError::UnsafeItem)?;
        if encoded.len() > 7168 {
            return Err(CleanupError::UnsafeItem);
        }
        log.write_all(&encoded)?;
        log.write_all(b"\n")?;
        log.sync_all()?;
        set_descriptor(&file, &private_descriptor()?)?;
        if let Err(error) = rename(&file, &directory, &name) {
            restore_security(&file, &record.original_security).map_err(|e| {
                eprintln!("restore_security_failed: {e}");
                CleanupError::RecoveryRequired(id)
            })?;
            return Err(error);
        }
        Ok(CleanupReceipt {
            operation_id: id,
            quarantined_file: name,
            size_bytes: expected.size_bytes,
        })
    }
    fn matches(&self, directory: &File, name: &str, expected: &FilePreview) -> bool {
        open_at(directory, name, false, false, false, false)
            .and_then(|file| preview(&file, &expected.file_name))
            .is_ok_and(|current| current == *expected)
    }
    pub fn restore(&self, id: Uuid) -> Result<RestoreReceipt, CleanupError> {
        let directory = self.trash_directory()?;
        let _lock = self.lock(&directory)?;
        let (record, marker) = journal(&directory, id)?;
        let restore_id = marker.unwrap_or_else(Uuid::new_v4);
        if marker.is_some()
            && self.matches(&self.directory, &record.preview.file_name, &record.preview)
        {
            let file = open_at(
                &self.directory,
                &record.preview.file_name,
                false,
                false,
                true,
                false,
            )?;
            restore_security(&file, &record.original_security).map_err(|e| {
                eprintln!("restore_security_failed: {e}");
                CleanupError::RecoveryRequired(id)
            })?;
            return Ok(RestoreReceipt {
                operation_id: restore_id,
                quarantine_operation_id: id,
                size_bytes: record.preview.size_bytes,
            });
        }
        let file = open_at(
            &directory,
            &record.quarantined_file,
            false,
            false,
            true,
            false,
        )?;
        if preview(&file, &record.preview.file_name)? != record.preview {
            return Err(CleanupError::Changed);
        }
        if marker.is_none() {
            let mut log = open_at(
                &directory,
                &format!("{id}.journal"),
                false,
                false,
                true,
                true,
            )?;
            use std::io::{Seek, SeekFrom};
            log.seek(SeekFrom::End(0))?;
            log.write_all(
                &serde_json::to_vec(&Marker { restore_id })
                    .map_err(|_| CleanupError::UnsafeItem)?,
            )?;
            log.write_all(b"\n")?;
            log.sync_all()?;
        }
        rename(&file, &self.directory, &record.preview.file_name)?;
        restore_security(&file, &record.original_security).map_err(|e| {
            eprintln!("restore_security_failed: {e}");
            CleanupError::RecoveryRequired(id)
        })?;
        Ok(RestoreReceipt {
            operation_id: restore_id,
            quarantine_operation_id: id,
            size_bytes: record.preview.size_bytes,
        })
    }
    pub fn trash(&self) -> Result<Vec<TrashEntry>, CleanupError> {
        let directory = self.trash_directory()?;
        let _lock = self.lock(&directory)?;
        let mut entries = Vec::new();
        for name in names(&directory)? {
            let Some(id) = name
                .strip_suffix(".journal")
                .and_then(|s| Uuid::parse_str(s).ok())
            else {
                continue;
            };
            let (record, marker) = journal(&directory, id)?;
            let captured = self.matches(&directory, &record.quarantined_file, &record.preview);
            let restored = marker.is_some()
                && !captured
                && self.matches(&self.directory, &record.preview.file_name, &record.preview);
            if captured || restored {
                entries.push(TrashEntry {
                    operation_id: id,
                    file_name: record.preview.file_name,
                    size_bytes: record.preview.size_bytes,
                    restored,
                })
            }
        }
        Ok(entries)
    }
    pub fn recovered_receipts(&self) -> Result<Vec<CleanupReceipt>, CleanupError> {
        Ok(self
            .trash()?
            .into_iter()
            .map(|item| CleanupReceipt {
                operation_id: item.operation_id,
                quarantined_file: format!("{}.item", item.operation_id),
                size_bytes: item.size_bytes,
            })
            .collect())
    }
    pub fn restored_receipts(&self) -> Result<Vec<RestoreReceipt>, CleanupError> {
        let ids = self
            .trash()?
            .into_iter()
            .filter(|item| item.restored)
            .map(|item| item.operation_id)
            .collect::<Vec<_>>();
        ids.into_iter().map(|id| self.restore(id)).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn windows_owner_rights_private_acl_is_allowed_but_world_write_is_rejected() {
        use std::os::windows::fs::OpenOptionsExt;
        let path = std::env::temp_dir().join(format!("pet-owner-rights-{}", Uuid::new_v4()));
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .access_mode(FILE_READ_DATA | FILE_WRITE_DATA | 0x00020000 | 0x00040000)
            .open(&path)
            .unwrap();
        for (sddl, allowed) in [
            ("D:P(A;;FA;;;OW)(A;;FA;;;SY)(A;;FA;;;BA)", true),
            ("D:P(A;;FA;;;OW)(A;;FA;;;SY)(A;;FA;;;BA)(A;;FA;;;WD)", false),
        ] {
            let text = sddl.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
            let mut raw = std::ptr::null_mut();
            assert_ne!(
                unsafe {
                    ConvertStringSecurityDescriptorToSecurityDescriptorW(
                        text.as_ptr(),
                        1,
                        &mut raw,
                        std::ptr::null_mut(),
                    )
                },
                0
            );
            let descriptor = Local(raw);
            set_descriptor(&file, &descriptor).unwrap();
            assert_eq!(secure(&file, false).is_ok(), allowed);
            assert_eq!(secure(&file, true).is_ok(), allowed);
        }
        drop(file);
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn windows_handle_capture_restore_and_conflict_are_idempotent() {
        let path = std::env::temp_dir().join(format!("pet-windows-{}", Uuid::new_v4()));
        std::fs::create_dir(&path).unwrap();
        std::fs::write(path.join("a.txt"), b"original").unwrap();
        let root = PinnedRoot::open(&path).unwrap();
        let item = root.preview("a.txt").unwrap();
        let original_acl =
            security_text(&open_at(&root.directory, "a.txt", false, false, false, false).unwrap())
                .unwrap();
        assert!(
            !original_acl.contains('\0'),
            "SDDL excludes unused capacity and terminator"
        );
        std::fs::write(path.join("a.txt"), b"changed").unwrap();
        assert!(root.quarantine(&item).is_err());
        std::fs::write(path.join("a.txt"), b"original").unwrap();
        let item = root.preview("a.txt").unwrap();
        let receipt = root.quarantine(&item).unwrap();
        assert!(!path.join("a.txt").exists());
        assert_eq!(root.recovered_receipts().unwrap(), vec![receipt.clone()]);
        std::fs::write(path.join("a.txt"), b"replacement").unwrap();
        assert!(root.restore(receipt.operation_id).is_err());
        assert_eq!(std::fs::read(path.join("a.txt")).unwrap(), b"replacement");
        std::fs::remove_file(path.join("a.txt")).unwrap();
        let restored = root.restore(receipt.operation_id).unwrap();
        assert_eq!(root.restore(receipt.operation_id).unwrap(), restored);
        assert_eq!(root.restored_receipts().unwrap(), vec![restored]);
        assert_eq!(std::fs::read(path.join("a.txt")).unwrap(), b"original");
        let restored_acl =
            security_text(&open_at(&root.directory, "a.txt", false, false, false, false).unwrap())
                .unwrap();
        // SetSecurityInfo records SE_DACL_AUTO_INHERITED after propagation. This
        // bookkeeping bit may change; every ACE, order and protection flag must match.
        let permissions = |s: &str| {
            let (control, aces) = s.split_once('(').expect("nonempty fixture DACL");
            format!("{}({aces}", control.replace("AI", ""))
        };
        assert_eq!(permissions(&restored_acl), permissions(&original_acl));
        drop(root);
        std::fs::remove_dir_all(path).unwrap();
    }
    #[test]
    fn windows_rejects_links_and_writers_and_pins_renamed_root() {
        let path = std::env::temp_dir().join(format!("pet-windows-reject-{}", Uuid::new_v4()));
        std::fs::create_dir(&path).unwrap();
        std::fs::write(path.join("a.txt"), b"original").unwrap();
        let root = PinnedRoot::open(&path).unwrap();
        let item = root.preview("a.txt").unwrap();
        std::fs::hard_link(path.join("a.txt"), path.join("hard.txt")).unwrap();
        assert!(root.preview("a.txt").is_err());
        std::fs::remove_file(path.join("hard.txt")).unwrap();
        let writer = std::fs::OpenOptions::new()
            .write(true)
            .open(path.join("a.txt"))
            .unwrap();
        assert!(root.quarantine(&item).is_err());
        drop(writer);
        let moved = path.with_extension("moved");
        std::fs::rename(&path, &moved).unwrap();
        std::fs::create_dir(&path).unwrap();
        std::fs::write(path.join("a.txt"), b"unrelated").unwrap();
        let receipt = root.quarantine(&item).unwrap();
        assert_eq!(std::fs::read(path.join("a.txt")).unwrap(), b"unrelated");
        assert_eq!(
            std::fs::read(moved.join(".pet-quarantine").join(receipt.quarantined_file)).unwrap(),
            b"original"
        );
        assert!(root.preview("../a.txt").is_err());
        assert!(root.preview("a.txt:stream").is_err());
        drop(root);
        std::fs::remove_dir_all(path).unwrap();
        std::fs::remove_dir_all(moved).unwrap();
    }
}
