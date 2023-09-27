use std::io::Error;

use google_calendar3::oauth2::{
    authenticator::Authenticator, hyper::client::HttpConnector, hyper_rustls::HttpsConnector,
    read_application_secret, InstalledFlowAuthenticator,
    InstalledFlowReturnMethod,
};

pub async fn authenticate() -> Result<Authenticator<HttpsConnector<HttpConnector>>, Error> {
    let secret = read_application_secret("credentials.json")
        .await
        .expect("credentials.json");

    InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
        .persist_tokens_to_disk("tokencache.json")
        .build()
        .await
}
