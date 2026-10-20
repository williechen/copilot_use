import importlib.util,copy,unittest
from pathlib import Path
s=importlib.util.spec_from_file_location('gate',str(Path(__file__).with_name('publication_gate.py')));g=importlib.util.module_from_spec(s);s.loader.exec_module(g)
class GateTests(unittest.TestCase):
 def setUp(self):
  self.src='\n'.join('## '+str(n)+'. Section\nRed Green Refactor '+('正文 '*300)+'\n```rust\nlet x = 1;\n```' for n in range(1,5))+'\n'+'c965681eeb2b4791dba8d157624424b7f274b2fd'
  self.args=('c965681eeb2b4791dba8d157624424b7f274b2fd','page=abc','abc');self.state={'next_day':dict(chat=24,pet=24,analytics=24),'makeup_queue':[{'status':'completed'}],'last_full_comparison':'preserve'}
 def proof(self): return g.verify(self.src,self.src,*self.args)
 def test_full_article(self): self.assertEqual(self.proof()['code_block_count'],4)
 def test_truncated(self):
  with self.assertRaises(AssertionError): g.verify(self.src,self.src[:150],*self.args)
 def test_code_whitespace_not_ignored(self):
  with self.assertRaises(AssertionError): g.verify(self.src,self.src.replace('let x = 1;','let  x = 1;'),*self.args)
 def test_wrong_index(self):
  with self.assertRaises(AssertionError): g.verify(self.src,self.src,self.args[0],'missing','abc')
 def test_short_payload(self):
  with self.assertRaises(AssertionError): g.verify('test','test',*self.args)
 def test_missing_section(self):
  with self.assertRaises(AssertionError): g.verify(self.src.replace('## 3.','### 3.'),self.src,*self.args)
 def test_conditional_transition_preserves_fields(self):
  new,changed=g.transition(self.state,'chat',self.proof());self.assertTrue(changed);self.assertEqual(new['next_day'],dict(chat=25,pet=24,analytics=24));self.assertEqual(new['makeup_queue'],self.state['makeup_queue']);self.assertEqual(new['last_full_comparison'],'preserve');self.assertEqual(self.state['next_day']['chat'],24)
 def test_idempotent_retry(self):
  new,_=g.transition(self.state,'chat',self.proof());again,changed=g.transition(new,'chat',self.proof());self.assertFalse(changed);self.assertEqual(new,again)
 def test_unexpected_day(self):
  self.state['next_day']['chat']=26
  with self.assertRaises(AssertionError): g.transition(self.state,'chat',self.proof())
 def test_conflicting_retry(self):
  new,_=g.transition(self.state,'chat',self.proof());proof=self.proof();proof['body_sha256']='different'
  with self.assertRaises(AssertionError): g.transition(new,'chat',proof)
 def test_failed_readback(self):
  with self.assertRaises(AssertionError): g.transition(self.state,'chat',{})
unittest.main()
