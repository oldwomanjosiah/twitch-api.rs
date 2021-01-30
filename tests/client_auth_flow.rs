extern crate tokio;

/// Test should pass so long as client_id and client_secret are set
#[tokio::test]
async fn correct_auth_flow() {
    use twitch_api_rs::auth::client_credentials::*;
    use twitch_api_rs::requests::*;

    let client_id = match std::env::var("TWITCH_API_RS_TEST_CLIENT_ID") {
        Ok(v) => v,
        Err(_) => panic!("Must provide TWITCH_API_RS_TEST_CLIENT_ID"),
    };

    let client_secret = match std::env::var("TWITCH_API_RS_TEST_CLIENT_SECRET") {
        Ok(v) => v,
        Err(_) => panic!("Must provide TWITCH_API_RS_TEST_CLIENT_SECRET"),
    };

    let client = reqwest::Client::new();

    let error = match ClientAuthRequest::builder()
        .set_client_id(client_id)
        .set_client_secret(client_secret)
        .make_request(&client)
        .await
    {
        Ok(resp) => {
            let (token, expiration) = resp.into();
            eprintln!("Got Token {}. (Expires in {} seconds)", token, expiration);
            return;
        }
        Err(e) => e,
    };

    match error {
        RequestError::MalformedRequest(msg) =>
            unreachable!("Failed to set all parameters correctly with message {}", msg),

        RequestError::ErrorCodes(c) =>
            unreachable!("Request failed with code: {}, did you not set your client_id and client_secret correctly?", c),

        err =>
            unreachable!("Could not complete request for reason {}", err),
    }
}
