use google_calendar::Client;

pub async fn do_call(code: String, state: String) {
    let client = Client::new(code, state, None, None, None);

    // Get the URL to request consent from the user.
    // You can optionally pass in scopes. If none are provided, then the
    // resulting URL will not have any scopes.
    let user_consent_url = client.user_consent_url(&["https://www.googleapis.com/auth/calendar.readonly".to_string()]);

    // In your redirect URL capture the code sent and our state.
    // Send it along to the request for the token.
    let mut access_token = client.get_access_token(&code, &state).await.unwrap();

    // You can additionally refresh the access token with the following.
    // You must have a refresh token to be able to call this function.
    access_token = client.refresh_access_token().await.unwrap();
}
