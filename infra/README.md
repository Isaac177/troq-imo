# Mungini infra

Files in this folder:

- `docker-compose.dev.yml`: local stack. Postgres 17 with PostGIS (`db`, database `mungini_dev`), Redis 7.2 (`redis`), MinIO (dev only, `minio-init` creates the two buckets) and the backend (`api`, built from `Dockerfile`). Container names are `mungini-*-dev`.
- `docker-compose.yml`: production compose for the backend only (`api`). It builds the image on the server from `Dockerfile`, reads `.env` and binds to 127.0.0.1:8080 for a reverse proxy. No registry is used.
- `Dockerfile`: multi-stage build of the `mungini-api` binary from `backend/` (cargo-zigbuild, musl), runtime on alpine with tini. Its build context is the Mungini root, so `Dockerfile.dockerignore` sits beside it.
- `Dockerfile.dockerignore`: keeps `apps/`, `packages/`, `docs/` and `target/` out of the build context.
- `.env.example`: development configuration template. The local file is `.env.dev`.
- `.env.prod.example`: the production configuration template. The local production file is `.env`. Object storage is Cloudflare R2 through the same `STORAGE_S3_*` names as TROQ: set `STORAGE_S3_ENDPOINT` to the account's R2 endpoint and `STORAGE_S3_REGION=auto`.
- `postgres/init-extensions.sql`: enables PostGIS on first start of the database.

Local development, from the Mungini root:

1. Run `make dev` to start the dependencies and the backend under `cargo watch`. The command creates `infra/.env.dev` from `infra/.env.example` if it is missing waits for the dependencies to be ready, and runs the one-time MinIO bucket setup. Run `make dev-infra` to start and configure only the dependencies.
2. To use the containerized backend instead, run `docker compose --env-file infra/.env.dev -f infra/docker-compose.dev.yml up -d --build`. The first build compiles Rust in release mode and is slow.

When editing `infra/.env.dev`, keep the PostgreSQL password in `DATABASE_URL` in sync with `POSTGRES_PASSWORD`, the Redis password in `REDIS_URL` in sync with `REDIS_PASSWORD`, and the MinIO root credentials in sync with the storage access keys.

The API does not apply migrations or load seeds at startup. The owner handles local database migrations and seed runs after the required backup.

Production configuration:

- `infra/.env` is the production environment file and is ignored by Git. Create it from `infra/.env.prod.example` if needed and fill in the real database, Redis, Cloudflare R2 and JWT values on the production host. Blank values are not usable production credentials.
- Production Postgres and Redis are external to the production compose file. Production object storage is Cloudflare R2.
- The owner handles production builds and deployments. `backend/Cargo.lock` must be committed once Cargo generates it. Compose needs BuildKit (Docker 23 or later).
