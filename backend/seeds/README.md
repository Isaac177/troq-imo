# Demo seeds

`00-order.txt` lists the Mungini seed files in dependency order. The current list includes the available identity, property, inventory, pricing, reservation, review, guest, notification and billing examples; it is not a complete Stay demo dataset. The parity ledger in `docs/parity/B12-migrations.md` tracks the remaining source data.

These records are for development databases only. Apply the Mungini migrations first, then load the listed files in order when the owner requests a local seed run. Seed identifiers start with `seed-`, and inserts leave existing rows with matching identifiers unchanged. Do not load these records into shared, staging or production databases.

Adding a feature includes updating the ordered list and its parity ledger, with realistic examples covering the feature's states and both supported cities where applicable. A seed file must follow its schema and data dependencies; do not rely on alphabetical discovery.

The OTP examples in `identity/07-registration-otp.sql` use the default development-only `OTP_SIGNING_KEY` from `infra/.env.example` and the code `123456`. If the local signing key differs, the seeded pending example cannot be verified. OTP records are not messages and no delivery is performed by the seed.
