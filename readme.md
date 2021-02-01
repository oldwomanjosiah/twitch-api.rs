[![Crates.io](https://img.shields.io/crates/v/twitch-api-rs?style=plastic)](https://crates.io/crates/twitch-api-rs)
[![Docs.rs](https://docs.rs/twitch-api-rs/badge.svg)](https://docs.rs/twitch-api-rs)
[![Gihub Actions Testing CI](https://github.com/oldwomanjosiah/twitch-api.rs/workflows/Rust/badge.svg)](https://github.com/oldwomanjosiah/twitch-api.rs/actions?query=workflow%3ARust)

Note: this library only covers the three apis on the critical path for my other project which [downloads all the clips for a user's channel](https://github.com/oldwomanjosiah/twitch-clip-downloader). Eventually I'd like to get to 100% coverage, at the very least for [Application Endpoints](https://dev.twitch.tv/docs/authentication#types-of-tokens).

# twitch-api

A Small Crate to query the twitch public api (helix)

## Example: Get Clips

```rust
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    use twitch_api_rs::prelude::*;
    use twitch_api_rs::auth::{ClientId, ClientSecret};

    let client_id: ClientId =
        env::var("TWITCH_API_RS_TEST_CLIENT_ID")
            .expect("Client Id environment variable not defined")
            .into();

    let client_secret: ClientSecret =
        env::var("TWITCH_API_RS_TEST_CLIENT_SECRET")
            .expect("Client Secret environment variable not defined")
            .into();

    // Make a reqwest client to send the requests, and wrap it in an arc so it
    // can be reused in multiple futures
    let client = Arc::new(reqwest::Client::new());

    use twitch_api_rs::auth::client_credentials::{
        ClientAuthRequest, ClientAuthResponse, ClientAuthToken,
    };

    // Get a client credentials (application) access token and wrap it in an arc
    // to be used across multiple tasks without repeating
    let auth_token: Arc<ClientAuthToken> = Arc::new(
        match ClientAuthRequest::builder()
            .set_client_id(client_id.clone())
            .set_client_secret(client_secret)
            .make_request(client.clone())
            .await {
                Ok(resp) => {
                    // Create the token from the token value provided by twitch and
                    // your client_id
                    ClientAuthToken::from_client(resp, client_id)
                }

                // Better error handling can be performed here by matching against
                // the type of this requests::RequestError. Elided for conciseness
                Err(e) => panic!("Could not complete auth request for reason {}", e),
            }
    );

    use twitch_api_rs::values::users::UserId;
    use twitch_api_rs::resource::users::get_users::*;

    let user_ids: Vec<UserId> =
        match GetUsersRequest::builder()
            .set_auth(auth_token.clone())
            .add_login("TheHoodlum12")
            .make_request(client.clone())
            .await {
                Ok(mut resp) => {
                    resp.users.into_iter().map(|i| i.id).collect()
                }

                Err(e) =>
                    panic!("Could not complete request for user by display name for reason {}", e),
            };

    use twitch_api_rs::resource::clips::ClipInfo;
    use twitch_api_rs::resource::clips::get_clips::*;
    use twitch_api_rs::values::clips::ClipTitle;

    for user_id in user_ids {
        let auth_token = auth_token.clone();
        let client = client.clone();

        tokio::spawn(async move {
            match GetClipsRequest::builder()
                .set_auth(auth_token)
                .set_broadcaster_id(user_id.clone())
                .make_request(client)
                .await {
                    Ok(resp) => {
                        // Print out the response
                        for clip in resp.clips {
                            eprintln!(
                                "Found clip for broadcaster {:?} titled {:?}",
                                &user_id, clip.title
                            );
                        }
                    }
                    Err(e) =>
                        panic!("Could not get clips for user {:?} for reason {}", user_id, e),
                }
        });
    }
}
```

### Testing

To run the integration tests you need to set the environment variables with valid
values from the [twitch developer console](https://dev.twitch.tv/console).

```bash
TWITCH_API_RS_TEST_CLIENT_ID=<client_id> /
TWITCH_API_RS_TEST_CLIENT_SECRET=<client_secret> /
cargo test -- --nocapture
```

If you use [cargo-make](https://crates.io/crates/cargo-make) you can also add the following to your `Makefile.toml`

```toml
[tasks.test-env]
env = { "TWITCH_API_RS_TEST_CLIENT_ID" = "<client_id>", "TWITCH_API_RS_TEST_CLIENT_SECRET" = "<client_secret>" }
command = "cargo"
args = [ "test", "--", "--nocapture" ]
```

-------

Maintainer: oldwomanjosiah (jhilden13@gmail.com)
