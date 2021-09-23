use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};

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

/// Format genius authentication URL, the result is an url. `client_id` and `redirect_uri` ​​are found at <https://genius.com/api-clients>.
///
/// only `response_type` and `client_id` is required, `response_type` can be `token` or `code`.
/// > If you are creating something server side, the `code` works great and this library has a method to handle it: [`authenticate`].
///
/// Available scopes are `me`, `create_annotation`, `manage_annotation` and `vote`.
pub fn auth_url(
    client_id: &str,
    response_type: &str,
    redirect_uri: &str,
    scope: &str,
    state: &str,
) -> String {
    let mut url = format!(
        "https://api.genius.com/oauth/authorize?client_id={}&response_type={}",
        client_id, response_type
    );
    if !scope.is_empty() {
        url.push_str("&scope=");
        url.push_str(scope);
    }
    if !redirect_uri.is_empty() {
        url.push_str("&redirect_uri=");
        url.push_str(redirect_uri);
    }
    if !state.is_empty() {
        url.push_str("&state=");
        url.push_str(state);
    }
    url
}

/// Transform the `code` in a token, the result is [`AuthResponse`]. `code` expires so be very light on this operation. The response token will be level `client`.
///
/// `client_secret`, `client_id` and `redirect_uri` are found at <https://genius.com/api-clients>.
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
        response_type: "code".to_string(),
        grant_type: "authorization_code".to_string(),
    };
    let url = Url::parse("https://api.genius.com/oauth/token")
        .expect("Could not parse valid URL from login_with_username input.");
    let client = Client::new();
    let request = client.post(url).json(&auth_req).send().await?;
    let result = request.json::<AuthResponse>().await?;
    Ok(result)
}
