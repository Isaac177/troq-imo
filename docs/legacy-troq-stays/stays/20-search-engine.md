# Stay search engine

Status: implemented in stay-service, engine version 3. Public routes are live through the gateway. Decisions taken for this engine are listed in section 13.

Audience: developers who add filters or modules, data engineers who build pipelines on the search events, and language models that answer questions about searches, offers, prices, availability and demand. Every field name, code and rule below is the contract the code enforces.

---

## 1. Purpose and boundary

The search engine answers one question: which stays can a guest book for these dates and guests, with which filters, in what order, and at what price. It also returns the facets and statistics that describe the answer, and it can record an analytics event for each search when the client gives consent.

Boundary rules:

- The engine lives in `tzone-rs/stay-service/src/search_engine/`. It has no HTTP, NATS or gateway code. Callers pass a database pool, a city code, a validated query and a consent flag, and receive a response value.
- Transport code lives elsewhere. The public discovery handler (`public_discovery/handler.rs`) and the gateway (`api-gateway/src/routes/stays/public_discovery.rs`) only translate between NATS or HTTP and the engine.
- Later modules reuse the engine by calling `engine::run`. Examples: recommendations that need candidate sets, operator reports that need facet counts, pricing previews, and pipelines that read `stay_search_events`.
- The engine never writes stays, holds, allocations or rates. Its only write is the optional analytics event.

Versioning: the engine version is returned with every response (`engineVersion`). A change to ranking, to a default, or to the meaning of a field increments the version. Additive filters and fields do not require a new version but must be listed in the catalog and in this document.

---

## 2. Architecture

A search runs in four phases. Each phase has one input and one output, so a later module can stop after any phase.

| Phase | Module | Input | Output | Cost driver |
|---|---|---|---|---|
| A. Candidates | `sql.rs` | validated query | one row per candidate plan | one SQL statement, indexed |
| B. Restrictions and pricing | `pricing.rs` | candidate rows | offers with stay totals | batched loads, one pure pricing call per plan |
| C. Ranking and paging | `stage.rs` | priced offers | hits, sorted, one page | SQL aggregation and pagination |
| D. Facets, stats and cover | `stage_aggregates.rs`, `engine.rs` | staged offers and page ids | facets, stats, cover images | SQL counts and statistics; one cover query for the page |
| E. Analytics (optional) | `events.rs` | canonical query and counts | one row in `stay_search_events` | one insert, only with consent |

Entry point: `engine::run(pool, city_code, query, analytics_consent) -> SearchResponse`.

Phase A reads candidate plans in batches of 250 inside a repeatable-read transaction. Pricing uses the same snapshot and existing quote rules. Priced offers are staged in a transaction-local table, and SQL applies price bounds, aggregates facets and totals, and selects the requested page without a candidate cap.

Phase B:

1. Restriction verdicts for every candidate (property, unit type) pair come from one batched query (`restrictions::batch::violations_batch`). A pair with any violated rule is dropped.
2. Rate rules for all candidate plans come from one query, and promotions for all candidate properties come from another.
3. Each plan is priced with the same function the quote uses (`quotes::charges::price_stay`), so a search total equals the quote total for the same inputs.
4. A plan with a missing nightly rate on any night is unbookable and is dropped. It is never priced at zero.

Phase C groups offers by property, picks the best offer per property, sorts the property hits, and slices one page.

Phase D computes facets and statistics over all hits (not only the returned page). Cover images are fetched only for the returned page.

---

## 3. Data used

Tables read by Phase A and B:

| Table | Used for |
|---|---|
| `stay_properties` | city, status, operator link, timezone, verification, amenities, coordinates |
| `stay_operators` | status (only `APPROVED` operators are searchable) |
| `stay_unit_types` | status, occupancy limits, accessibility codes, inventory model, category code, room counts |
| `stay_inventory_pools`, `stay_night_inventory` | sellable and allocated capacity per night |
| `stay_availability_blocks`, `stay_physical_units` | physical-room blocks overlapping the stay |
| `stay_rate_plans` | status, booking mode, meal plan, occupancy basis, child nightly amount, policy links |
| `stay_policy_versions` | cancellation content (`freeUntilHours`, `penaltyPercent`) and payment content (`collection`) |
| `stay_rate_rules`, `stay_promotions` | pricing, via the shared quote pricing |
| `stay_restrictions` | minimum and maximum nights, closures, notice, horizon, turnover buffers |
| `stay_property_media` | cover image of the returned page only |

Tables written by the engine: `stay_search_events` (only with consent).

Location privacy: distance is computed from each property's coordinates rounded to two decimals (about 1.1 km), in the same SQL statement that computes it. The exact coordinates never leave the database, and a distance cannot be used to recover them more closely than that cell. The public DTOs contain no coordinates and no address.

Migration 00021 adds:

- `stay_properties.amenities` (`jsonb` array of amenity codes, default `[]`, checked to be an array).
- Partial indexes on published properties by city, on active unit types by property, on active rate plans by unit type, and on inventory pools by unit type.
- GIN indexes on unit accessibility and on property amenities.
- `stay_search_events`, the analytics table (section 9).

Property type is not stored. It is derived from the unit inventory model: `WHOLE_UNIT` is `ENTIRE_PLACE`, and any other model is `ROOM`.

---

## 4. Query model

The request is validated into a canonical `SearchQuery`. The canonical form has sorted, de-duplicated codes, normalised numbers, rounded coordinates and the sort applied. Two requests with the same canonical form have the same fingerprint.

| Field (camelCase) | Type | Default | Rules | Effect |
|---|---|---|---|---|
| `propertyId` | id | none | non-empty | restricts to one property (used by availability) |
| `checkIn` | date `YYYY-MM-DD` | none | needs `checkOut`; must be before `checkOut` | first night |
| `checkOut` | date | none | needs `checkIn`; at most 365 nights after `checkIn` | departure day; nights are `[checkIn, checkOut)` |
| `adults` | integer | 1 | 1 to 20 | occupancy: unit `maxAdults` must be at least this |
| `children` | integer | 0 | 0 to 10 | occupancy: unit `maxChildren` must be at least this |
| `childAges` | list of integers | required when `children` > 0 | exactly one age per child, each 0 to 17; sorted in the canonical form | not used by pricing yet (section 13); stored in the fingerprint, never in analytics |
| `instantBooking` | boolean | false | | keeps only plans with booking mode `INSTANT` |
| `propertyTypes` | list | all | `ENTIRE_PLACE`, `ROOM` | unit inventory model |
| `mealPlans` | list | all | `ROOM_ONLY`, `BREAKFAST`, `HALF_BOARD`, `FULL_BOARD` | plan meal plan |
| `paymentCollections` | list | all | `PREPAY`, `PAY_AT_PROPERTY` | payment policy content `collection` |
| `accessibility` | list of codes | all | `STEP_FREE_ENTRY`, `GROUND_FLOOR`, `ACCESSIBLE_BATHROOM`, `WIDE_DOORWAYS`, `HEARING_LOOP` | every code must be on the unit (`?&`) |
| `categories` | list of codes | all | active codes from the accommodation category catalog (`GET /api/public/stays/accommodation-categories`) | unit `categoryCode` is one of the codes |
| `amenities` | list of codes | all | `WIFI`, `AIR_CONDITIONING`, `FAN`, `HEATING`, `HOT_WATER`, `TV`, `WORKSPACE`, `WASHING_MACHINE`, `HAIR_DRYER`, `KITCHEN`, `REFRIGERATOR`, `MICROWAVE`, `COFFEE_MAKER`, `KITCHENWARE`, `DINING_AREA`, `PRIVATE_BATHROOM`, `BATHTUB`, `TOILETRIES`, `GARDEN`, `POOL`, `TERRACE`, `BARBECUE`, `OUTDOOR_FURNITURE`, `LAKE_VIEW`, `MOUNTAIN_VIEW`, `BEACH_ACCESS`, `PARKING`, `GARAGE`, `EV_CHARGER`, `BICYCLES`, `AIRPORT_SHUTTLE`, `GENERATOR`, `SOLAR_POWER`, `BACKUP_BATTERY`, `WATER_TANK`, `FIBER_INTERNET`, `SATELLITE_INTERNET`, `SECURITY`, `GATED_COMPOUND`, `CCTV`, `SAFE`, `SMOKE_DETECTOR`, `FIRE_EXTINGUISHER`, `FIRST_AID_KIT`, `MOSQUITO_NET`, `BREAKFAST`, `RESTAURANT`, `BAR`, `ROOM_SERVICE`, `HOUSEKEEPING`, `LAUNDRY_SERVICE`, `RECEPTION_24H`, `AIRPORT_PICKUP`, `LUGGAGE_STORAGE`, `BABY_COT`, `KIDS_PLAY_AREA`, `EXTRA_BED`, `ELEVATOR`, `GYM`, `MEETING_ROOM`, `GAMES` | every code must be on the property |
| `priceMin` | money in CDF, e.g. `150000.00` | none | needs dates; at most two decimals; not above `priceMax` | lower bound of the stay total of the offer |
| `priceMax` | money in CDF | none | needs dates | upper bound of the stay total of the offer |
| `minFreeCancellationHours` | integer | none | 0 to 8784 | plan cancellation `freeUntilHours` must be at least this |
| `maxPenaltyPercent` | integer | none | 0 to 100 | plan cancellation `penaltyPercent` must be at most this |
| `latitude`, `longitude` | decimal degrees | none | both or neither; ranges checked; rounded to 2 decimals (about 1.1 km) | search point; the property side is rounded to the same precision (section 3) |
| `maxDistanceKm` | number | none | above 0, at most 500; needs a point | distance limit |
| `sort` | enum | `relevance` | `relevance`, `name`, `price`, `distance` | section 5 |
| `page` | integer | 1 | at least 1 | |
| `pageSize` | integer | 20 | 1 to 100 | |
| `analyticsConsent` | boolean | false | not part of the fingerprint | records one analytics row when true |

Unknown codes are rejected with a message that lists the allowed values. Codes are upper-cased and trimmed before the check, so `wifi` and `WIFI` are the same.

The fingerprint is the first 16 hexadecimal characters of the SHA-256 of the canonical JSON of the `SearchQuery`. It is stable for equal queries and changes when any filter changes. `analyticsConsent` and paging do not change the fingerprint, but page and page size are part of the canonical JSON, so the fingerprint identifies one page of one query.

---

## 5. Sorting

| Sort | Key (ascending) | Needs | Notes |
|---|---|---|---|
| `relevance` | verified first; then instant booking first; then lower best total; then `nameEn`; then `id` | none | Version 2 ranking. Reviews join it when spec 12 exists; a change to this order requires a new engine version. |
| `name` | `nameEn`, then `id` | none | Alphabetical, deterministic |
| `price` | best offer total, then `nameEn`, then `id` | dates | Total is the stay total of the cheapest offer that passed every filter |
| `distance` | `distanceKm`, then `nameEn`, then `id` | a search point | Haversine distance from the rounded point to the property |

Distance uses the haversine formula with Earth radius 6371 km, computed in SQL on the stored property coordinates. A property without coordinates is excluded by a distance filter and sorts last.

---

## 6. Response

```text
{
  engineVersion: 1,
  fingerprint: "3f6d2e5b0f0eb4cf",
  query: { ...canonical query... },
  dataAsOf: "2026-10-13T07:47:38Z",          time the answer was computed
  items: [ hit, ... ],                        one page
  total: 1,                                   all hits, not only this page
  page: 1,
  pageSize: 20,
  facets: { ... },                            over all hits
  stats: { ... },                             over all hits
  meta: { candidatePlans, pricedPlans, timingsMs: { candidates, pricing, total }, analyticsRecorded }
}
```

A hit:

```text
{
  id, nameEn, nameFr,
  verificationStatus: "VERIFIED" | "PENDING" | "UNVERIFIED" | "REJECTED",
  cover: { mediaId, altEn, altFr, contentPath } | null,
  reviewCount: 0,                 always 0 until reviews exist (spec 12)
  verifiedRating: null,           always null until reviews exist
  distanceKm: number | null,      one decimal, only with a search point
  amenities: [codes],
  instantBooking: boolean,        at least one offer is INSTANT
  best: offer,                    lowest total; lowest rate plan id when there are no dates
  offers: [offer, ...]            every plan that passed the filters, lowest total first
}
```

An offer:

```text
{
  unitTypeId, unitNameEn, unitNameFr,
  ratePlanId,
  propertyType: "ENTIRE_PLACE" | "ROOM",
  bookingMode: "INSTANT" | "REQUEST",
  mealPlan, occupancyBasis: "PER_UNIT" | "PER_GUEST",
  paymentCollection: "PREPAY" | "PAY_AT_PROPERTY" | "UNKNOWN",
  cancellation: { class, freeUntilHours, penaltyPercent },
  accessibility: [codes],
  bedroomCount, bedCount, bathroomCount,
  remainingRooms: integer | null,   null without dates; the lowest free count across the nights
  total: "180000.00" | null          CDF stay total; null without dates
}
```

Money is a string with two decimals in CDF. Dates are `YYYY-MM-DD`. Times are RFC 3339 UTC.

### Example

A search for two nights in December 2026 in Goma, for two adults, sorted by price, for ROOM units only, with consent. Abbreviated from a real response on the seeded dev database:

```text
GET /api/public/stays/properties?checkIn=2026-12-27&checkOut=2026-12-29&adults=2&sort=price&propertyTypes=ROOM&analyticsConsent=true

total: 1, fingerprint: 3f6d2e5b0f0eb4cf
item: Lakeside Villa, best offer: Triple room, ROOM, INSTANT, ROOM_ONLY, PREPAY,
      cancellation FLEXIBLE (48 h, 0 %), remainingRooms 3, total "180000.00"
facets.propertyType: [{ value: ROOM, count: 1 }]
stats: resultCount 1, pricedCount 1, price { min, median, max, average } = "180000.00"
meta.candidatePlans 1, pricedPlans 1, analyticsRecorded true
```

---

## 7. Facets and statistics

Facets count properties, not offers. A property is counted once per value even when several of its offers carry that value.

| Facet | Values | Source |
|---|---|---|
| `propertyType` | `ENTIRE_PLACE`, `ROOM` | offers |
| `mealPlan` | the four meal codes | offers |
| `paymentCollection` | `PREPAY`, `PAY_AT_PROPERTY`, `UNKNOWN` | offers |
| `cancellationClass` | `FLEXIBLE`, `MODERATE`, `NON_REFUNDABLE` | offers |
| `bookingMode` | `INSTANT`, `REQUEST` | offers |
| `accessibility` | accessibility codes | offers (unit level) |
| `amenities` | amenity codes | property |
| `verificationStatus` | verification states | property |

Each facet is a list of `{ value, count }` sorted by value.

Cancellation classes:

- `NON_REFUNDABLE`: penalty of 100 percent or more.
- `FLEXIBLE`: no penalty and at least 48 hours free.
- `MODERATE`: everything else, including plans with missing cancellation content.

Stats (over all hits):

| Field | Definition |
|---|---|
| `resultCount` | number of hits |
| `pricedCount` | hits with a best total (zero without dates) |
| `instantBookingCount` | hits with an instant offer |
| `offerCount` | sum of offers over all hits |
| `price` | `min`, `max`, `median`, `average` of the best totals; `null` without priced hits. Median of an even count is the floor of the mean of the two middle values. Average is the floor of the integer mean. |
| `distance` | `minKm` and `maxKm` of hit distances; `null` without a search point |

All money in stats is a CDF string with two decimals.

---

## 8. Availability and pricing rules

- A unit type is available for a stay when every night in `[checkIn, checkOut)` has a positive sum of `sellableCapacity - allocatedCount` across its pools. Blocks are stored as allocations, so they reduce this sum.
- A physical-room block that overlaps the stay removes its whole unit type, as the quote does.
- Restriction rules are evaluated with the quote's own logic for the booking day, which is today in the property's timezone. A violated rule removes the unit type.
- Pricing uses the quote's rules: the base override, dated amounts and percentages, the per-guest factor, child nightly amounts, and one automatic promotion. The search total therefore equals the quote total for the same inputs. Promotions are loaded only for the requested dates.
- A plan is dropped when it has no nightly rate for any night of the stay.
- Without dates the engine does not check availability or price. It returns hits with `total: null` and `remainingRooms: null`. Price filters and price sort are refused in that case.

---

## 9. Analytics and the data pipeline

When `analyticsConsent` is true, one row is written to `stay_search_events`:

| Column | Meaning |
|---|---|
| `id` | `sev-` plus a UUID |
| `city` | city code of the database |
| `fingerprint` | the 16-character fingerprint of the canonical query |
| `engine_version` | engine version at the time |
| `query` | the canonical query as JSON (no user identifier, no contact data) |
| `result_count` | total hits |
| `candidate_plans`, `priced_plans` | counts from phases A and B |
| `duration_ms` | total engine time |
| `createdAt` | insert time |

Rules for the pipeline:

- Store nothing that identifies a guest. The engine stores the search point rounded to 3 decimals, which is about 110 metres. Treat it as personal data for retention purposes.
- Events are written only with consent. A missing or false flag writes nothing.
- A failed event write is logged and does not fail the search.
- Retention is six months. The expiry sweep (`reservations/expiry.rs`) runs every 30 seconds and deletes rows older than six months, up to 1000 per run, through `search_engine::events::purge_expired`.
- Child ages are never stored in an event. The stored query has `childAges: []`, and `children` gives the count.

Example analyses (PostgreSQL, run against one city database):

```sql
SELECT query->>'sort' AS sort, COUNT(*) AS searches
FROM stay_search_events GROUP BY 1 ORDER BY 2 DESC;

SELECT COUNT(*) FILTER (WHERE result_count = 0)::float / NULLIF(COUNT(*), 0) AS zero_result_rate
FROM stay_search_events;

SELECT value AS property_type, COUNT(*) AS searches
FROM stay_search_events, jsonb_array_elements_text(query->'propertyTypes') AS value
GROUP BY 1 ORDER BY 2 DESC;

SELECT date_trunc('day', "createdAt") AS day, COUNT(*) AS searches, AVG(duration_ms) AS avg_ms
FROM stay_search_events GROUP BY 1 ORDER BY 1;

SELECT fingerprint, COUNT(*) AS repeats
FROM stay_search_events GROUP BY 1 HAVING COUNT(*) > 1 ORDER BY 2 DESC LIMIT 20;
```

Joins to bookings are done through the dates and the property id inside `query`, never through a guest id.

---

## 10. Performance

What is built:

- Phase A is one statement. It uses the primary keys of night inventory, operators and physical units, the availability block range index, and the new partial and GIN indexes. The per-night aggregation runs only for the candidate units, through a semi-join.
- Every filter is a parameter with a neutral value. Unused filters cost one comparison.
- Phase B issues restriction, rate rule and promotion queries for each candidate batch.
- Phase D fetches cover images only for the returned page.
- Candidate rows are read in batches; `LIMIT` and `OFFSET` select the final property page after pricing and grouping.

Measured on the seeded dev database (small data): `EXPLAIN ANALYZE` of the candidate statement with two nights and two adults ran in about 0.16 ms of execution. The plan showed index scans on the night inventory and block indexes, a hash semi-join on the pools, and a sort of at most a few plans. Re-run the plan at production volume before launch.

Known costs and next steps:

- The city filter on properties is a scan of the city's properties. It is cheap while a city has hundreds of properties. At thousands, add a materialised search projection (decision D-S4).
- Policy content is cast in SQL (`freeUntilHours`, `penaltyPercent`, `collection`). A malformed value raises an error that surfaces as an internal error. Policy content is written only by migrations and seeds today, so the risk is low. Validate it on write when an operator editor exists.
- Facets and stats are SQL aggregates over transaction-local priced offers. Pricing still runs per candidate plan, so production-volume profiling is needed.
- Caching is not built. The fingerprint and `dataAsOf` are the keys a cache layer would use.
- Scale decision (section 13): no materialised projection yet. A projection is built when any city has more than 2,000 published properties, or when the candidate phase (`meta.timingsMs.candidates`) has a 95th percentile above 100 ms over a week of events. Both measurements are already returned or stored, so the trigger can be watched without new instrumentation.

---

## 11. Catalog and use by language models

`GET /api/public/stays/search/catalog` returns the machine-readable contract: every filter with its type, bounds and allowed values; the sorts and their prerequisites; the cancellation classes; paging bounds; limits; the response shape; and the analytics behaviour. A model or pipeline should read the catalog once and build requests from it.

How a model should call the engine:

1. Read the catalog. Do not guess codes.
2. Send dates together. Send a point and a distance together.
3. Use `instantBooking` for "book now" requests and `sort=price` only with dates.
4. Read `total` and `facets` before paging, to explain how many results match and why.
5. Quote the `fingerprint` when reporting a result, so the same answer can be reproduced.
6. Never state a stay price without `total` from an offer; never state availability without dates.

---

## 12. Extending the engine

To add a filter:

1. Add the field to `RawSearch` and `SearchQuery` (`query.rs`), with validation and a canonical form.
2. Add the predicate to the candidate statement (`sql.rs`) with a new parameter number, and update the parameter count in the bind list. Every parameter must be referenced in the statement.
3. If the filter needs prices or policies, apply it in Phase B or C with a unit test.
4. Add the field to the catalog (`catalog.rs`) and to the gateway query mapping (`api-gateway/src/routes/stays/public_discovery.rs`).
5. Add unit tests (`query_tests.rs`) and a DB test (`db_tests.rs`) that proves the filter removes what it should and keeps what it should.
6. Update this document (sections 4, 6 and 11).

To add a sort: add it to `SORTS`, to `rank::sort_hits`, to the catalog, and to this document. Its tie-breakers must end with `id`.

To add a facet: add it to `facets::facets` and to `Facets`, and document its definition in section 7.

Compatibility: adding optional fields and new enum values is compatible. Removing a field, changing a default, or changing a tie-breaker requires a new engine version.

---

## 13. Decisions and gaps

Decisions taken (engine version 2):

- D18 (location privacy): public results carry no coordinates and no address. Distance uses each property's coordinates rounded to two decimals (about 1.1 km), on both sides of the calculation, so the public point is a fixed cell per property. Exact coordinates and the address are released only after a confirmed reservation; that release is not built yet and belongs to spec 09. The owner's note that the earlier approach was "not normal" is addressed by this change; if a different rule is meant, say which.
- D-S1 (ranking): relevance is verified properties first, then instant booking, then the lower best total, then name and id (section 5). Reviews join this order when they exist.
- D-S2 (child ages): required, one age per child, 0 to 17, because spec 08 requires them. Pricing does not use age bands yet: no cut-offs or child prices by age exist, and inventing them is an owner decision. The quote and hold do not take ages yet; they must before a guest can book with ages.
- D-S3 (analytics retention): six months, enforced by the expiry sweep (section 9).
- D-S4 (scale): no projection now; the trigger is in section 10.

Gaps, not built:

- Ratings and review counts: the fields exist and are null. Reviews are spec 12.
- Age bands in pricing and the quote and hold taking child ages (section 13).
- Exact location after a confirmed booking (spec 09; section 13).
- Neighbourhood and a declared property type: property type is derived from the unit model. Neighbourhood is not stored.
- Amenity editing: amenities are seeded. The operator property editor (spec 05 O03) does not exist yet, so nothing writes amenities at runtime.
- Cross-city search: each city database is searched separately (decision D12).
- Share links and alerts for saved searches (spec 08 C05, needs notifications from spec 12).
- A cache layer and a search-result projection.

---

## 14. Test matrix

Unit (no database): `search_engine/query_tests.rs` covers defaults, paired dates, the night limit, price ordering and format, malformed prices, distance rules, the origin rounding, code normalisation and refusal, cancellation limits, the fingerprint, and the cancellation classes.

Database (ignored by default, run with `STAY_TEST_DATABASE_URL` set to a seeded database): `search_engine/db_tests.rs` covers:

- analytics never store child ages, and rows older than six months are purged while recent rows stay;

- a search prices the lakeside property and returns a cover, with stats that agree with the hits;
- instant booking keeps only instant offers;
- meal and property type filters narrow the offers;
- accessibility and amenities must all be present;
- payment and cancellation filters follow the policy content;
- price sort is ordered by total, and price ranges use the stay total, with inverted ranges refused;
- the distance filter keeps a near property and drops a far one;
- a sold-out night removes the unit type;
- pages are counted on the server and beyond-last pages are empty;
- analytics rows are written only with consent.

Each test creates its own inventory for fresh dates, so tests can run in parallel.

Acceptance commands:

```bash
cd tzone-rs && cargo test -p stay-service
cd tzone-rs && STAY_TEST_DATABASE_URL=... cargo test -p stay-service -- --ignored
```

End-to-end check (running stack): the catalog returns 200 and a stocked-window search returns the seeded property with the seeded total. A malformed price returns 400 with a message.
