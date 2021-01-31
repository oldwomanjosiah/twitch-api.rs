//! Requests to do with the `Channels` resource

/// Request to the [`Get Channel Information`] endpoint
///
/// [`Get Channel Information`]: https://dev.twitch.tv/docs/api/reference#get-channel-information
pub mod get_channel_information {

    use crate::crate_prelude::{serde_derive::*, *};
    use crate::values::broadcasters::*;
    use crate::values::games::*;

    #[derive(Debug, Clone, Serialize)]
    /// Request builder for the `Get Channel Information` endpoint
    ///
    /// See module level documentation for usage.
    pub struct GetChannelInformationRequest<A>
    where
        A: AuthToken,
    {
        #[serde(skip)]
        auth: Option<A>,

        broadcaster_id: Option<BroadcasterId>,
    }

    impl<A> Request for GetChannelInformationRequest<A>
    where
        A: AuthToken + Sync,
    {
        const ENDPOINT: &'static str = "https://api.twitch.tv/helix/channels";
        const METHOD: reqwest::Method = reqwest::Method::GET;

        type Headers = A;
        type Parameters = Self;
        type Body = ();

        type Response = GetChannelInformationResponse;
        type ErrorCodes = CommonResponseCodes;

        fn builder() -> Self {
            Self {
                auth: None,
                broadcaster_id: None,
            }
        }

        fn headers(&self) -> &Self::Headers {
            self.auth.as_ref().unwrap()
        }
        fn parameters(&self) -> &Self::Parameters {
            self
        }
        fn body(&self) -> &Self::Body {
            &()
        }

        fn ready(&self) -> Result<(), RequestError<Self::ErrorCodes>> {
            if self.auth.is_none() {
                Err(RequestError::MissingAuth)
            } else if self.broadcaster_id.is_none() {
                Err(RequestError::MalformedRequest(
                    "You must provide a broadcaster_id".into(),
                ))
            } else {
                Ok(())
            }
        }
    }

    impl<A> GetChannelInformationRequest<A>
    where
        A: AuthToken,
    {
        /// Set the authorization token for this request
        pub fn set_auth(&mut self, auth: A) -> &mut Self {
            self.auth.replace(auth);
            self
        }

        /// Set the broadcaster_id for whom you are requesting informaiton
        pub fn set_broadcaster_id<B>(&mut self, broadcaster_id: B) -> &mut Self
        where
            B: Into<BroadcasterId>,
        {
            self.broadcaster_id.replace(broadcaster_id.into());
            self
        }
    }

    impl<A> ParametersExt for GetChannelInformationRequest<A> where A: AuthToken {}

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[allow(missing_docs)]
    /// Represents a response from a sucessful request to the get channel
    /// information endpoint
    pub struct GetChannelInformationResponse {
        #[serde(rename = "data")]
        pub channels: Vec<ChannelInformation>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[allow(missing_docs)]
    /// Represents a single channel object
    pub struct ChannelInformation {
        pub broadcaster_id: BroadcasterId,
        pub broadcaster_name: BroadcasterName,

        /// The name of the game being played on the current stream
        pub game_name: GameName,

        /// The id of the game being played on the current stream
        pub game_id: GameId,
        pub broadcaster_language: BroadcasterLanguage,

        /// The title of the current stream
        pub title: String,
    }
}
