use crate::response::BadRequest;
use log::{error, info, warn};
use reqwest::{self, header::HeaderMap, Client};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq)]
pub struct ChannelInformationResponse {
    pub data: Vec<ChannelInformationResponseItem>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ChannelInformationResponseItem {
    pub id: String,
    pub login: String,
    pub display_name: String,
    #[serde(rename = "type")]
    pub channel_type: String,
    pub broadcaster_type: String,
    pub description: String,
    pub profile_image_url: String,
    pub offline_image_url: String,
    pub view_count: i32,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PossibleChannelInformationResponse {
    ChannelInformationResponse(ChannelInformationResponse),
    BadRequest(BadRequest),
}

/// Get more info on a channel based on its login name
///
/// # Usage
///
/// ```
/// # use crate::twitch::response::*;
/// let resp = ChannelInformationRequest::builder()
///     .channel_name(String::from("nikeairjordan9"))
///     .build()
///     .make_request().await;
///
/// match resp {
///     PossibleChannelInformationResponse::ChannelInformationResponse(resp) => { ... },
///     PossibleChannelInformationResponse::BadRequest(err) => { ... },
/// }
/// ```
#[derive(Debug, Serialize, PartialEq)]
pub struct ChannelInformationRequest {
    login: String,
}

#[derive(Debug, PartialEq)]
pub struct ChannelInformationRequestBuilder {
    channel_name: Option<String>,
}

impl ChannelInformationRequestBuilder {
    /// Set the Login name for the channel
    pub fn channel_name(mut self, id: String) -> Self {
        self.channel_name.replace(id);
        self
    }

    /// Build [ChannelInformationRequest]
    ///
    /// Fails if [channel_name] not set
    ///
    /// [ChannelInformationRequest]: struct.ChannelInformationRequest.html
    /// [channel_name]: struct.ChannelInformationRequestBuilder.html#structfield.channel_name
    pub fn build(self) -> Result<ChannelInformationRequest, Self> {
        if self.channel_name.is_none() {
            warn!("Could not build ChannelInformationRequest");
            return Err(self);
        }
        info!("Building ChannelInformationRequest");
        Ok(ChannelInformationRequest {
            login: self.channel_name.unwrap(),
        })
    }
}

impl Default for ChannelInformationRequestBuilder {
    fn default() -> Self {
        ChannelInformationRequestBuilder { channel_name: None }
    }
}

impl ChannelInformationRequest {
    /// Create [builder] for `ChannelInformationRequest`
    ///
    /// [builder]: struct.ChannelInformationRequestBuilder.html
    pub fn builder() -> ChannelInformationRequestBuilder {
        ChannelInformationRequestBuilder::default()
    }

    pub async fn make_request(
        self,
        client: &Client,
        headers: HeaderMap,
    ) -> Option<PossibleChannelInformationResponse> {
        info!("Making user info request");
        let resp = match {
            client
                .get("https://api.twitch.tv/helix/users")
                .query(&self)
                .headers(headers)
                .send()
                .await
        } {
            Ok(resp) => resp,
            Err(e) => {
                error!("Error in ChannelInformationRequest GET:\n{:#?}", e);
                return None;
            }
        };

        info!(
            "Recieved response from ChannelInformationRequest:\n{:#?}",
            &resp
        );

        if let Ok(response) = resp.json::<PossibleChannelInformationResponse>().await {
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
    fn build() {
        assert_eq!(
            ChannelInformationRequest::builder()
                .channel_name(String::from("nikeairjordan9"))
                .build(),
            Ok(ChannelInformationRequest {
                login: String::from("nikeairjordan9"),
            })
        );
    }

    #[test]
    fn build_fail() {
        assert_eq!(
            ChannelInformationRequest::builder().build(),
            Err(ChannelInformationRequestBuilder { channel_name: None })
        );
    }

    #[test]
    pub fn channel_id_response_test() {
        let channel_id_response: ChannelInformationResponse = serde_json::from_str(
            r#"
{
  "data": [{
    "id": "44322889",
    "login": "dallas",
    "display_name": "dallas",
    "type": "staff",
    "broadcaster_type": "",
    "description": "Just a gamer playing games and chatting. :)",
    "profile_image_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/dallas-profile_image-1a2c906ee2c35f12-300x300.png",
    "offline_image_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/dallas-channel_offline_image-1a2c906ee2c35f12-1920x1080.png",
    "view_count": 191836881,
    "email": "login@provider.com"
  }]
}
                "#
        ).expect("Could not deserialize response");

        assert_eq!(
            channel_id_response,
            ChannelInformationResponse {
                data: [
                    ChannelInformationResponseItem {
                        id: "44322889".to_string(),
                        login: "dallas".to_string(),
                        display_name: "dallas".to_string(),
                        channel_type: "staff".to_string(),
                        broadcaster_type: "".to_string(),
                        description: "Just a gamer playing games and chatting. :)".to_string(),
                        profile_image_url:
                            "https://static-cdn.jtvnw.net/jtv_user_pictures/dallas-profile_image-1a2c906ee2c35f12-300x300.png".to_string(),
                        offline_image_url:
                            "https://static-cdn.jtvnw.net/jtv_user_pictures/dallas-channel_offline_image-1a2c906ee2c35f12-1920x1080.png".to_string(),
                        view_count: 191836881,
                        email: Some("login@provider.com".to_string()),
                    }
                ].into(),
            }
        );
    }

    #[test]
    pub fn channel_id_response_test_2() {
        let channel_id_response: ChannelInformationResponse = serde_json::from_str(r#"
{
  "data": [
    {
      "id": "447666275",
      "login": "nikeairjordan9",
      "display_name": "nikeairjordan9",
      "type": "",
      "broadcaster_type": "affiliate",
      "description": "",
      "profile_image_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/52770ffb-a8ee-4ca8-be04-ab9265a911a4-profile_image-300x300.png",
      "offline_image_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/841868f2-7447-4741-811a-ded1a3978ef3-channel_offline_image-1920x1080.jpeg",
      "view_count": 3000
    }
  ]
}
            "#).expect("Could not deserialize response");

        assert_eq!(channel_id_response,
                ChannelInformationResponse {
                    data: [
                        ChannelInformationResponseItem {
                          id: "447666275".to_string(),
                          login: "nikeairjordan9".to_string(),
                          display_name: "nikeairjordan9".to_string(),
                          channel_type: "".to_string(),
                          broadcaster_type: "affiliate".to_string(),
                          description: "".to_string(),
                          profile_image_url: "https://static-cdn.jtvnw.net/jtv_user_pictures/52770ffb-a8ee-4ca8-be04-ab9265a911a4-profile_image-300x300.png".to_string(),
                          offline_image_url: "https://static-cdn.jtvnw.net/jtv_user_pictures/841868f2-7447-4741-811a-ded1a3978ef3-channel_offline_image-1920x1080.jpeg".to_string(),
                          view_count: 3000,
                          email: None,
                        }
                  ].into()
                });
    }

    #[test]
    pub fn channel_id_response_test_optional() {
        let channel_id_response: PossibleChannelInformationResponse = serde_json::from_str(r#"
{
  "data": [
    {
      "id": "447666275",
      "login": "nikeairjordan9",
      "display_name": "nikeairjordan9",
      "type": "",
      "broadcaster_type": "affiliate",
      "description": "",
      "profile_image_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/52770ffb-a8ee-4ca8-be04-ab9265a911a4-profile_image-300x300.png",
      "offline_image_url": "https://static-cdn.jtvnw.net/jtv_user_pictures/841868f2-7447-4741-811a-ded1a3978ef3-channel_offline_image-1920x1080.jpeg",
      "view_count": 3000
    }
  ]
}
        "#).expect("Could not deserialize response");

        assert_eq!(channel_id_response,
            PossibleChannelInformationResponse::ChannelInformationResponse(ChannelInformationResponse {
                data: [
                    ChannelInformationResponseItem {
                      id: "447666275".to_string(),
                      login: "nikeairjordan9".to_string(),
                      display_name: "nikeairjordan9".to_string(),
                      channel_type: "".to_string(),
                      broadcaster_type: "affiliate".to_string(),
                      description: "".to_string(),
                      profile_image_url: "https://static-cdn.jtvnw.net/jtv_user_pictures/52770ffb-a8ee-4ca8-be04-ab9265a911a4-profile_image-300x300.png".to_string(),
                      offline_image_url: "https://static-cdn.jtvnw.net/jtv_user_pictures/841868f2-7447-4741-811a-ded1a3978ef3-channel_offline_image-1920x1080.jpeg".to_string(),
                      view_count: 3000,
                      email: None,
                    }
              ].into()
            }));
    }
}
