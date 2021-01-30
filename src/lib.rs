//! A twitch crate used to build and send (with the feature `reqwest`) requests to
//! the twitch helix api.
//!
//! See [`Twitch API Reference`]
//!
//! [`Twitch API Reference`]: https://dev.twitch.tv/docs/
#![deny(missing_docs, missing_debug_implementations)]
#![cfg_attr(feature = "nightly", feature(doc_spotlight))]

pub mod auth;
pub mod requests;

/// Common functions any types used in most application
pub mod prelude {}
