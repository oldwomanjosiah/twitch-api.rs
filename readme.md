[![Crates.io](https://img.shields.io/crates/v/twitch-api-rs?style=plastic)](https://crates.io/crates/twitch-api-rs)
[![Docs.rs](https://docs.rs/twitch-api-rs/badge.svg)](https://docs.rs/twitch-api-rs)
[![Docs-master](https://img.shields.io/badge/docs-master-blue)](https://oldwomanjosiah.github.io/twitch-api.rs/docs/)
[![Gihub Actions Testing CI](https://github.com/oldwomanjosiah/twitch-api.rs/workflows/Rust/badge.svg)](https://github.com/oldwomanjosiah/twitch-api.rs/actions?query=workflow%3ARust)

Note: As of right now this only covers 4/66 of the endpoints and 1/3 of the authorization flows.
This is because the [original version](https://github.com/oldwomanjosiah/twitch-api.rs/releases/tag/cargo-0.1.0) was built for use in a small application made to [download all the clips associated with a broadcaster](https://github.com/oldwomanjosiah/twitch-clip-downloader).
Version 0.2.0 moved to a trait based workflow to promote consistency between endpoints, as well as introducing transparent type wrappers on values consumed and created by the API to increase clarity on how the endpoints fit together.

My goal right now is to cover 100% of the end points that require only an application authorization [(client auth flow)](https://dev.twitch.tv/docs/authentication#types-of-tokens), as the other types of authorization require spinning up a webserver to complete. This feels out of scope as of right now but may become a feature-gated use of the library in the future. 

# twitch-api

A twitch crate used to build and send requests to
the twitch helix api.

See [Twitch API Reference](https://dev.twitch.tv/docs/)


### Example: Getting the top 20 clips from a creator
To get a list of clips from the twitch api, by a streamers dislplay name, there
are 4 steps.

1) Get an application ClientId and ClientSecret <sup> [1](#getting_credentials) </sup>
2) Request a client_flow Authentication Token
3) Use that token to request information on a user by their display name
4) Use the UserId returned by that request to request a list of clips associated
    with their channel.

```rust
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    use twitch_api_rs::prelude::*;
    use twitch_api_rs::auth::{ClientId, ClientSecret};

	// Here we use the values that you should have gotten from step one

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

	// Step three

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

	// Step three

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

	// Step four

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

<a name="getting_credentials">1</a>: If you do not already have these then you will need to
    log into your [twitch developer console](https://dev.twitch.tv/console), create and name a new application, and copy down
    the associated values.  
    The client secret that you get from this process is imediately invalidated if you request a
    new one, and it will not be shown to you once you leave the page where you generate it, so
    make sure to write it down somewhere safe. It is recommended that you treat it like a
    password.

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
