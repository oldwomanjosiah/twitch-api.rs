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
