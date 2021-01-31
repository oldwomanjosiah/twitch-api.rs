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
pub mod resource;
pub mod values;

/// Common functions and types used in most application
pub mod prelude {
    /// Trait used by many endpoints for authentication and scopes
    pub use crate::auth::AuthToken;

    /// Trait that exposes methods common to all requests, required to use
    /// `.make_request(&client).await`
    pub use crate::requests::Request;

    /// Types produced and consumed by endpoints
    pub use crate::values;
}

mod crate_prelude {
    pub use crate::auth::{self, AuthToken};
    pub use crate::requests::*;
    pub use reqwest::Method;

    pub mod serde_derive {
        pub use serde::{Deserialize, Serialize};
    }
    pub mod serde_impl {
        pub use serde::{ser::SerializeMap, Deserialize, Serialize, Serializer};
    }
}
