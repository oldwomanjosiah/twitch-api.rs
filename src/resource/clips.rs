//! Endpoints to do with the Clips resource

use serde::{Deserialize, Serialize};

use crate::values::*;
use broadcasters::*;
use clips::*;
use games::GameId;
use users::*;
use videos::VideoId;

#[derive(Debug, Serialize, Deserialize)]
#[allow(missing_docs)]
/// Information relating to a single clip resource
pub struct ClipInfo {
    pub broadcaster_id: BroadcasterId,
    pub broadcaster_name: BroadcasterName,
    pub created_at: RFC3339Time,
    pub creator_id: UserId,
    pub creator_name: UserName,
    pub embed_url: Url,
    pub game_id: GameId,
    #[serde(rename = "id")]
    pub clip_id: ClipId,
    pub language: ISOLanguage,
    pub thumbnail_url: Url,
    pub title: ClipTitle,
    pub url: Url,
    pub video_id: VideoId,
    pub view_count: ViewCount,
}

/// Request to the [`Get Clips`] endpoint
///
/// [`Get Clips`]: https://dev.twitch.tv/docs/api/reference#get-clips
///
/// ### Example
///
/// ```ignore
/// # use twitch_api_rs::resource::clips::ClipInfo;
/// # use twitch_api_rs::resource::clips::get_clips::*;
/// # use twitch_api_rs::requests::{RequestError, RequestError, FailureStatus, CommonResponseCodes};
/// #
/// let resp = match GetClipsRequest::builder()
///     .set_auth(auth_token)
///     .set_broadcaster_id(broadaster_id)
///     .set_count(50)
///     .make_request(&client)
///     .await {
///         Ok(resp) => resp,
///         Err(RequestError::KnownErrorStatus(s)) => { /* ... */ }
///         Err(e) => { /* ... */ }
///     };
///
/// for clip in resp.clips {
///     eprintln!("Clip found with name: {}", clip.title);
/// }
/// ```
pub mod get_clips {
    use super::ClipInfo;
    use super::*;
    use crate::auth::AuthToken;
    use crate::requests::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug)]
    enum PaginationDirection {
        None,
        Before(Pagination),
        After(Pagination),
    }

    impl Default for PaginationDirection {
        fn default() -> Self {
            Self::None
        }
    }

    use serde::ser::SerializeMap;
    use serde::Serializer;

    #[derive(Debug)]
    enum QueryType {
        Unset,
        BroadCasterId(BroadcasterId),
        GameId(GameId),
        ClipId(Vec<ClipId>),
    }

    impl Default for QueryType {
        fn default() -> Self {
            Self::Unset
        }
    }

    #[derive(Debug)]
    /// Request builder for the [`Get Clips`] endpoint
    ///
    /// [`Get Clips`]: https://dev.twitch.tv/docs/api/reference#get-clips
    pub struct GetClipsRequest<A>
    where
        A: AuthToken,
    {
        auth: Option<A>,
        query_type: QueryType,
        pagination: PaginationDirection,
        count: Option<Count>,
        period: Option<(StartedAt, Option<EndedAt>)>,
    }

    impl<A> Serialize for GetClipsRequest<A>
    where
        A: AuthToken,
    {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut map = s.serialize_map(None)?;

            // Required query params
            match &self.query_type {
                QueryType::Unset => unreachable!("Cannot serialize without a query type"),
                QueryType::GameId(id) => map.serialize_entry("game_id", &id)?,
                QueryType::BroadCasterId(id) => map.serialize_entry("broadcaster_id", &id)?,
                QueryType::ClipId(ids) => {
                    for id in ids {
                        map.serialize_entry("id", &id)?;
                    }
                }
            }

            // Optional params
            match &self.pagination {
                PaginationDirection::None => (),
                PaginationDirection::Before(pag) => {
                    if pag.cursor.is_some() {
                        map.serialize_entry("before", &pag.cursor)?
                    }
                }
                PaginationDirection::After(pag) => {
                    if pag.cursor.is_some() {
                        map.serialize_entry("after", &pag.cursor)?
                    }
                }
            }

            if let Some(count) = &self.count {
                map.serialize_entry("first", count)?;
            }

            if let Some((start, maybe_end)) = &self.period {
                map.serialize_entry("started_ad", start)?;
                if let Some(end) = maybe_end.as_ref() {
                    map.serialize_entry("ended_at", end)?;
                }
            }

            map.end()
        }
    }

    impl<A> ParametersExt for GetClipsRequest<A> where A: AuthToken {}

    impl<A> Request for GetClipsRequest<A>
    where
        A: AuthToken + Sync,
    {
        const ENDPOINT: &'static str = "https://api.twitch.tv/helix/clips";
        const METHOD: reqwest::Method = reqwest::Method::GET;

        type Headers = A;
        type Parameters = Self;
        type Body = ();

        type Response = GetClipsResponse;
        type ErrorCodes = CommonResponseCodes;

        fn builder() -> Self {
            Self {
                auth: None,
                query_type: QueryType::default(),
                pagination: PaginationDirection::default(),
                count: None,
                period: None,
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
            } else if let &QueryType::Unset = &self.query_type {
                Err(RequestError::MalformedRequest(
                    "Must provide at least one of broadcaster_id, game_id, clip_id".into(),
                ))
            } else if let QueryType::ClipId(clips) = &self.query_type {
                if clips.len() == 0 {
                    Err(RequestError::MalformedRequest(
                        "Must provide at least one of broadcaster_id, game_id, clip_id".into(),
                    ))
                } else if clips.len() > 100 {
                    Err(RequestError::MalformedRequest(
                        "Cannot send more than 100 clip_ids at a time".into(),
                    ))
                } else {
                    Ok(())
                }
            } else {
                Ok(())
            }
        }
    }

    impl<A> GetClipsRequest<A>
    where
        A: AuthToken,
    {
        /// Set the authorization token to be used with this request
        pub fn set_auth(&mut self, auth: A) -> &mut Self {
            self.auth.replace(auth);
            self
        }

        /// Set the broadcaster_id request
        ///
        /// Will replace current query type if already called `set_game_id` or `add_clip_id`
        pub fn set_broadcaster_id<S: Into<BroadcasterId>>(&mut self, id: S) -> &mut Self {
            self.query_type = QueryType::BroadCasterId(id.into());
            self
        }

        /// Set the game_id request
        ///
        /// Will replace current query type if already called `set_broadcaster_id` or `add_clip_id`
        pub fn set_game_id<S: Into<GameId>>(&mut self, id: S) -> &mut Self {
            self.query_type = QueryType::GameId(id.into());
            self
        }

        /// Add a clip_id to search for
        ///
        /// Will replace current query type if already called `set_broadcaster_id` or `set_game_id`
        pub fn add_clip_id<S: Into<ClipId>>(&mut self, id: S) -> &mut Self {
            if let QueryType::ClipId(clips) = &mut self.query_type {
                clips.push(id.into());
            } else {
                self.query_type = QueryType::ClipId(vec![id.into()]);
            }
            self
        }

        /// Replace the currect query with a specific set of game_ids
        ///
        /// Will replace current query type if already called `set_broadcaster_id` or `set_game_id`
        pub fn set_clip_ids<S>(&mut self, set: Vec<S>) -> &mut Self
        where
            S: Into<ClipId>,
        {
            self.query_type = QueryType::ClipId(set.into_iter().map(Into::into).collect());
            self
        }

        /// Clear the current query if it is of the type clip_id
        pub fn clear_clip_ids(&mut self) -> &mut Self {
            if let QueryType::ClipId(ids) = &mut self.query_type {
                ids.clear();
            }
            self
        }

        /// Sets the max amount of items to be returned from this request
        ///
        /// Without being set this value is 20
        pub fn set_count<C: Into<Count>>(&mut self, count: C) -> &mut Self {
            self.count.replace(count.into());
            self
        }

        /// Resets the amount of items to be returned from this request to its default of 20
        pub fn reset_count(&mut self) -> &mut Self {
            self.count.take();
            self
        }

        /// Set a time window filter, times are [`RFC3339`]
        ///
        /// [`RFC3339`]: https://datatracker.ietf.org/doc/rfc3339
        pub fn set_period<S, T>(&mut self, started_at: S, ended_at: T) -> &mut Self
        where
            S: Into<StartedAt>,
            T: Into<EndedAt>,
        {
            self.period = Some((started_at.into(), Some(ended_at.into())));
            self
        }

        /// Set the start of the date/time window filter, if set_ended_at not called then the
        /// window ends a week from this value
        pub fn set_started_at<S>(&mut self, started_at: S) -> &mut Self
        where
            S: Into<StartedAt>,
        {
            if let Some((start, _)) = &mut self.period {
                *start = started_at.into();
            } else {
                self.period = Some((started_at.into(), None));
            }
            self
        }

        /// Set the end of the date/time window filter, if `set_started_at` not called before this
        /// then it does nothing as and end may not be set without a start
        pub fn set_ended_at<S: Into<EndedAt>>(&mut self, ended_at: S) -> &mut Self {
            if let Some((_, end)) = &mut self.period {
                end.replace(ended_at.into());
            }
            self
        }

        /// Set the backwards pagination cursor for this request, use with Pagination
        /// from previous response
        pub fn before<P: Into<Pagination>>(&mut self, before: P) -> &mut Self {
            self.pagination = PaginationDirection::Before(before.into());
            self
        }

        /// Set the forwards pagination cursor for this request, use with Pagination
        /// from previous response
        pub fn after<P: Into<Pagination>>(&mut self, after: P) -> &mut Self {
            self.pagination = PaginationDirection::After(after.into());
            self
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[allow(missing_docs)]
    /// Response container from the Get Clips endpoint
    pub struct GetClipsResponse {
        #[serde(rename = "data")]
        #[serde(default)]
        pub clips: Vec<ClipInfo>,
        /// The inner may be empty, indicating that there was not more than the value passed to [`GetClipsRequest::set_count`]
        pub pagination: Pagination,
    }
}
