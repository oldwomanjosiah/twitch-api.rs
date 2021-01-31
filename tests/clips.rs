mod common;

#[tokio::test]
async fn get_clips_success() {
    common::init();
    use twitch_api_rs::requests::*;
    use twitch_api_rs::resource::clips::get_clips::*;
    use twitch_api_rs::resource::users::get_users::*;

    let (client, client_auth_token) = (
        common::client(),
        common::get_client_auth_token_correct().await,
    );

    log::info!("Got client and auth: {:?}", client_auth_token);

    let broadaster_id = match GetUsersRequest::builder()
        .set_auth(client_auth_token.clone())
        .add_login("TheHoodlum12")
        .make_request(client.clone().as_ref())
        .await
    {
        Ok(resp) => resp.data[0].id.clone(),
        Err(e) => panic!("Could not get user id for TheHoodlum12 for reason {}", e),
    };

    match GetClipsRequest::builder()
        .set_auth(client_auth_token.clone())
        .set_broadcaster_id(broadaster_id)
        .make_request(client.clone().as_ref())
        .await
    {
        Ok(resp) => {
            eprintln!("Sucessfully got clips object\n{:#?}", resp.clips);
            return;
        }
        Err(e) => panic!(
            "Could not get clips for broadaster_id returned by GetUsersRequest with reason {}",
            e
        ),
    }
}
