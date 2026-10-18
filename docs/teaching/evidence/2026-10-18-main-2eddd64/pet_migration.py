"""Disposable SQLite/SQLx migration checks; never reads an existing database."""
import os
from pathlib import Path
import sqlite3
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
VERSION = "20261018100114"
CHILDREN = ["pet_extra", "pet_activity", "pet_study", "pet_life_log"]


def migrate(db, source, expected=0):
    cmd = ["cargo", "sqlx", "migrate", "run", "--source", str(source)]
    result = subprocess.run(cmd, env={**os.environ, "DATABASE_URL": f"sqlite://{db}?mode=rwc"},
                            capture_output=True, text=True)
    print("COMMAND", " ".join(cmd), "EXIT", result.returncode)
    print(result.stdout + result.stderr, end="")
    assert (result.returncode == 0) == (expected == 0)
    return result


with tempfile.TemporaryDirectory(prefix="pet-migration-") as directory:
    base = Path(directory)
    old = base / "old"
    old.mkdir()
    for path in (ROOT / "migrations").glob("*.sql"):
        if not path.name.startswith(VERSION):
            (old / path.name).write_bytes(path.read_bytes())
    for scenario in ["fresh", "upgrade", "null_owner_id", "orphan_owner"]:
        print("CASE", scenario)
        db = base / (scenario + ".sqlite")
        if scenario != "fresh":
            migrate(db, old)
            with sqlite3.connect(db) as connection:
                connection.execute("INSERT INTO pet_owner(id,name) VALUES(?, 'fixture-owner')",
                                   (None if scenario == "null_owner_id" else 7,))
                connection.execute("INSERT INTO pet_profile(id,name,species,owner_id) VALUES(10,'fixture-pet','cat',?)",
                                   (8 if scenario == "orphan_owner" else 7,))
                for table in CHILDREN:
                    connection.execute(f"INSERT INTO {table}(id,pet_id,owner_id) VALUES(20,10,7)")
        invalid = scenario in ["null_owner_id", "orphan_owner"]
        result = migrate(db, ROOT / "migrations", expected=1 if invalid else 0)
        with sqlite3.connect(db) as connection:
            connection.execute("PRAGMA foreign_keys=ON")
            if invalid:
                assert "CHECK constraint failed" in result.stdout + result.stderr
                assert connection.execute("SELECT count(*) FROM _sqlx_migrations WHERE version=?", (int(VERSION),)).fetchone()[0] == 0
                assert connection.execute("SELECT count(*) FROM pet_profile").fetchone()[0] == 1
                assert "owner_id" in [row[1] for row in connection.execute("PRAGMA table_info(pet_profile)")]
                assert not connection.execute("SELECT name FROM sqlite_master WHERE name LIKE '%_new'").fetchall()
                print("PASS invalid ownership rejected; original data/schema and migration history preserved")
                continue
            assert not connection.execute("PRAGMA foreign_key_check").fetchall()
            if scenario == "upgrade":
                assert connection.execute("SELECT id,name FROM pet_owner").fetchall() == [("7", "fixture-owner")]
                assert connection.execute("SELECT id,pet_owner_id,name FROM pet_profile").fetchall() == [(10, "7", "fixture-pet")]
                for table in CHILDREN:
                    assert connection.execute(f"SELECT id,pet_id,pet_owner_id FROM {table}").fetchall() == [(20, 10, "7")]
            connection.execute("INSERT INTO pet_owner(id,name) VALUES('fixture-new','new owner')")
            connection.execute("INSERT INTO pet_profile(name,species,pet_owner_id) VALUES('new pet','cat','fixture-new')")
            pet_id = connection.execute("SELECT id FROM pet_profile WHERE name='new pet'").fetchone()[0]
            assert isinstance(pet_id, int)
            for table in CHILDREN:
                connection.execute(f"INSERT INTO {table}(pet_id,pet_owner_id) VALUES(?, 'fixture-new')", (pet_id,))
                assert isinstance(connection.execute(f"SELECT id FROM {table} WHERE pet_id=?", (pet_id,)).fetchone()[0], int)
            for sql in ["INSERT INTO pet_owner(name) VALUES('missing id')",
                        "INSERT INTO pet_profile(name,species,pet_owner_id) VALUES('invalid','cat','missing')",
                        "DELETE FROM pet_owner WHERE id='fixture-new'"]:
                try:
                    connection.execute(sql)
                except sqlite3.IntegrityError:
                    pass
                else:
                    raise AssertionError("missing ID/FK was accepted: " + sql)
            assert not connection.execute("PRAGMA foreign_key_check").fetchall()
            print("PASS migrated; preserved data; generated integer IDs; enforced owner IDs and foreign keys")
print("PASS 4 scenarios; SQLite", sqlite3.sqlite_version)
