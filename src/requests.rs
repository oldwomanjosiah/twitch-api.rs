//! Common traits used to construct requestable types
//!
//! - TODO: Make `make_request` function based on feature and requests constructable without
//!     async or sending in all cases

use async_trait::async_trait;
use reqwest::Client;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use thiserror::Error;

/// Returned in the case that a query parameter or the request body was missing or malformed
pub const BAD_REQUEST_CODE: u16 = 400;

/// Returned in the case that the authentication was invalid for some reason
pub const AUTH_ERROR: u16 = 401;

/// Returned in the case of an upstream error, try again once and then assume
/// temporary problems on server side.
pub const SERVER_ERROR: u16 = 500;

/// Used in place of [`Headers`], [`Parameters`] or [`Body`] to inidicate for the
/// respective type that there is none
type None = ();

impl Headers for None {
    fn write_headers(&self, req: RequestBuilder) -> RequestBuilder {
        req
    }
}

impl Parameters for None {
    fn write_parameters(&self, req: RequestBuilder) -> RequestBuilder {
        req
    }
}

impl Body for None {
    fn write_body(&self, req: RequestBuilder) -> RequestBuilder {
        req
    }
}

use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Represents a sucessful request that was denied by the twitch api for some reason.
/// Use request's associated [`ErrorCodes`] to get matchable value.
pub struct FailureStatus {
    status: u16,
    message: String,
}

impl FailureStatus {
    fn into_status<S: ErrorCodes>(self) -> S {
        S::from_status(self)
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
/// Represents a possible response from the twitch api, deserialized from a sucessful
/// request. May not contain the requested content but instead a [`FailureStatus`]
pub enum PossibleResponse<R>
where
    R: Response + 'static,
{
    #[serde(bound(deserialize = "R: DeserializeOwned"))]
    /// Sucessful response
    Response(R),

    /// Response that was denied by the twitch service
    Failure(FailureStatus),
}

impl<R> PossibleResponse<R>
where
    R: Response + 'static,
{
    fn into_result(self) -> Result<R, FailureStatus> {
        match self {
            Self::Response(r) => Ok(r),
            Self::Failure(f) => Err(f),
        }
    }
}

#[derive(Debug, Error)]
/// Returned from a request when it could not be completed
pub enum RequestError<C: ErrorCodes + 'static> {
    #[error("Request Malformed with message: {0}")]
    /// Could not try to make request because it was malformed in some way
    MalformedRequest(String),

    #[error("Response code was an error: {0}")]
    /// Server responded with some error code
    ErrorCodes(#[from] C),

    #[error("Reqwest encountered an error: {0}")]
    /// Reqwest could not complete the request for some reason
    ReqwestError(#[from] reqwest::Error),

    #[error("Unknown Error encountered {0:?}")]
    /// Unknown error
    UnknownError(#[from] Box<dyn std::error::Error>),
}

/// Error codes that can be used in [`ReqwestError::ErrorCodes`], can be built
/// from a [`reqwest::StatusCode`].
pub trait ErrorCodes: std::error::Error + Sized {
    /// Should return `Ok(())` this type does not catch the status code,
    /// otherwise return `Err(Self)` where self contains information on the failure
    fn from_status(codes: FailureStatus) -> Self;
}

#[derive(Debug, Error)]
/// Error codes used by twitch that are the same across most endpoints.
pub enum CommonResponseCodes {
    #[error("400: Malformed Request")]
    /// The request did not conform to what the endpoint was expecting
    BadRequestCode,

    #[error("401: Authorization Error")]
    /// The authorization provided was not valid or was out of date
    AuthErrorCode,

    #[error("500: Server Error")]
    /// Twitch may be experiencing internal errors, if encountered the request should
    /// be retried once. If that fails then assume twitch is temporarily down.
    ServerErrorCode,

    #[error("{0}: Other error with message {1}")]
    /// Other / unexpected error
    Other(u16, String),
}

impl ErrorCodes for CommonResponseCodes {
    fn from_status(codes: FailureStatus) -> Self {
        match codes.status {
            AUTH_ERROR => Self::AuthErrorCode,
            BAD_REQUEST_CODE => Self::BadRequestCode,
            SERVER_ERROR => Self::ServerErrorCode,
            c => Self::Other(c, codes.message),
        }
    }
}

/// Headers for a request
pub trait Headers {
    /// Write headers to request builder and return request builder
    fn write_headers(&self, req: RequestBuilder) -> RequestBuilder;
}

/// Parameters for a request
pub trait Parameters {
    /// Write parameters to request builder and return request builder
    fn write_parameters(&self, req: RequestBuilder) -> RequestBuilder;
}

/// Body for a request
pub trait Body {
    /// Write body to request builder and return request builder
    fn write_body(&self, req: RequestBuilder) -> RequestBuilder;
}

/// Represents a request that can be made to the twitch api
#[async_trait]
#[cfg_attr(feature = "nightly", doc(spotlight))]
pub trait Request {
    /// Endpoint where the request is made
    const ENDPOINT: &'static str;

    /// The type that represents the headers sent with this request
    type Headers: Headers;

    /// The type that represents the query parameters sent with this request
    type Parameters: Parameters;

    /// The type that represents the body of this request
    type Body: Body;

    /// The type returned by a sucessful request, must be [`DeserializeOwned`]
    /// and have at least a static lifetime (owned).
    type Response: Response + 'static;

    /// The type that encapsulates the error codes that this endpoint can return,
    /// must have at least a static lifetime (owned).
    type ErrorCodes: ErrorCodes + 'static;

    /// The method that this request will use
    const METHOD: reqwest::Method;

    /// Get a builder for this method
    fn builder() -> Self;

    /// Get the Headers struct for this Request
    ///
    /// Will only be called when [`Self::ready`] returns `Ok(())` and may not fail
    /// in that case
    fn headers(&self) -> &Self::Headers;

    /// Get the Parameters struct for this Request
    ///
    /// Will only be called when [`Self::ready`] returns `Ok(())` and may not fail
    /// in that case
    fn parameters(&self) -> &Self::Parameters;

    /// Get the Body struct for this Request
    ///
    /// Will only be called when [`Self::ready`] returns `Ok(())` and may not fail
    /// in that case
    fn body(&self) -> &Self::Body;

    /// Must return `Ok(())` if and only if this request is ready to have
    /// [`Self::make_request`] called on it.
    ///
    /// Should return [`RequestError::MalformedRequest`] with a message in the case
    /// that the request is not ready to be sent.
    ///
    /// Called by [`Self::make_request`], error is propogated.
    fn ready(&self) -> Result<(), RequestError<Self::ErrorCodes>>;

    /// Make the request represented by this object. Only makes request if [`Self::ready`] returns
    /// `Ok(())`.
    async fn make_request(
        &self,
        client: &Client,
    ) -> Result<Self::Response, RequestError<Self::ErrorCodes>> {
        // Make sure request thinks that it is ready to be sent
        self.ready()?;

        // Build request with method and endpoint
        let mut req = client.request(Self::METHOD, Self::ENDPOINT);

        // add headers, body, and params
        req = self.headers().write_headers(req);
        req = self.parameters().write_parameters(req);
        req = self.body().write_body(req);

        // send
        let resp = req.send().await?;

        // TODO deserialize into possible response untagged enum and return correct error codes if
        // failed with status

        /*match resp
            .json::<Result<Self::Response, FailureStatus<Self::ErrorCodes>>>()
            .await?
        {
            PossibleResponse::Response(r) => Ok(r),
            PossibleResponse::Failure(f) => Self::ErrorCodes::from_status(f)
                .map(|_| unreachable!())
                .map_err(|e| RequestError::ErrorCodes(e)),
        }*/

        resp.json::<PossibleResponse<Self::Response>>()
            .await?
            .into_result()
            .map_err(|e| {
                RequestError::ErrorCodes(FailureStatus::into_status::<Self::ErrorCodes>(e))
            })
    }
}

/// Type that is returned by a sucessful request
pub trait Response: DeserializeOwned + Sized {}

// Auto impl for types that are already [`DeserializeOwned`]
impl<T: DeserializeOwned> Response for T {}
