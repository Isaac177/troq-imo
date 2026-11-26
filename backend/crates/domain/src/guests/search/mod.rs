mod aggregates;
mod candidates;
pub mod catalog;
mod engine;
mod events;
pub mod facets;
mod normalize;
pub(crate) mod params;
mod pricing;
mod query;
mod rank;
mod raw;
pub mod response;
mod stage;

#[cfg(test)]
mod query_tests;

pub use catalog::{amenity_codes, catalog as search_catalog, category_codes};
pub use engine::run as run_search;
pub use events::{anonymised as anonymised_query, record as record_search_event, EventInput};
pub use facets::{DistanceStats, FacetValue, Facets, PriceStats, Stats};
pub use normalize::cancellation_class;
pub use params::SearchParams;
pub use query::{Origin, SearchQuery, Window, ENGINE_VERSION};
pub use rank::string_list;
pub use rank::{CancellationView, CoverView, OfferView, SearchHit};
pub use raw::RawSearch;
pub use response::{Meta, SearchResponse, Timings};
