# P0-3 infra parity ledger

Legacy items from TROQ `tzone-rs` (Stay infra) and their Mungini locations. Status is `done`, `dropped by decision D#`, `dropped by architecture` (with the section cited) or `open`. Rows marked `open` need the owner before P0-3 is closed.

| Legacy location | Mungini location | Status |
|---|---|---|
| `tzone-rs/docker-compose.dev.yml` service `stay-db` (container `troqzone-stay-db-dev`, port 6007, volume `stay_db_data`) | `infra/docker-compose.dev.yml` service `db` (container `mungini-db-dev`, port 6200, volume `mungini_db_data`, PostGIS) | done |
| `tzone-rs/docker-compose.yml` service `stay-db` (container `troq-stay-db`, volume `stay_db_data`) | none in `infra/docker-compose.yml`, which runs only `api` | open (owner: where production Postgres runs) |
| `tzone-rs/docker/postgres/init-city-dbs.sh` creates `stay-db_goma` and `stay-db_kin` | `infra/postgres/init-extensions.sql` enables PostGIS on one database `mungini_dev` | dropped by decision D4 (one database with `city_id`) |
| `tzone-rs/docker-compose.yml` service `stay-service` (image `ghcr.io/isaac177/troq-stay:latest`, container `troq-stay`) | `infra/docker-compose.yml` service `api` (built on the server from `infra/Dockerfile`, no registry, container `mungini-api`) and `infra/docker-compose.dev.yml` service `api` | done |
| `tzone-rs/stay-service/Dockerfile` (cargo-zigbuild musl build, alpine runtime with tini, copies `stay-service/migrations`) | `infra/Dockerfile` (same toolchain, builds `mungini-api` from `backend/`, copies `backend/migrations`) | done |
| `tzone-rs/docker-bake.hcl` target `stay-service` (tag `troq-stay`) | none: images are built on the server | dropped by owner decision (build on server, no registry) |
| `tzone-rs/Makefile` `docker-push` line for `troq-stay` | none: no image push | dropped by owner decision (build on server, no registry) |
| Build context exclusions from `tzone-rs/.dockerignore` | `infra/Dockerfile.dockerignore` | done |
| `STAY_DATABASE_URL` and `STAY_DATABASE_URL_KIN` (`.env.example`, `stay-service/src/config.rs`) | `DATABASE_URL` in `infra/.env.example`, one database (D4) | done |
| `STAY_SERVICE_PORT` (default 3090) | `API_PORT` (default 8080) in `infra/.env.example` and `infra/docker-compose.dev.yml` | done |
| `STAY_GUEST_RETENTION_DAYS` (`stay-service/src/config.rs`) | `GUEST_RETENTION_DAYS` in `infra/.env.example`. The consumer is in `backend/crates/jobs` (slice B11) | done (variable listed; code is B11) |
| `redis` service in dev compose (`redis:7.2-alpine`, volume `redis_data`) | `redis` in `infra/docker-compose.dev.yml` (container `mungini-redis-dev`, port 6380, volume `mungini_redis_data`), `REDIS_URL` in `infra/.env.example` | done |
| `redis` service in production compose | none in `infra/docker-compose.yml` | open (owner: where production Redis runs) |
| `minio` and `minio-init` in dev compose (volume `uploads_data`) | `minio` and `minio-init` in `infra/docker-compose.dev.yml` (ports 9100 and 9101, volume `mungini_minio_data`) | done |
| `minio` and `minio-init` in production compose | none: production media is Cloudflare R2 through `STORAGE_S3_*` (`infra/.env.prod.example`) | dropped by owner decision (R2 in production) |
| `minio-init` per-city buckets from `MINIO_BUCKET_CITIES` (`troqzone-<city>`) | `minio-init` creates `STORAGE_BUCKET_PUBLIC` and `STORAGE_BUCKET_PRIVATE` only, one set for all cities | dropped by decision D4 |
| `STORAGE_S3_*`, `STORAGE_BUCKET_PUBLIC`, `STORAGE_BUCKET_PRIVATE`, `STORAGE_PUBLIC_BASE_URL`, `MINIO_ROOT_*` (`.env.example`) | same names in `infra/.env.example`, matching `troq-storage` | done |
| `nats` service in dev and production compose (`nats:2.10-alpine`, `nats_data`, `docker/nats/nats-server.conf`), `NATS_*` variables | none | dropped by architecture: `MUNGINI_ARCHITECTURE.md`, "What is deliberately dropped" and "Transport and contract" (no NATS in the first version) |
| `STAY_EVENTS` JetStream stream (`stay.events.*`) and its consumers | none. Events go through the in-process outbox in `backend/crates/kernel` and `backend/crates/jobs` | dropped by architecture: `MUNGINI_ARCHITECTURE.md`, "What is deliberately dropped" |
| `.env.enc` and `.env.dev.enc` in `tzone-rs` holding `STAY_*` values | no encrypted env files are created in Mungini | open (owner removes the `STAY_*` values from the TROQ encrypted files at removal slice R1a) |
| `troqzone-network` / `troq-network` compose network | `mungini-network-dev` (dev) and `mungini-network` (production) | done |
