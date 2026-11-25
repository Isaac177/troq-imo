# Mungini

Standalone accommodation booking product, extracted from TROQ Stay. One Rust modular monolith backend and every client in the same repository.

## Layout

- `backend/`: Cargo workspace with the `kernel`, `domain`, `api` and `jobs` crates, `migrations/` and `seeds/`.
- `apps/web`: guest site (Next.js).
- `apps/host`: host app (Vite, mobile first).
- `apps/admin`: platform staff app (Vite).
- `apps/mobile`: Expo app for guests and hosts.
- `packages/*`: shared packages (`api`, `types`, `i18n`, `state`, `storage`, `theme`, `utils`, `hooks`, `ui-web`, `ui-native`).
- `infra/`: compose files and Dockerfile. Images are built on the server, with no registry.
- `docs/`: specs, `parity/` ledgers, `legacy-troq-stays/` reference.

## Start here

Read `AGENTS.md`, then `MUNGINI_MIGRATION_PLAN.md`.

## Tooling

Bun for every JavaScript workspace. Rust services are compiled by the owner's `cargo watch`.

## Local development

From the Mungini root, install dependencies with `bun install`, then start the local infrastructure and backend watcher:

```sh
make dev
```

The command creates `infra/.env.dev` from `infra/.env.example` if needed. Start web, host or admin with `bun run dev` in its app folder. Their default ports are 3101, 5273 and 5274. Run `bun run start` in `apps/mobile` for Expo. The API exposes `/api/public/health` and `/api/openapi.json`; feature routes are still in progress. See `infra/README.md` for container and production configuration.
