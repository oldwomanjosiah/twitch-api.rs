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
        .make_request(client)
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
        RequestError::MalformedRequest(msg) => unreachable!(
            "Failed to set all parameters correctly with message {}",
            msg
        ),

        RequestError::KnownErrorStatus(c) => unreachable!(
            "Request failed with known status: {}, and message {}?",
            c.status, c.message
        ),

        RequestError::UnkownErrorStatus(c) => unreachable!(
            "Request failed with unknown/unexpected status: {}, and message {}?",
            c.status, c.message
        ),

        err => unreachable!("Could not complete request for reason {}", err),
    }
}

#[tokio::test]
async fn fail_on_client_id_not_set() {
    use twitch_api_rs::auth::client_credentials::*;
    use twitch_api_rs::requests::*;

    let client_secret = String::from("Should fail");

    let client = reqwest::Client::new();

    let error = match ClientAuthRequest::builder()
        .set_client_secret(client_secret)
        .make_request(client)
        .await
    {
        Ok(_) => unreachable!(
            "Sent request (and recieved a valid answer???) even though client_id not set"
        ),
        Err(e) => e,
    };

    match error {
        RequestError::MalformedRequest(msg) => {
            eprintln!("Sucessfully blocked request with message {}", msg);
            return;
        }
        e => unreachable!(
            "Failed to block request being sent and gave the wrong error {:?}",
            e
        ),
    }
}

#[tokio::test]
async fn fail_on_client_secret_not_set() {
    use twitch_api_rs::auth::client_credentials::*;
    use twitch_api_rs::requests::*;

    let client_id = String::from("Should fail");

    let client = reqwest::Client::new();

    let error = match ClientAuthRequest::builder()
        .set_client_id(client_id)
        .make_request(&client)
        .await
    {
        Ok(_) => unreachable!(
            "Sent request (and recieved a valid answer???) even though client_id not set"
        ),
        Err(e) => e,
    };

    match error {
        RequestError::MalformedRequest(msg) => {
            eprintln!("Sucessfully blocked request with message {}", msg);
            return;
        }
        e => unreachable!(
            "Failed to block request being sent and gave the wrong error {:?}",
            e
        ),
    }
}

#[tokio::test]
async fn bad_client_id() {
    use twitch_api_rs::auth::client_credentials::*;
    use twitch_api_rs::requests::*;

    let client_id = String::from("Should fail");
    let client_secret = String::from("Should fail");

    let client = reqwest::Client::new();

    let error = match ClientAuthRequest::builder()
        .set_client_id(client_id)
        .set_client_secret(client_secret)
        .make_request(&client)
        .await
    {
        Ok(_) => unreachable!(
            "Sent request (and recieved a valid answer???) even though client_id not set"
        ),
        Err(e) => e,
    };

    match error {
        RequestError::KnownErrorStatus(s) => match s.status {
            CommonResponseCodes::BadRequestCode => eprintln!(
                "Sucessfully parsed rejection from twitch server with message {}",
                s.message
            ),
            e => unreachable!("Parsed wrong error code: {:?}", e),
        },
        e => unreachable!(
            "Failed to parse out well known error status with error {:?}",
            e
        ),
    }
}
