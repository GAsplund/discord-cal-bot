extern crate google_calendar3 as calendar3;

use std::default::Default;

use axum::body::Body;
use axum::http::Response;
use calendar3::api::{Channel, Events};
use calendar3::oauth2::{
    authenticator::Authenticator, hyper::client::HttpConnector, hyper_rustls::HttpsConnector,
};
use calendar3::{chrono, hyper, hyper_rustls, CalendarHub, Error};

use uuid::Uuid;

pub async fn get_cal(
    auth: Authenticator<HttpsConnector<HttpConnector>>,
) -> CalendarHub<HttpsConnector<HttpConnector>> {
    CalendarHub::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .build(),
        ),
        auth,
    )
}

pub async fn watch(hub: CalendarHub<HttpsConnector<HttpConnector>>, uri: &str, calendar_id: &str) {
    let mut req = Channel::default();
    req.address = Some(uri.to_string());
    req.id = Some(Uuid::new_v4().to_string());
    req.type_ = Some("webhook".to_string());

    let result = hub
        .events()
        .watch(req, calendar_id)
        .updated_min(chrono::Utc::now())
        .time_min(chrono::Utc::now())
        .single_events(true)
        .show_hidden_invitations(false)
        .show_deleted(true)
        .always_include_email(false)
        .doit()
        .await;

    match result {
        Err(e) => match e {
            // The Error enum provides details about what exactly happened.
            // You can also just use its `Debug`, `Display` or `Error` traits
            Error::HttpError(_)
            | Error::Io(_)
            | Error::MissingAPIKey
            | Error::MissingToken(_)
            | Error::Cancelled
            | Error::UploadSizeLimitExceeded(_, _)
            | Error::Failure(_)
            | Error::BadRequest(_)
            | Error::FieldClash(_)
            | Error::JsonDecodeError(_, _) => println!("{}", e),
        },
        Ok(res) => println!("Success: {:?}", res),
    }
}

pub async fn get_newly_changed(
    hub: CalendarHub<HttpsConnector<HttpConnector>>,
    calendar_id: &str,
) -> Result<(Response<Body>, Events), calendar3::Error> {
    hub.events()
        .list(calendar_id)
        .updated_min(chrono::Utc::now() - chrono::Duration::minutes(1))
        .show_deleted(true)
        .doit()
        .await
}

pub async fn get_upcoming(
    hub: CalendarHub<HttpsConnector<HttpConnector>>,
    calendar_id: &str,
) -> Result<(Response<Body>, Events), calendar3::Error> {
    hub.events()
        .list(calendar_id)
        .time_min(chrono::Utc::now())
        .time_max(chrono::Utc::now() + chrono::Duration::days(7))
        .show_deleted(true)
        .doit()
        .await
}
