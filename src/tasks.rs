use tokio::sync::Mutex;
use std::{collections::BTreeMap, time::{Instant, Duration}};

use crate::{google_calendar::calendar::{WATCHES, CALENDARS}, discord::events::remind_event};

pub static NOTIFIED: Mutex<BTreeMap<String, Instant>> = Mutex::const_new(BTreeMap::new());

pub async fn update_watches() {
    println!("Updating watched calendars");
    let mut watches = WATCHES.lock().await;
    let watches_checks = watches.clone();
    for (calendar_id, watch_time) in watches_checks.iter() {
        if watch_time.elapsed().as_secs() > 3600 {
            println!("Refreshing watch for {}", calendar_id);
            watches.remove(calendar_id);
        }
    }
}

pub async fn update_events() {
    println!("Updating calendar events");
    let mut notified = NOTIFIED.lock().await;
    let cals = CALENDARS.lock().await;
    let notified_checks = notified.clone();
    for (calendar_id, cal) in cals.iter() {
        let upcoming = cal.get_upcoming().await.unwrap();
        for event in upcoming.1.items.unwrap().iter() {
            if event.start.as_ref().unwrap().date_time.is_some() {
                let start = event.start.as_ref().unwrap().date_time.as_ref().unwrap().clone();
                let now = google_calendar3::chrono::Utc::now();
                let delta = start.signed_duration_since(now);
                let id = event.id.as_ref().unwrap();
                if notified.get(id).is_none() && delta.num_seconds() < 3600 && delta.num_seconds() > 0 {
                    let _ = remind_event(event.clone(), calendar_id.clone(), 182891432166555649).await;
                    let week = Duration::from_secs(60 * 60 * 24 * 7);
                    notified.insert(id.clone(), Instant::now() + week);
                }
            }
        };
    }

    for (calendar_id, notify_time) in notified_checks.iter() {
        if notify_time.elapsed().as_secs() > 3600 {
            notified.remove(calendar_id);
        }
    }
}
