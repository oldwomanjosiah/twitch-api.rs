use crate::response::BadRequest;
use log::{error, info};
use reqwest::{self, header::HeaderMap, Client};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize)]
pub struct ClipsRequest {
    broadcaster_id: String,
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
}

#[derive(Debug, PartialEq)]
pub struct ClipsRequestBuilder {
    broadcaster_id: Option<String>,
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
}

impl Default for ClipsRequestBuilder {
    fn default() -> Self {
        Self {
            broadcaster_id: None,
            after: None,
            before: None,
            first: None,
        }
    }
}

impl ClipsRequestBuilder {
    pub fn broadcaster_id(mut self, ins: String) -> Self {
        self.broadcaster_id.replace(ins);
        self
    }

    pub fn after(mut self, ins: Option<String>) -> Self {
        self.after = ins;
        self
    }

    pub fn before(mut self, ins: Option<String>) -> Self {
        self.before = ins;
        self
    }

    pub fn first(mut self, ins: Option<i32>) -> Self {
        self.first = ins;
        self
    }

    pub fn build(self) -> Result<ClipsRequest, Self> {
        if self.broadcaster_id.is_none() {
            return Err(self);
        }

        Ok(ClipsRequest {
            broadcaster_id: self.broadcaster_id.unwrap(),
            after: self.after,
            before: self.before,
            first: self.first,
        })
    }
}

impl ClipsRequest {
    pub fn builder() -> ClipsRequestBuilder {
        ClipsRequestBuilder::default()
    }

    pub async fn make_request(
        self,
        client: &Client,
        headers: HeaderMap,
    ) -> Option<PossibleClipsResponse> {
        info!("Making clips reqest");
        let resp = match {
            client
                .get("https://api.twitch.tv/helix/clips")
                .query(&self)
                .headers(headers)
                .send()
                .await
        } {
            Ok(resp) => resp,
            Err(e) => {
                error!("Error in ClipsRequest GET:\n{:#?}", e);
                return None;
            }
        };

        info!("Recieved response from ClipsRequest:\n{:#?}", &resp);

        if let Ok(resp) = resp.json::<PossibleClipsResponse>().await {
            Some(resp)
        } else {
            error!("Could not deserialize ClipsRequest\n");
            None
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ClipsResponse {
    pub data: Vec<ClipsResponseItem>,
    pub pagination: PaginationPartial,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct PaginationPartial {
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ClipsResponseItem {
    pub id: String,
    pub url: String,
    pub embed_url: String,
    pub broadcaster_id: String,
    pub broadcaster_name: String,
    pub creator_id: String,
    pub creator_name: String,
    pub video_id: String,
    pub game_id: String,
    pub language: String,
    pub title: String,
    pub view_count: i32,
    pub created_at: String,
    pub thumbnail_url: String,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PossibleClipsResponse {
    ClipsResponse(ClipsResponse),
    BadRequest(BadRequest),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn clips_response() {
        let clips_response: ClipsResponse = serde_json::from_str(
            r#"
{
  "data": [
    {
      "id":"RandomClip1",
      "url":"https://clips.twitch.tv/AwkwardHelplessSalamanderSwiftRage",
      "embed_url":"https://clips.twitch.tv/embed?clip=RandomClip1",
      "broadcaster_id":"1234",
      "broadcaster_name": "JJ",
      "creator_id":"123456",
      "creator_name": "MrMarshall",
      "video_id":"1234567",
      "game_id":"33103",
      "language":"en",
      "title":"random1",
      "view_count":10,
      "created_at":"2017-11-30T22:34:18Z",
      "thumbnail_url":"https://clips-media-assets.twitch.tv/157589949-preview-480x272.jpg"
    }
  ],
  "pagination": {
    "cursor": "eyJiIjpudWxsLCJhIjoiIn0"
  }
}
        "#,
        )
        .expect("Could not deserialize clips response");

        assert_eq!(
            clips_response,
            ClipsResponse {
                data: [ClipsResponseItem {
                    id: "RandomClip1".to_string(),
                    url: "https://clips.twitch.tv/AwkwardHelplessSalamanderSwiftRage".to_string(),
                    embed_url: "https://clips.twitch.tv/embed?clip=RandomClip1".to_string(),
                    broadcaster_id: "1234".to_string(),
                    broadcaster_name: "JJ".to_string(),
                    creator_id: "123456".to_string(),
                    creator_name: "MrMarshall".to_string(),
                    video_id: "1234567".to_string(),
                    game_id: "33103".to_string(),
                    language: "en".to_string(),
                    title: "random1".to_string(),
                    view_count: 10,
                    created_at: "2017-11-30T22:34:18Z".to_string(),
                    thumbnail_url:
                        "https://clips-media-assets.twitch.tv/157589949-preview-480x272.jpg"
                            .to_string(),
                },]
                .into(),
                pagination: PaginationPartial {
                    cursor: Some("eyJiIjpudWxsLCJhIjoiIn0".to_string()),
                },
            }
        );
    }
}
