use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Auth {
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
/// Avaliabe scopes are `me`, `create_annotation`, `manage_annotation` and `vote`.
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
    if scope != "" {
        url.push_str("&scope=");
        url.push_str(scope);
    }
    if redirect_uri != "" {
        url.push_str("&redirect_uri=");
        url.push_str(redirect_uri);
    }
    if state != "" {
        url.push_str("&state=");
        url.push_str(state);
    }
    url
}

/// Transform the `code` in a token, the result is [`Auth`]. `code` expires so be very light on this operation.
///
/// `client_secret`, `client_id` and `redirect_uri` are found at <https://genius.com/api-clients>.
pub async fn authenticate(
    code: &str,
    client_secret: &str,
    client_id: &str,
    redirect_uri: &str,
) -> Result<Auth, reqwest::Error> {
    let form = [
        ("code", code),
        ("client_secret", client_secret),
        ("client_id", client_id),
        ("redirect_uri", redirect_uri),
        ("response_type", "code"),
        ("grant_type", "authorization_code"),
    ];
    let client = Client::new();
    let request = client
        .post("https://api.genius.com/oauth/token")
        .form(&form)
        .send()
        .await?;
    let result = request.json::<Auth>().await?;
    Ok(result)
}
