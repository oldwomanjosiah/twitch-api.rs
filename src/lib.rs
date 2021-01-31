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

/// Common functions any types used in most application
pub mod prelude {}

mod crate_prelude {
    pub fn as_space_list<'a, L, T>(l: L) -> String
    where
        L: AsRef<[T]>,
        T: AsRef<str>,
    {
        let cap = l.as_ref().iter().fold(0, |a, e| a + e.as_ref().len());
        let mut l = l.as_ref().iter();
        let mut s = String::with_capacity(cap);
        s.push_str(l.next().unwrap().as_ref());
        l.fold(s, |mut s, e| {
            s.push_str("%20");
            s.push_str(e.as_ref());
            s
        })
    }
}
