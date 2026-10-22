from http.server import ThreadingHTTPServer, BaseHTTPRequestHandler
from pathlib import Path
import http.client, json, datetime, os
ROOT = Path(os.environ['DAY18_ACCEPTANCE_DIR']).resolve()
class Proxy(BaseHTTPRequestHandler):
    def do_GET(self):
        fail = self.path.split('?',1)[0] == '/api/rooms' and (ROOT/'work/rooms-fail').exists()
        if fail:
            status, headers, body = 503, {}, b'fixture unavailable'
        else:
            conn = http.client.HTTPConnection('127.0.0.1',3000,timeout=10)
            conn.request('GET',self.path)
            r = conn.getresponse()
            status, headers, body = r.status, dict(r.getheaders()), r.read()
            conn.close()
        with (ROOT/'outputs/full-2c7ace2/network.jsonl').open('a') as f:
            f.write(json.dumps({'time':datetime.datetime.now().isoformat(),'method':'GET','path':self.path,'status':status,'injected':fail})+'\n')
        self.send_response(status)
        for k,v in headers.items():
            if k.lower() not in ['content-length','transfer-encoding','connection','date','server']: self.send_header(k,v)
        self.send_header('Content-Length',str(len(body)))
        self.end_headers()
        self.wfile.write(body)
    def log_message(self,*args): pass
ThreadingHTTPServer(('127.0.0.1',3002),Proxy).serve_forever()
