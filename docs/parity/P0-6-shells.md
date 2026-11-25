# P0-6 client shells parity ledger

Foundation slice. Rows for content that later slices port are `open` with the slice id in the Mungini column; that is expected here. This slice only delivers the shells, the shared package skeletons and the i18n structure.

| Legacy location | Mungini location | Status |
|---|---|---|
| `troq-front/troqzone-web` toolchain (Next.js, React 19 line) | `apps/web` (Next.js 16.4.6, React 19.2.3, port 3101) | done |
| `troq-front/troqzone-web` Stay screens and components | `apps/web` guest screens | open, F2 |
| `troq-front/troqzone-business` toolchain (Vite) | `apps/host` (Vite 6, React 19.2.3, port 5273) | done |
| `troq-front/troqzone-business` Stay screens, types and routes | `apps/host` screens and module registry entries | open, F3 |
| `troq-front/troqzone-business` Tailwind and app shell | `packages/theme`, `packages/ui-web`, host shell | open, F0 |
| `troq-front/troqzone-admin` toolchain (Vite) | `apps/admin` (Vite 6, React 19.2.3, port 5274) | done |
| `troq-front/troqzone-admin` Stay screens, types and routes | `apps/admin` screens and module registry entries | open, F4 |
| `troq-front/troqzone-admin` app shell | `packages/ui-web`, admin shell | open, F0 |
| `troqzone-app/apps/mobile` toolchain (Expo, Metro monorepo resolver) | `apps/mobile` (Expo 57, React Native 0.86.3, React 19.2.3) | done |
| `troqzone-app/apps/mobile` Stay screens and components | `apps/mobile` guest area | open, F5 |
| `troqzone-app/packages/merchant/src/stays` | `apps/mobile` host area | open, F6 |
| `troqzone-app/apps/mobile` NativeWind, Firebase, fonts, navigation, other TROQ features | not carried, Mungini mobile is a separate Expo app | dropped by decision D9 |
| Module registry (navigation by registry) | `apps/host/src/modules/registry.ts`, `apps/admin/src/modules/registry.ts`, empty lists | done |
| `troqzone-app/packages/shared-api` | `packages/api` skeleton | open, F1 |
| `troqzone-app/packages/shared-types` | `packages/types` skeleton | open, F1 |
| `troqzone-app/packages/shared-state` | `packages/state` skeleton | open, F1 |
| `troqzone-app/packages/shared-storage` | `packages/storage` skeleton | open, F1 |
| `troqzone-app/packages/shared-utils` | `packages/utils` skeleton | open, F1 |
| `troqzone-app/packages/shared-hooks` | `packages/hooks` skeleton | open, F1 |
| `troqzone-app/packages/shared-ui` | `packages/ui-native` skeleton | open, F1 |
| `troqzone-app/packages/shared-theme` | `packages/theme` | open, F0 |
| Web UI primitives of the three TROQ web apps | `packages/ui-web` | open, F0 |
| `troqzone-app/packages/shared-i18n` | `packages/i18n` (typed per-feature en and fr files, `common` feature with the app name) | done |
| Stay locale files in the three web apps and the mobile app | `packages/i18n/locales/{en,fr}/<feature>.ts` | open, F7 |
| `troqzone-app/packages/delivery`, `troqzone-app/packages/merchant` (non-Stay) | not carried, Mungini mobile is a separate Expo app | dropped by decision D9 |
| Generated API client | `packages/api` | open, P0-4 |

Notes:

- The mobile reference actually runs Expo 57 and React Native 0.86.3 (the TROQ guide still says Expo 54 and RN 0.81). Mungini follows the versions in the reference `package.json`.
- Root pins `react` and `react-dom` 19.2.3 through `overrides`. `bunfig.toml` sets the hoisted linker so Metro and Next find one copy of React.
- Ports 3101, 5273 and 5274 are chosen so they do not collide with the TROQ dev servers.
