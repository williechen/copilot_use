#!/usr/bin/env python3
"""本機原生 Chat／Analytics；資料與憑證只存於忽略的 .local/runtime。"""
import argparse
import fcntl
import json
import os
from pathlib import Path
import secrets
import socket
import subprocess
import time
import urllib.request

ROOT = Path(__file__).resolve().parents[1]
STATE = ROOT / ".local/runtime"


def run(argv, **kwargs):
    return subprocess.run([str(x) for x in argv], check=True, **kwargs)


def pg_bin():
    return Path(subprocess.check_output(["pg_config", "--bindir"], text=True).strip())


def healthy(port):
    try:
        with urllib.request.urlopen(f"http://127.0.0.1:{port}/healthz", timeout=2) as response:
            return response.status == 200
    except Exception:
        return False


def stop():
    import signal
    for name, binary in (("chat", "day9-chat"), ("analytics", "day9-analytics")):
        pid_file = STATE / f"{name}.pid"
        if not pid_file.exists():
            continue
        pid = int(pid_file.read_text())
        try:
            if not Path(f"/proc/{pid}/exe").exists():
                pid_file.unlink(missing_ok=True)
                continue
            actual = Path(os.readlink(f"/proc/{pid}/exe").removesuffix(" (deleted)"))
            expected = ROOT / "target/release" / binary
            if actual != expected:
                raise RuntimeError(f"PID {pid} 不屬於此服務，拒絕停止")
            os.kill(pid, signal.SIGTERM)
            for _ in range(50):
                if not Path(f"/proc/{pid}/exe").exists():
                    break
                time.sleep(.1)
            else:
                raise RuntimeError(f"{name} 未在期限內停止；保留 PID")
        except ProcessLookupError:
            pass
        finally:
            if not Path(f"/proc/{pid}/exe").exists():
                pid_file.unlink(missing_ok=True)
    if (STATE / "postgres/postmaster.pid").exists():
        run([pg_bin() / "pg_ctl", "-D", STATE / "postgres", "stop", "-m", "fast"])


def start():
    for binary in ("day9-chat", "day9-analytics"):
        if not (ROOT / "target/release" / binary).is_file():
            raise SystemExit("請先執行 bash scripts/build-production.sh")
    if not (ROOT / "target/site/pkg/chat_ui_bg.wasm").exists():
        raise SystemExit("缺少前端 WASM，請先完成原生建置")
    STATE.mkdir(parents=True, exist_ok=True, mode=0o700)
    STATE.chmod(0o700)
    config_file = STATE / "settings.json"
    if not config_file.exists():
        config = {key: secrets.token_hex(32) for key in (
            "db_password", "db_owner_password", "chat_token", "query_token", "admin_token", "chat_ingest", "pet_ingest")}
        config_file.touch(mode=0o600)
        config_file.write_text(json.dumps(config))
    config = json.loads(config_file.read_text())
    for port in (3001, 3100, 55432):
        with socket.socket() as sock:
            sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
            try:
                sock.bind(("127.0.0.1", port))
            except OSError:
                raise SystemExit(f"port {port} 已佔用；請先查看 status 或停止本組服務")
    pg = pg_bin()
    pg_env = os.environ | {"PGPASSWORD": config["db_owner_password"]}
    password_file = STATE / "pg-password"
    password_file.touch(mode=0o600)
    password_file.write_text(config["db_owner_password"])
    if not (STATE / "postgres/PG_VERSION").exists():
        run([pg / "initdb", "-D", STATE / "postgres", "-U", "copilot_owner",
             "--auth-host=scram-sha-256", "--auth-local=scram-sha-256",
             "--pwfile", password_file, "--no-locale", "-E", "UTF8"], stdout=subprocess.DEVNULL)
    run([pg / "pg_ctl", "-D", STATE / "postgres", "-l", STATE / "postgres.log",
         "-o", f"-h 127.0.0.1 -p 55432 -k {STATE}", "start"], stdout=subprocess.DEVNULL)
    try:
        psql = [pg / "psql", "-h", "127.0.0.1", "-p", "55432", "-U", "copilot_owner", "-d", "postgres", "-v", "ON_ERROR_STOP=1"]
        if not (STATE / "db-initialized").exists():
            # 只初始化此工具建立的私人 cluster，不接觸既有系統資料庫。
            sql = f"CREATE ROLE analytics_user LOGIN PASSWORD '{config['db_password']}';\nCREATE DATABASE analytics_db OWNER analytics_user;\n"
            run(psql, input=sql, text=True, env=pg_env, stdout=subprocess.DEVNULL)
            (STATE / "db-initialized").touch()
        common = os.environ.copy()
        common.pop("OPENAI_API_KEY", None)
        common.update({"ANALYTICS_CHAT_TOKEN": config["chat_ingest"],
            "ANALYTICS_PET_TOKEN": config["pet_ingest"], "ANALYTICS_API_TOKEN": config["query_token"],
            "ANALYTICS_ADMIN_TOKEN": config["admin_token"], "CHAT_API_TOKEN": config["chat_token"],
            "CHAT_LISTEN": "127.0.0.1:3001", "ANALYTICS_LISTEN": "127.0.0.1:3100",
            "ANALYTICS_DATABASE_URL": f"postgres://analytics_user:{config['db_password']}@127.0.0.1:55432/analytics_db",
            "CHAT_DATABASE_URL": f"sqlite://{STATE}/chat.sqlite3", "CHAT_SITE_DIR": str(ROOT / "target/site/pkg"),
            "ANALYTICS_ENDPOINT": "http://127.0.0.1:3100/api/v1/events", "COOKIE_SECURE": "false"})
        for name, binary, port in (("analytics", "day9-analytics", 3100), ("chat", "day9-chat", 3001)):
            with (STATE / f"{name}.log").open("ab") as log:
                proc = subprocess.Popen([ROOT / "target/release" / binary], cwd=ROOT, env=common,
                                        stdout=log, stderr=log, start_new_session=True)
            (STATE / f"{name}.pid").write_text(str(proc.pid))
            for _ in range(100):
                if proc.poll() is not None:
                    raise RuntimeError(f"{name} 啟動失敗，請檢查私人日誌")
                if healthy(port):
                    break
                time.sleep(.2)
            else:
                raise RuntimeError(f"{name} 健康檢查逾時")
        print("Chat: http://127.0.0.1:3001/login\nAnalytics: http://127.0.0.1:3100")
    except BaseException:
        stop()
        raise
    finally:
        password_file.unlink(missing_ok=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=("start", "stop", "status"))
    args = parser.parse_args()
    os.umask(0o077)
    if os.getuid() == 0:
        raise SystemExit("請以一般使用者執行")
    STATE.mkdir(parents=True, exist_ok=True, mode=0o700)
    lock = (STATE / "manager.lock").open("a")
    fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    if args.action == "start":
        start()
    elif args.action == "stop":
        stop()
    else:
        print(json.dumps({"chat_healthy": healthy(3001), "analytics_healthy": healthy(3100)}, ensure_ascii=False))


if __name__ == "__main__":
    main()
