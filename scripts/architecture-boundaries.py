#!/usr/bin/env python3
"""驗證實際 Cargo 依賴圖，防止 Driver/Adapter 回流到純 Domain/Application。"""
import json
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
PURE = {"chat-domain", "chat-application", "pet-domain"}
FORBIDDEN = {"reqwest", "axum", "hyper", "tauri", "gtk", "glib", "day9-pet",
             "day9-chat", "day9-analytics", "persistence", "mcp-bridge",
             "openai-adapter", "copilot-telemetry"}


def main():
    metadata = json.loads(subprocess.check_output(
        ["cargo", "metadata", "--locked", "--format-version", "1"]
        + (["--offline"] if "--offline" in sys.argv else []), cwd=ROOT
    ))
    names = {p["id"]: p["name"] for p in metadata["packages"]}
    nodes = {n["id"]: n for n in metadata["resolve"]["nodes"]}
    roots = {p["name"]: p["id"] for p in metadata["packages"]
             if p["id"] in metadata["workspace_members"] and p["name"] in PURE}
    if set(roots) != PURE:
        raise SystemExit("Missing pure workspace packages: " + str(PURE - roots.keys()))
    failures = []
    for root_name, root_id in roots.items():
        pending = [(root_id, [root_name])]
        seen = set()
        while pending:
            package, chain = pending.pop()
            if package in seen:
                continue
            seen.add(package)
            for dependency in nodes[package]["deps"]:
                # dev dependencies belong to test harnesses, never runtime architecture.
                if not any(k["kind"] in (None, "build") for k in dependency["dep_kinds"]):
                    continue
                child = dependency["pkg"]
                name = names[child]
                route = chain + [name]
                if name in FORBIDDEN or name.startswith(("sqlx", "tauri-")):
                    failures.append(" -> ".join(route))
                pending.append((child, route))
    if failures:
        raise SystemExit("Adapter dependency crossed core boundary:\n" + "\n".join(failures))
    for root_name in sorted(roots):
        print("PASS pure normal/build dependencies:", root_name)


if __name__ == "__main__":
    main()
