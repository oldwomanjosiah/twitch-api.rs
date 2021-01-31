//! Common traits used to construct requestable types
//!
//! - TODO: Make `make_request` function based on feature and requests constructable without
//!     async or sending in all cases

use async_trait::async_trait;
use reqwest::Client;
use reqwest::RequestBuilder;
use serde::de::DeserializeOwned;
use thiserror::Error;

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

#[derive(Debug, Deserialize, Error)]
#[error("Status error: {status} with message {message}")]
/// Represents a sucessful request that was denied by the twitch api for some reason.
/// Use request's associated [`ErrorCodes`] to get matchable value.
pub struct FailureStatus<S>
where
    S: DeserializeOwned + std::fmt::Display + std::fmt::Debug + 'static,
{
    #[serde(bound(deserialize = "S: DeserializeOwned"))]
    /// The status code of the Failure
    ///
    /// If S is ErrorCodes then this is a known error for this request, if u16 then it is not known
    pub status: S,

    /// The message twitch sent with the error
    pub message: String,
}

impl<E: ErrorCodes> From<FailureStatus<u16>> for RequestError<E> {
    fn from(failure: FailureStatus<u16>) -> Self {
        match E::from_status(failure) {
            Ok(known) => RequestError::KnownErrorStatus(known),
            Err(unkn) => RequestError::UnkownErrorStatus(unkn),
        }
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
    Failure(FailureStatus<u16>),
}

impl<R> PossibleResponse<R>
where
    R: Response + 'static,
{
    fn into_result(self) -> Result<R, FailureStatus<u16>> {
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

    #[error("Did not have user scopes required {0:?}")]
    /// Did not have the correct user scopes available to make request.
    ScopesError(Vec<String>),

    #[error("Known Error enountered: {0}")]
    /// Encountered a known error status, match on `0.status` for all `C::*`
    KnownErrorStatus(FailureStatus<C>),

    #[error("Unknown Error enountered: {0}")]
    /// Encountered an unknown error status from twitch
    UnkownErrorStatus(FailureStatus<u16>),

    #[error("Reqwest encountered an error: {0}")]
    /// Reqwest could not complete the request for some reason
    ReqwestError(#[from] reqwest::Error),

    #[error("Unknown Error encountered {0:?}")]
    /// Unknown error
    UnknownError(#[from] Box<dyn std::error::Error>),
}

/// Represents A Known set of error status codes that an endpoint may return.o
///
/// See src for [`CommonResponseCodes`] for example of implementation using thiserror
pub trait ErrorCodes: std::error::Error + Sized + DeserializeOwned + Copy {
    /// Possibly mark the status as a known status of this kind, used by [`RequestError`]
    fn from_status(codes: FailureStatus<u16>) -> Result<FailureStatus<Self>, FailureStatus<u16>>;
}

#[derive(Debug, Clone, Copy, Error, Deserialize)]
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
}

#[macro_export]
/// Generate a [`ErrorCodes`] impl block for a given Enum by mapping known status codes
/// to specific variants. Variants must not be struct variants
macro_rules! response_codes {
    ($for:ty : [$($val:expr => $item:path),+]) => {
        impl ErrorCodes for $for {
            fn from_status(codes: FailureStatus<u16>) -> Result<FailureStatus<Self>, FailureStatus<u16>> {
                match codes.status {
                $(
                    $val => Ok(FailureStatus::<Self> {
                        status: $item,
                        message: codes.message
                    }),
                )*
                    _ => Err(codes),
                }
            }
        }
    }
}

response_codes!(
    CommonResponseCodes: [
        400 => CommonResponseCodes::BadRequestCode,
        401 => CommonResponseCodes::AuthErrorCode,
        500 => CommonResponseCodes::ServerErrorCode
]);

/// Headers for a request
pub trait Headers {
    /// Write headers to request builder and return request builder
    fn write_headers(&self, req: RequestBuilder) -> RequestBuilder;
}

/// Marker trait for auto implementation of headers
///
/// Must be able to borrow as a map of header names to values
pub trait HeadersExt {
    /// Borrow the object as map from header names to values
    fn as_ref<'a>(&'a self) -> &'a [(&'a str, &'a str)];
}

impl<T: HeadersExt> Headers for T {
    fn write_headers<'a>(&'a self, mut req: RequestBuilder) -> RequestBuilder {
        for (a, b) in self.as_ref() {
            req = req.header(*a, *b);
        }
        req
    }
}

/// Parameters for a request
pub trait Parameters {
    /// Write parameters to request builder and return request builder
    fn write_parameters(&self, req: RequestBuilder) -> RequestBuilder;
}

/// Marker trait for auto implementation of Parameters for types that implement
/// [`serde::Serialize`]
pub trait ParametersExt: serde::Serialize {}

impl<T: ParametersExt> Parameters for T {
    fn write_parameters(&self, req: RequestBuilder) -> RequestBuilder {
        req.query(self)
    }
}

/// Body for a request
pub trait Body {
    /// Write body to request builder and return request builder
    fn write_body(&self, req: RequestBuilder) -> RequestBuilder;
}

/// Marker trait for auto implementation of Body for types that implement
/// [`serde::Serialize`]
pub trait BodyExt: serde::Serialize {}

impl<T: BodyExt> Body for T {
    fn write_body(&self, req: RequestBuilder) -> RequestBuilder {
        req.json(self)
    }
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

        log::info!("Making request {:#?}", req);

        // send
        let resp = req.send().await?;

        resp.json::<PossibleResponse<Self::Response>>()
            .await?
            .into_result()
            .map_err(FailureStatus::into)
    }
}

/// Type that is returned by a sucessful request
pub trait Response: DeserializeOwned + Sized {}

// Auto impl for types that are already [`DeserializeOwned`]
impl<T: DeserializeOwned> Response for T {}
