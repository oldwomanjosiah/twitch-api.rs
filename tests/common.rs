use reqwest::Client;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use twitch_api_rs::auth::client_credentials::ClientAuthToken;
use twitch_api_rs::auth::client_credentials::*;
use twitch_api_rs::requests::*;

use lazy_static::lazy_static;
use tokio::runtime::Builder;

lazy_static! {
    static ref CLIENT_SHARED: Arc<Client> = Arc::new(Client::new());
    static ref GUARD: Arc<()> = {
        let _ = env_logger::builder().is_test(true).init();
        Arc::new(())
    };
}

pub async fn get_client_auth_token_correct() -> ClientAuthToken {
    let (client_id, client_secret) = get_id_secret();

    let resp = ClientAuthRequest::builder()
        .set_client_id(client_id.clone())
        .set_client_secret(client_secret)
        .make_request(client().as_ref())
        .await
        .ok()
        .expect("Did not get a sucessful response from the server");

    ClientAuthToken::from_client(resp, client_id)
}

pub fn client() -> Arc<Client> {
    CLIENT_SHARED.clone()
}

pub fn get_id_secret() -> (String, String) {
    use std::env::var;
    match (
        var("TWITCH_API_RS_TEST_CLIENT_ID"),
        var("TWITCH_API_RS_TEST_CLIENT_SECRET"),
    ) {
        (Ok(a), Ok(b)) => (a, b),
        _ => panic!("Could not get client id and secret! are the environment variables TWITCH_API_RS_TEST_CLIENT_ID and TWITCH_API_RS_TEST_CLIENT_SECRET set?")
    }
}

pub fn init() {
    unsafe {
        let _ = std::ptr::read_volatile(&GUARD).clone();
    }
}
