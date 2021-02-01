//! Requests to do with the 'Users' resource

/// Request to the [`Get Users`] endpoint
///
/// # Example
///
/// ```ignore
/// # use crate::requests::{Request, RequestError};
/// let response = match GetUsersRequest::builder()
///     .set_auth(client_auth_token)
///     .add_login("TheHoodlum12")
///     .make_request(&client)
///     .await {
///         Ok(resp) => resp,
///         Err(RequestError::MalformedRequest(msg)) => unreachable!("We provided an auth token and at least one request, but it failed with {}", msg),
///         // ...
///     };
///
/// for user in response.users.iter() {
///     eprintln!("Found user with display name: {}", user.display_name);
/// }
/// ```
///
/// [`Get Users`]: https://dev.twitch.tv/docs/api/reference#get-users
pub mod get_users {
    use crate::auth::AuthToken;
    use crate::requests::*;
    use serde::{
        ser::{SerializeMap, Serializer},
        Deserialize, Serialize,
    };

    use crate::values::users::*;

    /// Request to the [`Get Users`] endpoint
    ///
    /// [`Get Users`]: https://dev.twitch.tv/docs/api/reference#get-users
    ///
    /// See module level documentation for usage
    #[derive(Debug)]
    pub struct GetUsersRequest<A>
    where
        A: AuthToken,
    {
        auth: Option<A>,
        id: Vec<UserId>,
        login: Vec<UserName>,
    }

    impl<A> Request for GetUsersRequest<A>
    where
        A: AuthToken + Send,
    {
        const ENDPOINT: &'static str = "https://api.twitch.tv/helix/users";
        const METHOD: reqwest::Method = reqwest::Method::GET;

        type Headers = A;
        type Parameters = Self;
        type Body = ();

        type Response = GetUsersResponse;

        type ErrorCodes = CommonResponseCodes;

        fn builder() -> Self {
            Self {
                auth: None,
                id: vec![],
                login: vec![],
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
            } else if self.id.len() == 0 && self.login.len() == 0 {
                Err(RequestError::MalformedRequest(String::from(
                    "At least one id or login must be provided",
                )))
            } else if self.id.len() + self.login.len() > 100 {
                Err(RequestError::MalformedRequest(String::from(
                    "You cannot make a get users request with more than 100 cumulative search terms",
                )))
            } else {
                Ok(())
            }
        }
    }

    impl<A: AuthToken> GetUsersRequest<A> {
        /// Set the authorization token to use
        ///
        /// Consumes the token, so should be cloned if you plan to re-use TODO offer reference
        /// counted storage for fewer allocations.
        pub fn set_auth(&mut self, auth: A) -> &mut Self {
            self.auth.replace(auth);
            self
        }

        /// Add the id to the set of ids to be sent. May not have more than 100 ids and logins
        pub fn add_id<S: Into<UserId>>(&mut self, id: S) -> &mut Self {
            self.id.push(id.into());
            self
        }

        /// Replace the set of ids to be sent
        ///
        /// ```
        /// # use twitch_api_rs::resource::users::get_users;
        /// # use twitch_api_rs::requests::Request;
        /// # use twitch_api_rs::auth::client_credentials::ClientAuthToken;
        /// # type GetUsersRequest = get_users::GetUsersRequest<ClientAuthToken>;
        /// let mut req = GetUsersRequest::builder();
        /// req.set_ids(vec!["477906794"]);
        /// // ...
        /// ```
        pub fn set_ids<C, S>(&mut self, ids: C) -> &mut Self
        where
            C: IntoIterator<Item = S>,
            S: Into<UserId>,
        {
            self.id = ids.into_iter().map(Into::into).collect();
            self
        }

        /// Clear the set of ids to be sent
        pub fn clear_ids(&mut self) -> &mut Self {
            self.id.clear();
            self
        }

        /// Add the login to the set of logins to be sent. May not have more than 100 ids and logins
        pub fn add_login<S: Into<UserName>>(&mut self, login: S) -> &mut Self {
            self.login.push(login.into());
            self
        }

        /// Clear the set of ids to be sent
        pub fn clear_logins(&mut self) -> &mut Self {
            self.login.clear();
            self
        }

        /// Replace the set of logins to be sent
        ///
        /// ```
        /// # use twitch_api_rs::resource::users::get_users;
        /// # use twitch_api_rs::requests::Request;
        /// # use twitch_api_rs::auth::client_credentials::ClientAuthToken;
        /// # type GetUsersRequest = get_users::GetUsersRequest<ClientAuthToken>;
        /// let mut req = GetUsersRequest::builder();
        /// req.set_ids(vec!["TheHoodlum12", "Denims"]);
        /// // ...
        /// ```
        pub fn set_logins<C, S>(&mut self, logins: C) -> &mut Self
        where
            C: IntoIterator<Item = S>,
            S: Into<UserName>,
        {
            self.login = logins.into_iter().map(Into::into).collect();
            self
        }
    }

    #[doc(hidden)]
    impl<A> Serialize for GetUsersRequest<A>
    where
        A: AuthToken,
    {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut map = s.serialize_map(Some(2))?;
            self.id
                .iter()
                .try_for_each(|e| map.serialize_entry("id", e))?;
            self.login
                .iter()
                .try_for_each(|e| map.serialize_entry("login", e))?;
            map.end()
        }
    }

    #[doc(hidden)]
    impl<A: AuthToken> ParametersExt for GetUsersRequest<A> {}

    /// A Collection of response items returned by [`GetUsersRequest`]
    #[derive(Debug, Serialize, Deserialize)]
    #[allow(missing_docs)]
    pub struct GetUsersResponse {
        #[serde(rename = "data")]
        pub users: Vec<UserDescription>,
    }

    use crate::values::broadcasters::*;
    use crate::values::users::UserLogin;
    use crate::values::{RFC3339Time, Url};

    #[derive(Debug, Serialize, Deserialize)]
    #[allow(missing_docs)]
    /// A single user datum returned by [`GetUsersRequest`]
    pub struct UserDescription {
        pub broadcaster_type: BroadcasterType,
        pub description: String,
        pub display_name: UserName,

        /// The email of the user, will only be returned if the access token provided
        /// has the [`scope`] 'user:read:email'
        ///
        /// [`scope`]: https://dev.twitch.tv/docs/authentication#scopes
        pub email: Option<UserEmail>,
        pub id: UserId,
        pub login: UserLogin,
        pub offline_image_url: Url,
        pub profile_image_url: Url,

        #[serde(rename = "type")]
        user_type: UserType,
        view_count: BroadcasterViews,
        created_at: RFC3339Time,
    }
}
