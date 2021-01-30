mod common;

#[tokio::test]
async fn get_users_correct() {
    let (client_id, client_secret) = crate::common::get_id_secret();

    use twitch_api_rs::auth::client_credentials::*;
    use twitch_api_rs::requests::Request;
    use twitch_api_rs::resource::users::get_users::*;

    let client = reqwest::Client::new();

    let resp = match ClientAuthRequest::builder()
        .set_client_id(client_id.clone())
        .set_client_secret(client_secret)
        .make_request(&client)
        .await
    {
        Ok(resp) => resp,
        Err(e) => panic!("Test failed trying to make auth request with error {}", e),
    };

    let token = ClientAuthToken::from_client(resp, client_id);

    let resp2 = match GetUsersRequest::builder()
        .set_auth(token)
        .add_login("TheHoodlum12")
        .make_request(&client)
        .await
    {
        Ok(resp) => resp,
        Err(e) => panic!(
            "Test failed trying to make get_users request with error {}",
            e
        ),
    };

    eprintln!(
        "Sucessfully recieved the response from twitch:\n{:#?}",
        resp2
    );
}

#[tokio::test]
async fn too_few_arguments() {
    let (client_id, client_secret) = crate::common::get_id_secret();

    use twitch_api_rs::auth::client_credentials::*;
    use twitch_api_rs::requests::{Request, RequestError};
    use twitch_api_rs::resource::users::get_users::*;

    let client = reqwest::Client::new();

    let resp = match ClientAuthRequest::builder()
        .set_client_id(client_id.clone())
        .set_client_secret(client_secret)
        .make_request(&client)
        .await
    {
        Ok(resp) => resp,
        Err(e) => panic!("Test failed trying to make auth request with error {}", e),
    };

    let token = ClientAuthToken::from_client(resp, client_id);

    let error = match GetUsersRequest::builder()
        .set_auth(token)
        .make_request(&client)
        .await
    {
        Ok(resp) => unreachable!("Should not even try to make the request as there were not enough arguments specified: got {:?}", resp),
        Err(e) => e,
    };

    match error {
        RequestError::MalformedRequest(msg) => {
            eprintln!("Failed correctly with message {}", msg);
            return;
        }
        e => unreachable!(
            "Incorrectly tried to send the request and failed with error: {}",
            e
        ),
    }
}

#[tokio::test]
async fn too_many_arguments() {
    let (client_id, client_secret) = crate::common::get_id_secret();

    use twitch_api_rs::auth::client_credentials::*;
    use twitch_api_rs::requests::{Request, RequestError};
    use twitch_api_rs::resource::users::get_users::*;

    let client = reqwest::Client::new();

    let resp = match ClientAuthRequest::builder()
        .set_client_id(client_id.clone())
        .set_client_secret(client_secret)
        .make_request(&client)
        .await
    {
        Ok(resp) => resp,
        Err(e) => panic!("Test failed trying to make auth request with error {}", e),
    };

    let token = ClientAuthToken::from_client(resp, client_id);

    let mut req = GetUsersRequest::builder();

    for _ in 0..101 {
        // Set too many search requests
        req.add_login("TheHoodlum12");
    }

    let error = match req.set_auth(token)
        .make_request(&client)
        .await
    {
        Ok(resp) => unreachable!("Should not even try to make the request as there were not enough arguments specified: got {:?}", resp),
        Err(e) => e,
    };

    match error {
        RequestError::MalformedRequest(msg) => {
            eprintln!("Failed correctly with message {}", msg);
            return;
        }
        e => unreachable!(
            "Incorrectly tried to send the request and failed with error: {}",
            e
        ),
    }
}

#[tokio::test]
async fn no_auth_specified() {
    use twitch_api_rs::auth::client_credentials::ClientAuthToken;
    use twitch_api_rs::requests::{Request, RequestError};
    use twitch_api_rs::resource::users::get_users::*;

    let client = reqwest::Client::new();

    let error = match GetUsersRequest::<ClientAuthToken>::builder()
        .make_request(&client)
        .await
    {
        Ok(resp) => unreachable!(
            "Should not try to make the request as there was no auth set: got {:?}",
            resp
        ),
        Err(e) => e,
    };

    match error {
        RequestError::MalformedRequest(msg) => {
            eprintln!("Failed correctly with message {}", msg);
            return;
        }
        e => unreachable!(
            "Incorrectly tried to send the request and failed with error: {}",
            e
        ),
    }
}
