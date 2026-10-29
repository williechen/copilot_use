import datetime
import json
import pathlib
import sqlite3
import sys
import time

database = pathlib.Path(sys.argv[1])
log = pathlib.Path(sys.argv[2])
previous = None
while True:
    try:
        connection = sqlite3.connect(f"file:{database}?mode=ro", uri=True)
        rows = connection.execute(
            "SELECT id,name,is_default_closure FROM pet_profile "
            "WHERE id IN (?,?) ORDER BY id",
            ("00000000-0000-0000-0000-0000000000a1", "00000000-0000-0000-0000-0000000000b2"),
        ).fetchall()
        connection.close()
        if rows != previous:
            event = {"time": datetime.datetime.now(datetime.timezone.utc).isoformat(), "rows": rows}
            with log.open("a") as output:
                output.write(json.dumps(event, ensure_ascii=False) + "\n")
            previous = rows
    except sqlite3.Error as error:
        with log.open("a") as output:
            output.write(json.dumps({"error": str(error)}) + "\n")
    time.sleep(0.5)
