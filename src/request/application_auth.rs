use crate::response::BadRequest;
use log::{debug, trace};
use reqwest::{self, Client};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq)]
pub struct AuthResponse {
    pub access_token: String,
    pub expires_in: i32,
    pub token_type: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PossibleAuthResponse {
    AuthResponse(AuthResponse),
    BadRequest(BadRequest),
}

#[derive(Debug)]
pub struct AuthRequestBuilder {
    client_id: Option<String>,
    client_secret: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AuthRequest {
    client_id: String,
    client_secret: String,
    grant_type: String,
}

impl Default for AuthRequestBuilder {
    fn default() -> Self {
        AuthRequestBuilder {
            client_id: None,
            client_secret: None,
        }
    }
}

impl AuthRequestBuilder {
    pub fn new() -> AuthRequestBuilder {
        AuthRequestBuilder::default()
    }

    pub fn client_id(mut self, id: String) -> Self {
        self.client_id.replace(id);
        self
    }

    pub fn client_secret(mut self, secret: String) -> Self {
        self.client_secret.replace(secret);
        self
    }

    pub fn build(self) -> Result<AuthRequest, Self> {
        trace!("Building Auth Request");
        if self.client_id.is_some() && self.client_secret.is_some() {
            Ok(AuthRequest {
                client_id: self.client_id.expect("Failed to unwrap"),
                client_secret: self.client_secret.expect("Failed to unwrap"),
                grant_type: "client_credentials".to_string(),
            })
        } else {
            Err(self)
        }
    }
}

impl AuthRequest {
    pub async fn make_request(self, client: &Client) -> Option<PossibleAuthResponse> {
        trace!("Beginning Authorization request");
        let resp = match {
            client
                .post("https://id.twitch.tv/oauth2/token")
                .query(&self)
                .send()
                .await
        } {
            Ok(resp) => resp,
            Err(_) => return None,
        };

        debug!("Recieved response: {:#?}", &resp);

        if let Ok(response) = resp.json::<PossibleAuthResponse>().await {
            Some(response)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn auth_response_test() {
        let auth_response: AuthResponse = serde_json::from_str(
            r#"
{
  "access_token": "prau3ol6mg5glgek8m89ec2s9q5i3i",
  "refresh_token": "",
  "expires_in": 3600,
  "scope": [],
  "token_type": "bearer"
}
                "#,
        )
        .expect("Could not deserialize response");

        assert_eq!(
            auth_response,
            AuthResponse {
                access_token: "prau3ol6mg5glgek8m89ec2s9q5i3i".to_string(),
                //refresh_token: "".to_string(),
                expires_in: 3600,
                //scope: Vec::new(),
                token_type: "bearer".to_string(),
            }
        );
    }
}
