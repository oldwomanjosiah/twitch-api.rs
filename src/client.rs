use std::{fmt::Display, future::Future, sync::Arc};

use reqwest::Client as RClient;
use thiserror::Error;

use crate::{
    auth::{
        client_credentials::{ClientAuthResponse, ClientAuthToken},
        ClientId, ClientSecret,
    },
    crate_prelude::{FailureStatus, PossibleResponse},
};

#[derive(Debug)]
pub struct RequestError {
    ty: RequestErrorType,
}

impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ty)
    }
}

impl std::error::Error for RequestError {}

impl<T> From<T> for RequestError
where
    RequestErrorType: From<T>,
{
    fn from(ty: T) -> Self {
        Self { ty: ty.into() }
    }
}

#[derive(Debug, Error)]
pub enum RequestErrorType {
    #[error("{from}")]
    ReqwestError {
        #[from]
        from: reqwest::Error,
    },

    #[error("{from}")]
    FailureStatusNum {
        #[from]
        from: FailureStatus<u16>,
    },
}

pub struct Client<T: ClientState> {
    common: Box<ClientStateCommon>,
    current: T,
}

struct ClientStateCommon {
    client: Arc<RClient>,
}

pub struct Unauthorized {}
pub struct ClientCredentials {
    token: Arc<ClientAuthToken>,
}

impl<T: ClientState> Client<T> {
    /// Create a new client
    pub fn new(client: RClient) -> Client<Unauthorized> {
        Client {
            common: Box::new(ClientStateCommon {
                client: Arc::new(client),
            }),
            current: Unauthorized {},
        }
    }
}

impl Client<Unauthorized> {
    pub async fn client_auth(
        self,
        id: ClientId,
        secret: ClientSecret,
    ) -> Result<Client<ClientCredentials>, (RequestError, Self)> {
        let resp = self
            .common
            .client
            .get("https://id.twitch.tv/oauth2/token/")
            .query(&[
                ("client_id", id.to_string()),
                ("client_secret", secret.to_string()),
                ("grant_type", "client_credentials".to_string()),
            ])
            .send()
            .await;

        let r = match resp {
            Ok(r) => r.json::<PossibleResponse<ClientAuthResponse>>().await,
            Err(e) => return Err((e.into(), self)),
        };

        let r = match r {
            Ok(PossibleResponse::Response(r)) => r.access_token,
            Ok(PossibleResponse::Failure(f)) => return Err((f.into(), self)),
            Err(e) => return Err((e.into(), self)),
        };

        Ok(Client {
            common: self.common,
            current: ClientCredentials {
                token: Arc::new(ClientAuthToken::new(r, id)),
            },
        })
    }
}

pub trait ClientState: sealed::Sealed {}

impl ClientState for Unauthorized {}
impl ClientState for ClientCredentials {}

/// Make sure that only types named here can implement ClientState
mod sealed {
    use super::*;

    pub trait Sealed {}

    impl Sealed for Unauthorized {}
    impl Sealed for ClientCredentials {}
}
