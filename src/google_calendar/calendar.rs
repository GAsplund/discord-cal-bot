extern crate google_calendar3 as calendar3;

use std::default::Default;

use axum::body::Body;
use axum::http::Response;
use calendar3::api::{Channel, Events};
use calendar3::oauth2::{
    authenticator::Authenticator, hyper::client::HttpConnector, hyper_rustls::HttpsConnector,
};
use calendar3::{chrono, hyper, hyper_rustls, CalendarHub};

use uuid::Uuid;

use crate::discord;
use std::collections::BTreeMap;
use std::sync::Mutex;

pub static CALS: Mutex<BTreeMap<String, Calendar>> = Mutex::new(BTreeMap::new());

#[derive(Clone)]
pub struct Calendar {
    hub: CalendarHub<HttpsConnector<HttpConnector>>,
    calendar_id: String,
}

impl Calendar {
    pub async fn from_auth(
        auth: Authenticator<HttpsConnector<HttpConnector>>,
        calendar_id: String,
    ) -> Option<Calendar> {
        let hub = CalendarHub::new(
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_or_http()
                    .enable_http1()
                    .build(),
            ),
            auth,
        );

        let mut cal_id = calendar_id.clone();
        if cal_id == "primary" {
            let res = hub.calendars().get(&calendar_id).doit().await;
            match res {
                Ok(calendar) => {
                    cal_id = calendar.1.id.expect("Could not get calendar id");
                }
                Err(_) => {
                    return None;
                }
            }
        }

        let cal = Calendar { hub, calendar_id: cal_id.clone() };
        CALS.lock().expect("Calendars mutex poisoned").insert(cal_id, cal.clone());

        Some(cal)
    }

    pub fn from_id(id: String) -> Result<Calendar, String> {
        let cals = CALS.lock();
        if cals.is_err() {
            return Err("Calendars mutex poisoned".to_string());
        }

        let cals = cals.unwrap();
        let cal = cals.get(&id);
        if cal.is_none() {
            return Err("No calendar found".to_string());
        }
        
        Ok(cal.unwrap().clone())
    }

    pub async fn watch(&self, uri: &str) -> Result<(Response<Body>, Channel), calendar3::Error> {
        let mut req = Channel::default();
        req.address = Some(uri.to_string());
        req.id = Some(Uuid::new_v4().to_string());
        req.type_ = Some("webhook".to_string());
        req.token = Some(self.calendar_id.clone());

        self.hub.events().watch(req, &self.calendar_id).doit().await
    }

    pub async fn get_newly_changed(&self) -> Result<(Response<Body>, Events), calendar3::Error> {
        self.hub
            .events()
            .list(&self.calendar_id)
            .updated_min(chrono::Utc::now() - chrono::Duration::minutes(1))
            .show_deleted(true)
            .doit()
            .await
    }

    pub async fn notify_newly_changed(&self) {
        let (_, events) = self.get_newly_changed().await.expect("Couldn't get events");
        for event in events.items.unwrap() {
            discord::events::notify_event(event, self.calendar_id.clone()).await;
        }
    }

    pub async fn get_upcoming(&self) -> Result<(Response<Body>, Events), calendar3::Error> {
        self.hub
            .events()
            .list(&self.calendar_id)
            .time_min(chrono::Utc::now())
            .time_max(chrono::Utc::now() + chrono::Duration::days(7))
            .show_deleted(true)
            .doit()
            .await
    }
}

pub async fn list_calendars(auth: Authenticator<HttpsConnector<HttpConnector>>) -> Vec<(String, String)> {
    let hub = CalendarHub::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .build(),
        ),
        auth,
    );

    hub.calendar_list().list().doit().await.expect("Couldn't get calendars").1.items.unwrap().iter().map(|cal| {
        (cal.id.clone().expect("Couldn't get calendar id"), cal.summary.clone().expect("Couldn't get calendar summary"))
    }).collect()
}
