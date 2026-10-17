# Executed only by the learner after reviewing the Day16 code.
import hashlib, os, secrets, sys, tempfile
from pathlib import Path
root=Path(sys.argv[1]).resolve()
if root != Path('/etc/rust-ai-chat'):
    raise SystemExit('本教學只接受 /etc/rust-ai-chat，避免將執行秘密寫入程式目錄')
env=root/'.env'
old=env.read_text()
if any(line.startswith('ANALYTICS_ADMIN_KEY_SHA256=') for line in old.splitlines()):
    raise SystemExit('已有管理員設定，請勿以初始化腳本輪替')
key=secrets.token_hex(32)
fd=os.open(root/'analytics-admin.bootstrap',os.O_WRONLY|os.O_CREAT|os.O_EXCL,0o600)
with os.fdopen(fd,'w') as f: f.write(key+'\n')
fd,name=tempfile.mkstemp(prefix='.analytics-admin-',dir=root)
try:
    with os.fdopen(fd,'w') as f:
        f.write(old.rstrip()+'\nANALYTICS_ADMIN_KEY_SHA256='+hashlib.sha256(key.encode()).hexdigest()+'\n')
    os.chmod(name,0o600)
    os.replace(name,env)
finally:
    if os.path.exists(name): os.unlink(name)
print('已寫入管理員設定。登入碼僅在 /etc/rust-ai-chat/analytics-admin.bootstrap；請自行讀取後安全保存並移除交付檔。')
