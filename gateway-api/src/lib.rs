#![deny(unsafe_code)]
#![warn(clippy::all, clippy::pedantic, missing_docs, rust_2018_idioms, unreachable_pub)]
#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    // generated API types trigger these
    clippy::struct_excessive_bools,
    clippy::too_many_lines,
)]

//! Rust bindings for the Kubernetes Gateway API.

pub mod duration;
pub use duration::Duration;
/// Auto-generated Gateway API type bindings.
#[allow(missing_docs, unreachable_pub, clippy::all, clippy::pedantic, rust_2018_idioms)]
pub mod apis;
#[cfg(feature = "experimental")]
pub use apis::experimental;
pub use apis::standard::*;
