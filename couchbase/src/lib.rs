#![doc(html_root_url = "https://docs.rs/couchbase/1.0.0-alpha.5")]

mod api;
mod io;

pub use api::buckets::*;
pub use api::collections::*;
pub use api::error::*;
pub use api::options::*;
pub use api::query_indexes::*;
pub use api::results::*;
pub use api::search::*;
pub use api::search_indexes::*;
pub use api::users::*;
pub use api::view_indexes::*;
pub use api::{
    Bucket, Cluster, Collection, DurabilityLevel, LookupInSpec, MutateInSpec, MutationState,
    MutationToken,
};

pub use api::Scope;

#[cfg(feature = "volatile")]
pub use io::request::{GenericManagementRequest, Request};
