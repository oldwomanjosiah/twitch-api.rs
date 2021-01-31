mod common;

#[tokio::test]
async fn get_channel_information() {
    use twitch_api_rs::prelude::*;
    use twitch_api_rs::resource::channels::get_channel_information::*;
    use twitch_api_rs::resource::users::get_users::*;

    common::init();
    let (client, auth) = (
        common::client(),
        common::get_client_auth_token_correct().await,
    );

    let user = {
        match GetUsersRequest::builder()
            .set_auth(auth.clone())
            .add_login("TheHoodlum12".to_string())
            .make_request(client.clone())
            .await
        {
            Ok(mut resp) => resp.data.remove(0).id,
            Err(e) => panic!("Could not get user information with reason {}", e),
        }
    };

    let resp = match GetChannelInformationRequest::builder()
        .set_auth(auth)
        .set_broadcaster_id(user)
        .make_request(client)
        .await
    {
        Ok(resp) => resp,
        Err(e) => panic!("Could not get channel information with reason {}", e),
    };

    log::info!(
        "Successfully got channel information request response {:#?}",
        resp
    );
}
