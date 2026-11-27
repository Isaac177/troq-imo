# Offline cutover reconciliation

This tool compares exported identifiers from the city databases and Mungini without connecting to or modifying a database. It is a preflight and reconciliation aid, not a copy script or proof that all data was copied.

Prepare UTF-8 JSON Lines files containing one object per record. Each object has `table`, `city` and `key`; `key` is an array of strings in primary-key column order. Use the destination table name in both files after confirming the source-to-target mapping, use Mungini city codes consistently, and include the same city-owned tables and scope in both exports. Include both source cities in the source file. Export the source and target from consistent snapshots and keep any personal identifiers and results protected.

Example rows:

```json
{"table":"stay_reservations","city":"GOMA","key":["reservation-id"]}
{"table":"stay_pool_physical_units","city":"GOMA","key":["pool-id","unit-id"]}
```

For example, a read-only export of reservation identifiers from the Goma Stay database can use `SELECT json_build_object('table', 'stay_reservations', 'city', 'GOMA', 'key', json_build_array(id)) FROM public.stay_reservations`. Use `KIN` for the Kinshasa source. The corresponding Mungini query is `SELECT json_build_object('table', 'stay_reservations', 'city', city_id, 'key', json_build_array(id)) FROM stay_reservations WHERE city_id IN ('GOMA', 'KIN')`. Configure the SQL client's unaligned, tuples-only output so that each JSON object occupies one line. Run these against consistent read-only snapshots; these examples do not export the other tables.

Run `bun scripts/cutover/reconcile-cli.ts source.jsonl target.jsonl > report.json`. The report provides per-table and city row counts and identifies duplicate snapshot keys, source records missing in the target, extra target records, and keys repeated between source cities. Exit status is 0 when no findings are found, 1 when findings exist and 2 for invalid input or I/O errors. An empty source is rejected; an empty target is valid before a copy.

Cross-city key findings identify possible collisions, not automatic merge instructions. Check the actual destination key and its city scope before deciding how to handle them. Do not include shared global reference tables in a city-owned export without accounting for their different merge semantics. Target seed data can produce extra-target findings and must be reviewed rather than deleted automatically.

The tool compares only rows supplied to it. It does not confirm that an export includes every table or city, verify column values, enforce referential integrity, validate media objects, reconcile auth or CRM contacts, or replay notifications. Empty tables have no rows and therefore do not appear in the report. Use an independently reviewed table and column mapping and verify completeness separately.
