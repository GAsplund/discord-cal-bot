use std::{future::Future, io::Error, pin::Pin};

use google_calendar3::oauth2::{
    authenticator::Authenticator, hyper::client::HttpConnector, hyper_rustls::HttpsConnector,
    InstalledFlowAuthenticator, InstalledFlowReturnMethod,
};
use serenity::model::id::UserId;
use yup_oauth2::authenticator_delegate::{DefaultInstalledFlowDelegate, InstalledFlowDelegate};

use crate::discord::bot::get_http;

pub async fn authenticate(
    auth_id: &str,
    send_channel: u64,
) -> Result<Authenticator<HttpsConnector<HttpConnector>>, Error> {
    let secret = yup_oauth2::read_application_secret("./auth/credentials.json")
        .await
        .expect("client secret couldn't be read.");

    InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
        .persist_tokens_to_disk(format!("./auth/token_cache/{}.json", auth_id))
        .flow_delegate(Box::new(InstalledFlowBrowserDelegate {
            channel: send_channel,
        }))
        .build()
        .await
}

async fn discord_send_url(url: &str, need_code: bool, channel: u64) -> Result<String, String> {
    println!("Test!");
    let msg = format!(
        "Please open the following link in your browser to authenticate:\n{}",
        url
    );

    let http = get_http().expect("Couldn't get HTTP client");
    let user = UserId(channel).to_user(&http).await;
    let _ = user
        .unwrap()
        .direct_message(&http, |m| m.content(msg))
        .await;

    let def_delegate = DefaultInstalledFlowDelegate;
    def_delegate.present_user_url(url, need_code).await
}

#[derive(Copy, Clone)]
struct InstalledFlowBrowserDelegate {
    channel: u64,
}

impl InstalledFlowDelegate for InstalledFlowBrowserDelegate {
    fn present_user_url<'a>(
        &'a self,
        url: &'a str,
        need_code: bool,
    ) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send + 'a>> {
        Box::pin(discord_send_url(url, need_code, self.channel))
    }
}
