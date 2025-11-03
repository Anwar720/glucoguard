#!/usr/bin/env python3
import csv
import sqlite3
from pathlib import Path

DB_PATH = Path("/Users/anwarjahid/Documents/Classes/security engineering/Project/glucoguard/data/database.db")
CSV_PATH = Path("/Users/anwarjahid/Documents/Classes/security engineering/Project/glucoguard/pump_simm/gcm_reader.csv")

def main():
    conn = sqlite3.connect(str(DB_PATH))
    try:
        conn.execute("PRAGMA foreign_keys = ON;")
        cur = conn.cursor()

        # Recreate temp table with explicit column names
        cur.execute("DROP TABLE IF EXISTS temp_readings;")
        cur.execute("""
            CREATE TABLE temp_readings (
              patient_id TEXT,
              glucose_level REAL,
              status TEXT
            );
        """)

        # Load CSV (skip header)
        with CSV_PATH.open(newline="", encoding="utf-8") as f:
            reader = csv.reader(f)
            header = next(reader, None)  # skip header row if present
            rows = []
            for row in reader:
                if not row or len(row) < 3:
                    continue
                patient_id = row[0]
                glucose_level = float(row[1])
                status = row[2]
                rows.append((patient_id, glucose_level, status))

        if rows:
            cur.executemany(
                "INSERT INTO temp_readings (patient_id, glucose_level, status) VALUES (?, ?, ?);",
                rows,
            )

        # Insert into final table, forcing current timestamp in reading_time
        cur.execute("""
            INSERT INTO glucose_readings (patient_id, glucose_level, status, reading_time)
            SELECT patient_id, glucose_level, status, datetime('now')
            FROM temp_readings;
        """)

        cur.execute("DROP TABLE temp_readings;")
        conn.commit()
        print("Imported readings successfully.")
    except Exception as e:
        conn.rollback()
        raise
    finally:
        conn.close()

if __name__ == "__main__":
    main()