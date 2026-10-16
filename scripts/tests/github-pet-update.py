#!/usr/bin/env python3
import importlib.util,io,json,pathlib,tarfile,tempfile,unittest,hashlib,sqlite3,subprocess
from unittest.mock import patch
ROOT=pathlib.Path(__file__).resolve().parents[2]
spec=importlib.util.spec_from_file_location('updater',ROOT/'scripts/github-pet-update.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
class Updates(unittest.TestCase):
 def release(self,tag='v0.3.0'):
  return {'tag_name':tag,'draft':False,'prerelease':False,'assets':[{'name':m.MANIFEST,'size':200,'browser_download_url':f'https://github.com/{m.REPOSITORY}/releases/download/{tag}/{m.MANIFEST}'}]}
 def test_stable_only_and_no_downgrades(self):
  self.assertIsNone(m.candidate(self.release('v0.1.0'),'0.2.1'));self.assertIsNone(m.candidate(self.release('v0.2.1'),'0.2.1'))
  self.assertEqual(m.candidate(self.release(),'0.2.1')[0],'v0.3.0')
  for field in ['draft','prerelease']:
   r=self.release();r[field]=True
   with self.assertRaises(ValueError):m.candidate(r,'0.2.1')
  for bad in ['v0.2.2-beta','../../x','0.02.3','v99999999999999.0.0']:
   with self.assertRaises(ValueError):m.version(bad)
 def test_release_asset_cannot_redirect_to_another_repository(self):
  r=self.release();r['assets'][0]['browser_download_url']='https://github.com/attacker/other/releases/download/v0.3.0/x'
  with self.assertRaises(ValueError):m.candidate(r,'0.2.1')
 def test_archive_rejects_traversal_links_and_duplicate_names(self):
  for names in [['../outside'],['copilot-desktop-pet','copilot-desktop-pet'],['link']]:
   with tempfile.TemporaryDirectory() as d:
    p=pathlib.Path(d);archive=p/'update.tar.gz';stage=p/'stage';stage.mkdir()
    with tarfile.open(archive,'w:gz') as tar:
     for name in names:
      member=tarfile.TarInfo(name)
      if name=='link':member.type=tarfile.SYMTYPE;member.linkname='/tmp/outside';tar.addfile(member)
      else:member.size=1;tar.addfile(member,io.BytesIO(b'x'))
    with self.assertRaises(ValueError):m.unpack(archive,stage)
    self.assertFalse((p/'outside').exists())
 def test_complete_archive_extracts_only_expected_files(self):
  with tempfile.TemporaryDirectory() as d:
   p=pathlib.Path(d);archive=p/'update.tar.gz';stage=p/'stage';stage.mkdir()
   with tarfile.open(archive,'w:gz') as tar:
    for name in m.FILES:
     member=tarfile.TarInfo(name);member.size=1;tar.addfile(member,io.BytesIO(b'x'))
   m.unpack(archive,stage);self.assertEqual({x.name for x in stage.iterdir()},m.FILES)
 def test_no_download_or_verifier_when_latest_is_older(self):
  with tempfile.TemporaryDirectory() as d:
   root=pathlib.Path(d);app=root/'app';release=app/'releases/v0.2.1';release.mkdir(parents=True);(app/'current').symlink_to(release)
   (release/'installation.json').write_text(json.dumps({'version':'0.2.1'}));calls=[]
   def fetch(url,limit):calls.append(url);return json.dumps(self.release('v0.1.0')).encode()
   result=m.check(app,root/'config',root/'cache',root/'pet.sqlite',root/'entry',fetch)
   self.assertEqual(result['status'],'up_to_date');self.assertEqual(calls,[m.API]);self.assertEqual((app/'current').resolve(),release)
 def test_activation_preserves_existing_entry_and_database(self):
  with tempfile.TemporaryDirectory() as d:
   root=pathlib.Path(d);app=root/'app';old=app/'releases/v0.2.0';old.mkdir(parents=True);(app/'current').symlink_to(old)
   (old/'installation.json').write_text(json.dumps({'version':'0.2.0'}));cache=root/'cache';cache.mkdir();stage=app/'stage';stage.mkdir()
   for name in m.FILES:(stage/name).write_bytes(b'fixture')
   metadata={'version':'0.2.1','source_sha':'a'*40,'binary_sha256':hashlib.sha256(b'fixture').hexdigest()}
   (stage/'build.json').write_text(json.dumps(metadata));entry=root/'entry';entry.write_text('unchanged launcher')
   db=root/'pet.sqlite'
   with sqlite3.connect(db) as c:c.execute('create table pets(id text)');c.execute("insert into pets values('same-pet')")
   with patch.object(m,'info',return_value={**metadata,'source_dirty':False,'release':True}),patch.object(m,'installed_processes',return_value=[]):result=m.activate(app,stage,metadata,cache,db,entry)
   self.assertEqual(result['status'],'updated');self.assertFalse(result['restarted']);self.assertEqual(entry.read_text(),'unchanged launcher')
   self.assertEqual((app/'current').resolve(),app/'releases/v0.2.1');self.assertTrue(old.is_dir())
   with sqlite3.connect(db) as c:self.assertEqual(c.execute('select id from pets').fetchone()[0],'same-pet')
   self.assertTrue(list(cache.glob('pre-update-*.sqlite')))
 def test_invalid_signature_never_unpacks_or_changes_current(self):
  with tempfile.TemporaryDirectory() as d:
   root=pathlib.Path(d);app=root/'app';old=app/'releases/v0.2.1';old.mkdir(parents=True);(app/'current').symlink_to(old);(old/'installation.json').write_text(json.dumps({'version':'0.2.1'}))
   config=root/'config';config.mkdir();(config/'update-public-key.pub').write_text('pinned fixture key')
   release=self.release();name='copilot-pet_0.3.0_linux-x86_64.tar.gz';release['assets'].append({'name':name,'size':1,'browser_download_url':f'https://github.com/{m.REPOSITORY}/releases/download/v0.3.0/{name}'})
   manifest={'version':'0.3.0','source_sha':'a'*40,'signature':'invalid'}
   def fetch(url,limit):return json.dumps(release).encode() if url==m.API else json.dumps(manifest).encode() if url.endswith(m.MANIFEST) else b'x'
   with patch.object(m.subprocess,'run',side_effect=subprocess.CalledProcessError(1,'verify')),patch.object(m,'unpack') as unpack:
    with self.assertRaises(subprocess.CalledProcessError):m.check(app,config,root/'cache',root/'db',root/'entry',fetch)
    unpack.assert_not_called()
   self.assertEqual((app/'current').resolve(),old)
if __name__=='__main__':unittest.main()
