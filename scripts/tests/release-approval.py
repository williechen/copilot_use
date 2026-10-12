#!/usr/bin/env python3
"""Ensure incomplete local evidence cannot reach any network request or tag mutation."""
import json,os,pathlib,shutil,subprocess,tempfile,unittest
ROOT=pathlib.Path(__file__).resolve().parents[2]
class ReleaseGate(unittest.TestCase):
    def test_rejects_missing_real_clean_packaged_update_evidence_before_network(self):
        with tempfile.TemporaryDirectory()as scratch:
            root=pathlib.Path(scratch);(root/'scripts').mkdir();(root/'.github').mkdir();(root/'docs/release-evidence').mkdir(parents=True)
            shutil.copyfile(ROOT/'scripts/release-approved.py',root/'scripts/release-approved.py')
            (root/'sitecustomize.py').write_text("import urllib.request\ndef deny(*args,**kwargs):raise AssertionError('NETWORK_TOUCHED')\nurllib.request.OpenerDirector.open=deny\n")
            def git(*args):return subprocess.check_output(['git',*args],cwd=root,stderr=subprocess.DEVNULL,text=True).strip()
            git('init','-q');(root/'source').write_text('fixture');git('add','source');git('-c','user.name=Test','-c','user.email=test@example.invalid','-c','commit.gpgsign=false','commit','-qm','source');sha=git('rev-parse','HEAD')
            (root/'source').write_text('approval');git('add','source');git('-c','user.name=Test','-c','user.email=test@example.invalid','-c','commit.gpgsign=false','commit','-qm','approval')
            evidence={k:'docs/release-evidence/'+k+'.json'for k in ('live','docker','packaged_gui','appimage_update')}
            approval={'approved_sha':sha,'tag':'v0.1.0-p9','evidence':evidence,'notes':'Disposable gate regression'}
            (root/'.github/release-approved.json').write_text(json.dumps(approval))
            cases=[('docker',{'containers':{'chat':{'revision':'0'*40,'image_id':'sha256:'+'1'*64},'analytics':{'revision':sha,'image_id':'sha256:'+'1'*64}}},'Actual Docker image revisions'),('live',{'source_sha':'0'*40},'exact clean source'),('live',{'source_dirty':True},'exact clean source'),('live',{'live_openai':False},'Real configured-model'),('packaged_gui',{'release':False},'Packaged release'),('appimage_update',{'update_install':False},'Actual signed update')]
            for kind,changes,expected in cases:
                with self.subTest(kind=kind,changes=changes):
                    for name,path in evidence.items():
                        report={'source_sha':sha,'source_dirty':False,'status':'passed','success':True,'live_openai':True,'openai_model':'gpt-5.6-luna','release':True,'update_install':{'fixture':True},'containers':{name:{'revision':sha,'image_id':'sha256:'+'1'*64}for name in ('chat','analytics')}}
                        if name==kind:report.update(changes)
                        (root/path).write_text(json.dumps(report))
                    result=subprocess.run(['python3',root/'scripts/release-approved.py'],cwd=root,env=os.environ|{'GH_REPOSITORY':'williechen/copilot_use','GH_TOKEN':'synthetic-unused','PYTHONPATH':str(root)},text=True,capture_output=True)
                    self.assertNotEqual(result.returncode,0);self.assertIn(expected,result.stderr);self.assertNotIn('NETWORK_TOUCHED',result.stderr)
if __name__=='__main__':unittest.main()
