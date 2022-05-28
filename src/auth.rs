use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod test {
    use crate::auth::auth_url;
    #[test]
    fn auth_url_test() {
        let url = auth_url("my_client_id", "code", None, Some("me vote"), None);
        assert_eq!("https://api.genius.com/oauth/authorize?client_id=my_client_id&response_type=code&scope=me+vote", url.as_str());
    }
}

/// Authentication by login.
pub mod login;

#[derive(Serialize)]
struct AuthRequest {
    code: String,
    client_secret: String,
    client_id: String,
    redirect_uri: String,
    response_type: String,
    grant_type: String,
}

/// Authentication response.
#[derive(Deserialize, Debug)]
pub struct AuthResponse {
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub error: Option<String>,
    pub error_description: Option<String>,
}

/// Format genius authentication URL, the result is a URL. `client_id` and `redirect_uri` ​​are found at <https://genius.com/api-clients>.
///
/// Only `response_type` and `client_id` is required, `response_type` can be `token` or `code`.
/// When the response type is `token` the redirect URL will contain an `access_token` parameter with the token.
/// When the response type is `code` The redirect URL will contain a `code` parameter with a code that using the [`authenticate`] function will return a token.
/// > If you are creating something server-side, the `code` works great and this library has a method to handle it: [`authenticate`].
///
/// The state will be a value that be passed when redirected.
///
/// The scope will define what permissions your application will have. Available scopes are `me`, `create_annotation`, `manage_annotation` and `vote`.
/// #### Examples
/// Basic usage:
/// ```
/// use genius_rs::auth::auth_url;
///
/// let auth_url = auth_url("my_client_id", "code", None, Some("me vote"), None);
/// ```
#[must_use]
pub fn auth_url(
    client_id: &str,
    response_type: &str,
    redirect_uri: Option<&str>,
    scope: Option<&str>,
    state: Option<&str>,
) -> Url {
    let mut params = vec![("client_id", client_id), ("response_type", response_type)];
    if let Some(redirect_uri) = redirect_uri {
        params.push(("redirect_uri", redirect_uri));
    }
    if let Some(scope) = scope {
        params.push(("scope", scope));
    }
    if let Some(state) = state {
        params.push(("state", state));
    }
    Url::parse_with_params("https://api.genius.com/oauth/authorize", params)
        .expect("Can't parse authentication URL.")
}

/// Transform the `code` in a token, the result is [`AuthResponse`]. `code` expires so be very light on this operation. The response token will be level `client`.
///
/// `client_secret`, `client_id` and `redirect_uri` are found at <https://genius.com/api-clients>.
///
/// # Errors
///
/// If the code is not valid or the request fails.
pub async fn authenticate(
    code: String,
    client_secret: String,
    client_id: String,
    redirect_uri: String,
) -> Result<AuthResponse, reqwest::Error> {
    let auth_req = AuthRequest {
        code,
        client_secret,
        client_id,
        redirect_uri,
        response_type: "code".to_owned(),
        grant_type: "authorization_code".to_owned(),
    };
    let url = Url::parse("https://api.genius.com/oauth/token")
        .expect("Could not parse valid URL from login_with_username input.");
    let client = Client::new();
    let request = client.post(url).json(&auth_req).send().await?;
    let result = request.json::<AuthResponse>().await?;
    Ok(result)
}
