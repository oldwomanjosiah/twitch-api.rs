// {{{ Crate level doc
//! [![Crates.io](https://img.shields.io/crates/v/twitch-api-rs?style=plastic)](https://crates.io/crates/twitch-api-rs)
//! [![Gihub Actions Testing CI](https://github.com/oldwomanjosiah/twitch-api.rs/workflows/Rust/badge.svg)](https://github.com/oldwomanjosiah/twitch-api.rs/actions?query=workflow%3ARust)
//!
//! A twitch crate used to build and send requests to
//! the twitch helix api.
//!
//! See [`Twitch API Reference`]
//!
//! [`Twitch API Reference`]: https://dev.twitch.tv/docs/
//!
//! ### Example: Getting the top 20 clips from a creator
//!
//! To get a list of clips from the twitch api, by a streamers dislplay name, there
//! are 4 steps.
//!
//! 1) Get an application ClientId and ClientSecret [^getting_credentials]
//! 2) Request a client_flow Authentication Token
//! 3) Use that token to request information on a user by their display name
//! 4) Use the UserId returned by that request to request a list of clips associated
//!     with their channel.
//!
//! ```
//! # use std::env;
//! # use std::sync::Arc;
//! #[tokio::main]
//! async fn main() {
//!     use twitch_api_rs::prelude::*;
//!     use twitch_api_rs::auth::{ClientId, ClientSecret};
//!
//!     // Here we use the values that you should have gotten from step one
//!
//!     let client_id: ClientId =
//!         env::var("TWITCH_API_RS_TEST_CLIENT_ID")
//!             .expect("Client Id environment variable not defined")
//!             .into();
//!
//!     let client_secret: ClientSecret =
//!         env::var("TWITCH_API_RS_TEST_CLIENT_SECRET")
//!             .expect("Client Secret environment variable not defined")
//!             .into();
//!
//!     // Make a reqwest client to send the requests, and wrap it in an arc so it
//!     // can be reused in multiple futures
//!     let client = Arc::new(reqwest::Client::new());
//!
//!     use twitch_api_rs::auth::client_credentials::{
//!         ClientAuthRequest, ClientAuthResponse, ClientAuthToken,
//!     };
//!
//!     // Step 2
//!
//!     // Get a client credentials (application) access token and wrap it in an arc
//!     // to be used across multiple tasks without repeating
//!     let auth_token: Arc<ClientAuthToken> = Arc::new(
//!         match ClientAuthRequest::builder()
//!             .set_client_id(client_id.clone())
//!             .set_client_secret(client_secret)
//!             .make_request(client.clone())
//!             .await {
//!                 Ok(resp) => {
//!                     // Create the token from the token value provided by twitch and
//!                     // your client_id
//!                     ClientAuthToken::from_client(resp, client_id)
//!                 }
//!
//!                 // Better error handling can be performed here by matching against
//!                 // the type of this requests::RequestError. Elided for conciseness
//!                 Err(e) => panic!("Could not complete auth request for reason {}", e),
//!             }
//!     );
//!
//!     // Step 3
//!
//!     use twitch_api_rs::values::users::UserId;
//!     use twitch_api_rs::resource::users::get_users::*;
//!
//!     // Notice that we get a Vec<UserId> here, as this endpoint allows you to query
//!     // for multiple users at once
//!     let user_ids: Vec<UserId> =
//!         match GetUsersRequest::builder()
//!             .set_auth(auth_token.clone())
//!             .add_login("TheHoodlum12")
//!             .make_request(client.clone())
//!             .await {
//!                 Ok(mut resp) => {
//!                     resp.users.into_iter().map(|i| i.id).collect()
//!                 }
//!
//!                 Err(e) =>
//!                     panic!("Could not complete request for user by display name for reason {}", e),
//!             };
//!
//!     // Step 4
//!
//!     use twitch_api_rs::resource::clips::ClipInfo;
//!     use twitch_api_rs::resource::clips::get_clips::*;
//!     use twitch_api_rs::values::clips::ClipTitle;
//!
//!     for user_id in user_ids {
//!         let auth_token = auth_token.clone();
//!         let client = client.clone();
//!
//!         tokio::spawn(async move {
//!             match GetClipsRequest::builder()
//!                 .set_auth(auth_token)
//!                 .set_broadcaster_id(user_id.clone())
//!                 .make_request(client)
//!                 .await {
//!                     Ok(resp) => {
//!                         // Print out the response
//!                         for clip in resp.clips {
//!                             eprintln!(
//!                                 "Found clip for broadcaster {:?} titled {:?}",
//!                                 &user_id, clip.title
//!                             );
//!                         }
//!                     }
//!                     Err(e) =>
//!                         panic!("Could not get clips for user {:?} for reason {}", user_id, e),
//!                 }
//!         });
//!     }
//! }
//! ```
//!
//! [^getting_credentials]: If you do not already have these then you will need to
//!     log into your [`twitch developer console`], create and name a new application, and copy down
//!     the associated values.  
//!     The client secret that you get from this process is imediately invalidated if you request a
//!     new one, and it will not be shown to you once you leave the page where you generate it, so
//!     make sure to write it down somewhere safe. It is recommended that you treat it like a
//!     password.
//!
//! [`twitch developer console`]: https://dev.twitch.tv/console
// }}}
#![deny(missing_docs, missing_debug_implementations)]
#![cfg_attr(feature = "nightly", feature(doc_spotlight))]

pub mod auth;
mod client;
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
