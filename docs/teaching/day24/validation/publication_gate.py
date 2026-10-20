"""Complete article readback and idempotent state transition; no network or blind overwrite."""
import copy,hashlib,json,re,sys
from pathlib import Path

def sha(text): return hashlib.sha256(text.encode()).hexdigest()
def blocks(text): return [(m[0],m[1].removesuffix('\n')) for m in re.findall(r'```([^\n]*)\n([\s\S]*?)```',text.replace('\r\n','\n'))]
def normal(text):
    parts=re.split(r'(```[\s\S]*?```)',text)
    for i in range(0,len(parts),2):
        parts[i]=re.sub(r'<code>([\s\S]*?)</code>',r'`\1`',parts[i])
        parts[i]=re.sub(r'\\([\\*~`$\[\]<>{}|^_])',r'\1',parts[i])
    return re.sub(r'\s+',' ',''.join(parts)).strip()
def verify(source,readback,source_sha,index_text,page_id):
    assert len(source)>3000, 'short test payload cannot be published as full lesson'
    assert re.findall(r'^## ([1-4])\.',source,re.M)==['1','2','3','4'], 'four sections required'
    assert all(x in source for x in ['Red','Green','Refactor',source_sha]), 'missing teaching or provenance'
    assert source.count('```')%2==0 and len(blocks(source))>=2, 'unclosed or missing code blocks'
    assert normal(source)==normal(readback), 'full text differs'
    assert blocks(source)==blocks(readback), 'literal code differs'
    assert page_id.replace('-','') in index_text.replace('-',''), 'not indexed'
    return {'full_text_verified':True,'code_blocks_exact':True,'code_block_count':len(blocks(source)),
            'source_sha':source_sha,'body_sha256':sha(source),'readback_sha256':sha(readback),'page_id':page_id,'index_verified':True}
def transition(state,project,proof):
    assert proof.get('full_text_verified') and proof.get('code_blocks_exact') and proof.get('index_verified'), 'unverified publication'
    before=copy.deepcopy(state);n=state['next_day'][project]
    if n==25:
        assert state.get('day24_'+project+'_delivery',{}).get('body_sha256')==proof['body_sha256'], 'progress evidence conflict'
        return before,False
    assert n==24, 'unexpected next_day'
    before['next_day'][project]=25;before['day24_'+project+'_delivery']=copy.deepcopy(proof)
    return before,True
if __name__=='__main__':
    print('Import verify/transition; writes require caller branch SHA and blob compare plus normal fast-forward push.')

def revise(state,project,expected_body_sha256,proof):
    assert proof.get('full_text_verified') and proof.get('code_blocks_exact') and proof.get('index_verified'), 'unverified revision'
    assert state['next_day'][project]==25, 'revision cannot advance or skip a day'
    current=state.get('day24_'+project+'_delivery',{})
    if current.get('body_sha256')==proof['body_sha256']: return copy.deepcopy(state),False
    assert current.get('body_sha256')==expected_body_sha256, 'revision evidence conflict; reread'
    result=copy.deepcopy(state);result['day24_'+project+'_delivery']=copy.deepcopy(proof)
    return result,True
