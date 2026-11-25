# Parity ledger F1: shared packages

The CDF formatter was moved from the F0 web UI foundation into the platform-independent utilities package and remains re-exported by the web UI package. It retains its existing formatting behavior. Type checks passed in both packages.

| Legacy location | Mungini location | Status |
|---|---|---|
| `troq-front/troqzone-business/src/utils/stays/formatCdf.ts` | `packages/utils/src/money.ts` | done |
| `troq-front/troqzone-business/src/utils/stays/stayDateTime.ts` | `packages/utils/src/date/formatDay.ts` with explicit property time zone; re-exported by `ui-web` | done |
| Other shared API, state, storage, hooks, utilities and cross-platform contracts | `packages/*` | open: inventory and port remain |
