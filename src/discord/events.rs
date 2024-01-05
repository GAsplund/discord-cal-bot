use google_calendar3::api::Event;
use serenity::model::prelude::ChannelId;
use crate::database;

use super::bot::get_http;

pub enum EventType {
    Create,
    Update,
    Delete,
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let string_thing = match self {
            EventType::Create => "created",
            EventType::Update => "updated",
            EventType::Delete => "deleted",
        };
        write!(f, "{}", string_thing)
    }
}

pub fn get_state(event: Event) -> EventType {
    if event.status.expect("Could not get event status").as_str() == "cancelled" {
        return EventType::Delete;
    }
    
    let create_edit_delta = event.updated.unwrap().signed_duration_since(event.created.unwrap());
    if create_edit_delta.num_seconds() < 1 {
        return EventType::Create;
    }

    return EventType::Update;
}

pub async fn remind_event(event: Event, calendar_id: String, ping: u64) {
    let channels = database::get_notify_channels_for_event(calendar_id);

    for channel in channels {
        let msg_chan = ChannelId(channel);
        let msg = format!("<@{}> {} is occurring soon!", ping, event.clone().summary.unwrap());

        let http = get_http().expect("Couldn't get HTTP client");
        let _ = msg_chan.say(http, msg).await;
    }
}

pub async fn notify_event(event: Event, calendar_id: String) {
    let channels = database::get_notify_channels_for_event(calendar_id);

    for channel in channels {
        let msg_chan = ChannelId(channel);
        let msg = format!("Event {} has been {}", event.clone().summary.unwrap(), get_state(event.clone()));

        let http = get_http().expect("Couldn't get HTTP client");
        let _ = msg_chan.say(http, msg).await;
    }
}
