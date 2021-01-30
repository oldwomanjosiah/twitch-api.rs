[![Build Status](https://travis-ci.org/oldwomanjosiah/twitch-api.rs.svg?branch=master)](https://travis-ci.org/oldwomanjosiah/twitch-api.rs)

Note: this library only covers the three apis on the critical path for my other project which [downloads all the clips for a user's channel](https://github.com/oldwomanjosiah/twitch-clip-downloader). Eventually I'd like to get to 100% coverage, at the very least for [Application Endpoints](https://dev.twitch.tv/docs/authentication#types-of-tokens).

# twitch-api

[Crates.io](https://crates.io/crates/twitch-api-rs)

A Small Crate to query the twitch public api (helix)

### Testing

To run the integration tests you need to set the environment variables with valid
values from the [twitch developer console](https://dev.twitch.tv/console).

```
TWITCH_API_RS_TEST_CLIENT_ID=<client_id>
TWITCH_API_RS_TEST_CLIENT_SECRET=<client_secret>
```

-------

Maintainer: oldwomanjosiah (jhilden13@gmail.com)
