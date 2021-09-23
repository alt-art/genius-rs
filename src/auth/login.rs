use base64::decode;
use chrono::offset::Utc;
use hmac::{Hmac, Mac, NewMac};
use reqwest::{Client, Url};
use serde::Serialize;
use sha2::Sha256;

use crate::auth::AuthResponse;

#[cfg(test)]
mod test {
    use crate::auth::login::*;
    use dotenv;
    #[tokio::test]
    async fn login_with_username_test() {
        let auth = login_with_username(
            &dotenv::var("LOG_USER").expect("Can't get username environment variable"),
            &dotenv::var("LOG_PASSW").expect("Can't get password environment variable."),
        )
        .await
        .expect("Error requesting login with username");
        assert_eq!(auth.access_token.is_some(), true);
    }
}

#[derive(Serialize)]
struct AuthLoginRequest {
    password: String,
    username: String,
    client_id: String,
    client_secret_digest: String,
    grant_type: String,
    timestamp: String,
}

/// Log into an account with username and password the response will be [`AuthResponse`]. The response token will be level `user-core`.
/// #### Examples
/// Basic usage:
/// ```
/// use genius_rs::auth::login;
///
/// # async fn run() -> Result<(), reqwest::Error> {
/// let auth = login::login_with_username("username", "password")
///     .await
///     .expect("Error requesting login with username");
///
/// match auth.access_token {
///     Some(token) => println!("The token is: {}.", token),
///     None => println!("Incorrect username or password."),
/// }
/// # Ok(())
/// # }
/// ```
pub async fn login_with_username(
    username: &str,
    password: &str,
) -> Result<AuthResponse, reqwest::Error> {
    let url = Url::parse("https://api.genius.com/oauth/token")
        .expect("Could not parse valid URL from login_with_username input.");
    let auth_request = username_auth_body(username, password);
    let client = Client::new();
    let res: AuthResponse = client
        .post(url)
        .json(&auth_request)
        .send()
        .await?
        .json()
        .await?;
    Ok(res)
}

fn username_auth_body(username: &str, password: &str) -> AuthLoginRequest {
    let timestamp = Utc::now().timestamp().to_string();
    let grant_type = "password".to_string();
    let long_id_hist =
        "aDNrY0UyM0ZkS1I0djZ1ck1ZVExrcDJUMGFOMFZ2WEdXMkY0a1VPMG5jWGZYeXk5Z2oxZGNSbnRmNnBJOS1RMA==";
    let client_id =
        String::from_utf8(decode(long_id_hist).expect("Unable to decode the client id."))
            .expect("Error transforming client id to string.");
    let client_secret_digest = digest(username, &timestamp);
    AuthLoginRequest {
        password: password.to_string(),
        username: username.to_string(),
        client_id,
        client_secret_digest,
        grant_type,
        timestamp,
    }
}

fn digest(username: &str, timestamp: &str) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(&decode("ZEVWWVpfcDVzX0tHY3Y0UGJJN015LWpjdXBhMHdWcTZJT081S1BqSzBKNjI2cXozWVA4OVphS1BTS3VHVDZONkQ1eTN1ZXc4WGVicnk4YmZXWkt5Rnc=").expect("Unable to decode the key.")).expect("An error occurred loading the key.");
    mac.update(format!("{}{}", username, timestamp).as_bytes());
    format!("{:x}", mac.finalize().into_bytes())
}
