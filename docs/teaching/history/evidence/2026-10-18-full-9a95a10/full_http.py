import http.client, json, re, sys, time, uuid

def call(port, method, path, body=None, headers=None):
    c = http.client.HTTPConnection('127.0.0.1', port, timeout=10)
    c.request(method, path, body, headers or {})
    r = c.getresponse()
    result = (r.status, dict(r.getheaders()), r.read().decode())
    c.close()
    return result

def check(name, result, expected):
    assert result[0] == expected, (name, result)
    print(f'PASS {name}: HTTP {result[0]}')
    return result

mode = sys.argv[1]
if mode == 'chat':
    for name in ['Rust 技術交流', '生活閒聊', 'RUST 中文長房名' * 4]:
        check('create fixture room', call(3000, 'POST', '/api/rooms', json.dumps({'name': name}), {'Content-Type': 'application/json'}), 201)
    r = check('list rooms', call(3000, 'GET', '/api/rooms'), 200)
    rows = json.loads(r[2])
    assert len(rows) == 3
    print('PASS fixture names and IDs:', json.dumps(rows, ensure_ascii=False))
    check('SSR homepage', call(3000, 'GET', '/'), 200)
    check('SSR room route', call(3000, 'GET', '/room/' + rows[0]['id']), 200)
else:
    url = '/api/v1/events/web_chat/summary'
    check('anonymous dashboard redirect', call(3100, 'GET', '/dashboard'), 303)
    check('anonymous query denied', call(3100, 'GET', url), 401)
    check('service token cannot log into human query', call(3100, 'GET', url, headers={'Authorization': 'Bearer fixture-chat'}), 401)
    login_headers = {'Origin': 'http://127.0.0.1:3100', 'Content-Type': 'application/x-www-form-urlencoded'}
    check('missing origin denied', call(3100, 'POST', '/login', 'key=' + 'a'*64, {'Content-Type': 'application/x-www-form-urlencoded'}), 403)
    check('wrong login denied', call(3100, 'POST', '/login', 'key=wrong', login_headers), 401)
    r = check('fixture login', call(3100, 'POST', '/login', 'key=' + 'a'*64, login_headers), 303)
    cookie = r[1]['set-cookie'].split(';')[0]
    assert 'HttpOnly' in r[1]['set-cookie'] and 'SameSite=Strict' in r[1]['set-cookie'] and 'Max-Age=1800' in r[1]['set-cookie']
    event = json.dumps({'schema_version': 1, 'event_id': str(uuid.uuid4()), 'source_app': 'web_chat', 'event_type': 'fixture.day18'})
    check('missing ingestion identity denied', call(3100, 'POST', '/api/v1/events', event, {'Content-Type': 'application/json'}), 401)
    check('cross-source ingestion denied', call(3100, 'POST', '/api/v1/events', event, {'Content-Type': 'application/json','Authorization': 'Bearer fixture-pet'}), 401)
    check('ingest committed event', call(3100, 'POST', '/api/v1/events', event, {'Content-Type': 'application/json','Authorization': 'Bearer fixture-chat'}), 201)
    check('idempotent ingest', call(3100, 'POST', '/api/v1/events', event, {'Content-Type': 'application/json','Authorization': 'Bearer fixture-chat'}), 200)
    h = {'Cookie': cookie}
    r = check('authenticated PostgreSQL summary', call(3100, 'GET', url, headers=h), 200)
    print('Query body:', r[2])
    time.sleep(1.1)
    r = check('active dashboard', call(3100, 'GET', '/dashboard', headers=h), 200)
    renewed = r[1].get('set-cookie','')
    print('COOKIE_RENEWED:', 'Max-Age=1800' in renewed)
    if len(sys.argv) > 2 and sys.argv[2] == 'require-renewal':
        assert 'Max-Age=1800' in renewed, 'valid activity did not renew browser cookie'
        assert renewed.split(';')[0] == cookie
    csrf = re.search('name="csrf" value="([^"]+)"', r[2]).group(1)
    check('invalid source denied', call(3100, 'GET', '/dashboard?source=invalid', headers=h), 400)
    check('wrong CSRF logout denied', call(3100, 'POST', '/logout', 'csrf=wrong', {**h, **login_headers}), 403)
    r = check('logout revokes session', call(3100, 'POST', '/logout', 'csrf='+csrf, {**h, **login_headers}), 303)
    assert 'Max-Age=0' in r[1]['set-cookie']
    check('revoked cookie denied', call(3100, 'GET', url, headers=h), 401)
