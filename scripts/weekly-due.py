#!/usr/bin/env python3
"""systemd ExecCondition：提前完成同一週後，不在原排程重跑；手動腳本不受限制。"""
import argparse
import datetime
import json
from pathlib import Path

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--state',type=Path,default=Path(__file__).resolve().parents[1]/'docs/learning-state.json')
    parser.add_argument('--now')
    args=parser.parse_args()
    due=datetime.datetime.fromisoformat(json.loads(args.state.read_text())['next_full_comparison'])
    now=datetime.datetime.fromisoformat(args.now) if args.now else datetime.datetime.now(datetime.timezone.utc)
    if due.tzinfo is None or now.tzinfo is None:
        raise SystemExit('週六到期時間必須含時區')
    raise SystemExit(0 if now>=due else 1)
if __name__=='__main__': main()
