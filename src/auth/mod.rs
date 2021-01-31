//! Types and functions for auth flows
//!
//! See [`Twitch Auth Documentation`]
//!
//! [`Twitch Auth Documentation`]: https://dev.twitch.tv/docs/authentication

pub mod scopes;

/// Represents a authorization token of some type that can be sent as a header to a
/// twitch endpoint.
pub trait AuthToken: crate::requests::Headers {
    /// Get the set of scopes that this token has associated with it
    fn scopes(&self) -> &scopes::ScopeSet;
}

/// [`Implicit Code`] Flow
///
/// [`Implicit Code`]: https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#oauth-implicit-code-flow
pub mod implicit_code {}

/// [`Authorization Code`] Flow
///
/// [`Authorization Code`]: https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#oauth-implicit-code-flow
pub mod authorization_code {}

/// [`Client Credentials`] Flow
///
/// [`Client Credentials`]: https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#oauth-client-credentials-flow
///
/// ```ignore
/// # use twitch_api_rs::requests::*;
/// # let (client_id, client_secret) = (
/// #     String::from("uo6dggojyb8d6soh92zknwmi5ej1q2"),
/// #     String::from("nyo51xcdrerl8z9m56w9w6wg"),
/// # );
///
/// let client_resp = ClientAuthRequestBuilder::builder()
///     .set_client_id(client_id)
///     .set_client_secret(client_secret)
///     .make_request()
///     .await;
///
/// match client_resp {
///
///     Ok(resp) => {
///         let (token, expiration) = resp.into();
///         eprintln!("Got Token {}. (Expires in {} seconds)", token, expiration);
///     }
///
///     Err(RequestError::MalformedRequest(msg)) =>
///         unreachable!("We set all the parameters but the struct said {}", msg),
///
///     Err(RequestError::ErrorCodes(code)) =>
///         eprintln!("Server rejected request for reason {}", code),
///
///     Err(e) =>
///         eprintln!("Failed to make request for reason {}", e),
///
/// }
/// ```
pub mod client_credentials {

    use crate::requests::*; // TODO: Replace with internal prelude
    use reqwest::RequestBuilder;
    use serde::{ser::SerializeMap, Deserialize, Serialize, Serializer};

    #[derive(Debug)]
    #[doc(hidden)]
    /// Do not use directly, instead use [`ClientAuthRequest`]
    pub struct ClientAuthRequestParams {
        client_id: Option<String>,
        client_secret: Option<String>,
        scopes: Vec<String>, // TODO change to list of Scope Enum items or maybe bitset that has display trait and named bits
    }

    impl ParametersExt for ClientAuthRequestParams {}

    #[derive(Debug)]
    /// Request for the [`client authentication`] flow.  
    /// See module level documentation for usage.
    ///
    /// implemnts [`Request`], see documentation for more information.
    ///
    /// [`client authentication`]: https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#oauth-client-credentials-flow
    pub struct ClientAuthRequest {
        params: ClientAuthRequestParams,
    }

    impl Serialize for ClientAuthRequestParams {
        fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut map = ser.serialize_map(Some(if self.scopes.len() > 0 { 4 } else { 3 }))?;
            map.serialize_entry("client_id", self.client_id.as_ref().unwrap())?;
            map.serialize_entry("client_secret", self.client_secret.as_ref().unwrap())?;
            map.serialize_entry("grant_type", "client_credentials")?;

            // TODO Serialize vec as space separated list

            map.end()
        }
    }

    #[cfg_attr(feature = "nightly", doc(spotlight))]
    impl Request for ClientAuthRequest {
        const ENDPOINT: &'static str = "https://id.twitch.tv/oauth2/token";

        type Headers = ();
        type Parameters = ClientAuthRequestParams;
        type Body = ();

        type Response = ClientAuthResponse;

        type ErrorCodes = CommonResponseCodes;

        const METHOD: reqwest::Method = reqwest::Method::POST;

        fn builder() -> Self {
            Self {
                params: ClientAuthRequestParams {
                    client_id: None,
                    client_secret: None,
                    scopes: Vec::new(),
                },
            }
        }

        fn headers(&self) -> &Self::Headers {
            &()
        }
        fn parameters(&self) -> &Self::Parameters {
            &self.params
        }
        fn body(&self) -> &Self::Body {
            &()
        }

        fn ready(&self) -> Result<(), RequestError<Self::ErrorCodes>> {
            if self.params.client_id.is_none() {
                Err(RequestError::MalformedRequest(String::from(
                    "field client_id must be set",
                )))
            } else if self.params.client_secret.is_none() {
                Err(RequestError::MalformedRequest(String::from(
                    "field client_secret must be set",
                )))
            } else {
                Ok(())
            }
        }
    }

    impl ClientAuthRequest {
        /// Set the client_id
        pub fn set_client_id(&mut self, client_id: String) -> &mut Self {
            self.params.client_id.replace(client_id);
            self
        }

        /// Set the client_secret
        pub fn set_client_secret(&mut self, client_secret: String) -> &mut Self {
            self.params.client_secret.replace(client_secret);
            self
        }
    }

    /// Build a complete request from `(client_id, client_secret)`
    impl From<(String, String)> for ClientAuthRequest {
        fn from((client_id, client_secret): (String, String)) -> Self {
            Self {
                params: ClientAuthRequestParams {
                    client_id: Some(client_id),
                    client_secret: Some(client_secret),
                    scopes: vec![],
                },
            }
        }
    }

    #[derive(Debug, Deserialize)]
    /// Response from a successful [`ClientAuthRequest`]
    ///
    /// See module level docuemntation to see how to get
    pub struct ClientAuthResponse {
        /// The access_token returned by twitch
        pub access_token: String,
        // refresh_token:
        /// The amount of seconds until the token expires
        pub expires_in: u32,
        // token_type: String // Always bearer
    }

    impl Into<(String, u32)> for ClientAuthResponse {
        fn into(self) -> (String, u32) {
            (self.access_token, self.expires_in)
        }
    }

    use super::scopes::ScopeSet;

    /// Represents an authorization token header for requests
    #[derive(Debug)]
    #[allow(missing_docs)]
    pub struct ClientAuthToken {
        scopes: ScopeSet,
        pub token: String,
        pub client_id: String,
    }

    impl ClientAuthToken {
        /// Create the auth token from a sucessful auth response and a client_id
        pub fn from_client(auth_response: ClientAuthResponse, client_id: String) -> Self {
            Self {
                // Fill with empty scopes item as scopes only apply to OAuth tokens
                scopes: ScopeSet::new(),
                token: auth_response.access_token,
                client_id,
            }
        }
    }

    impl Headers for ClientAuthToken {
        fn write_headers(&self, req: RequestBuilder) -> RequestBuilder {
            req.header("Authorization", format!("Bearer {}", self.token))
                .header("Client-Id", &self.client_id)
        }
    }

    impl super::AuthToken for ClientAuthToken {
        fn scopes(&self) -> &ScopeSet {
            &self.scopes
        }
    }
}
