//! Integration tests against the vendored upstream CRDs.
//!
//! Each run installs one channel's CRDs (see [`common::CHANNEL`]) and checks
//! every API version of every resource through the crate's types. Test bodies
//! live in `suite/` as plain Rust. Each one imports its types from `super::api`,
//! which [`channel!`] binds to the API version under test, so one body covers
//! every version that shares its shape, in both channels.
//!
//! `cargo xtask generate` writes the tables in `channels.rs` from the CRDs it
//! vendors: every API version each CRD defines. A version uses
//! `suite/<resource>_<version>.rs` if that exists, else `suite/<resource>.rs`,
//! which generation writes as a failing `todo!` stub for a new resource.
//! `tests/schema.rs` reads the same tables with its own `channel!`.

// Including one body for several API versions is the point.
#![allow(clippy::duplicate_mod)]

mod common;

/// Declares a module per channel, resource and API version, e.g.
/// `standard::httproutes::v1beta1`, that binds `api` to
/// `gateway_api::apis::standard::httproutes::v1beta1` and includes the given
/// test body from `suite/`. `$root` is for `tests/schema.rs`; it's unused here.
macro_rules! channel {
    ($channel:ident { $($resource:ident: $root:ident { $($version:ident => $body:literal),+ $(,)? })+ }) => {
        // The channel module points at `suite/`; the nested modules keep that
        // directory (`.`), so each body path resolves from `suite/`.
        #[path = "suite"]
        mod $channel {
            $(
                #[path = "."]
                mod $resource {
                    $(
                        #[path = "."]
                        mod $version {
                            use gateway_api::apis::$channel::$resource::$version as api;

                            #[path = $body]
                            mod suite;
                        }
                    )+
                }
            )+
        }
    };
}

include!("channels.rs");
