use std::collections::BTreeMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use serenity::builder::CreateComponents;
use serenity::model::application::interaction::{
    message_component::MessageComponentInteraction, InteractionResponseType,
};

use serenity::{
    builder::CreateSelectMenu,
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
};
use tokio::time::timeout;
use yup_oauth2::authenticator::Authenticator;

use crate::google_calendar::calendar::{CALENDARS, CHANNEL_CALENDARS, WATCHES};
use crate::google_calendar::{calendar::list_calendars, google_oauth};
use crate::WEBHOOK_URL;

pub static PENDING_AUTHENTICATIONS: Mutex<
    BTreeMap<
        (u64, u64),
        Authenticator<
            yup_oauth2::hyper_rustls::HttpsConnector<yup_oauth2::hyper::client::HttpConnector>,
        >,
    >,
> = Mutex::new(BTreeMap::new());

#[group]
#[commands(add_calendar)]
pub struct Calendar;

#[command]
pub async fn add_calendar(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Please authenticate through the console.")
        .await?;
    println!("Authenticating at {}", &msg.author.id.to_string());
    let auth = timeout(
        Duration::from_secs(60),
        google_oauth::authenticate(&msg.author.id.to_string(), msg.author.id.0),
    )
    .await;

    match auth {
        Ok(auth) => {
            let auth = auth.expect("Failed to authenticate");
            let auth_stored = auth.clone();
            let cals = list_calendars(auth).await;

            let menu: CreateSelectMenu = CreateSelectMenu::default()
                .custom_id("calendar_select")
                .placeholder("Please select a calendar")
                .min_values(1)
                .max_values(1)
                .options(|o| {
                    for cal in cals {
                        o.create_option(|opt| opt.label(cal.1).value(cal.0));
                    }
                    o
                })
                .to_owned();
            let _ = msg
                .channel_id
                .send_message(&ctx.http, |m| {
                    m.components(|c| c.create_action_row(|row| row.add_select_menu(menu)))
                })
                .await
                .and_then(|_| {
                    PENDING_AUTHENTICATIONS
                        .lock()
                        .expect("Pending authentications mutex poisoned")
                        .insert((msg.author.id.0, msg.channel_id.0), auth_stored.into());
                    Ok(())
                });
        }
        Err(_) => {
            let _ = msg.reply(ctx, "Authentication timed out.").await;
        }
    }

    Ok(())
}

pub async fn bind_calendar_interaction(ctx: &Context, interaction: MessageComponentInteraction) {
    let choice = &interaction.data.values[0];
    let auth = PENDING_AUTHENTICATIONS
        .lock()
        .expect("Pending authentications mutex poisoned")
        .remove(&(interaction.user.id.0, interaction.channel_id.0))
        .unwrap();
    let cal =
        crate::google_calendar::calendar::Calendar::from_auth(auth.clone(), choice.to_string())
            .await
            .unwrap();
    let _ = cal.watch(WEBHOOK_URL.get().unwrap()).await;
    let week = Duration::from_secs(60 * 60 * 24 * 7);
    CALENDARS
        .lock()
        .await
        .insert(choice.to_string(), cal.clone());
    CHANNEL_CALENDARS
        .lock()
        .await
        .insert(choice.to_string(), interaction.channel_id.0);
    WATCHES
        .lock()
        .await
        .insert(choice.to_string(), Instant::now() + week);

    let _ = interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::UpdateMessage)
                .interaction_response_data(|message| {
                    message
                        .content(format!("You selected {}", choice))
                        .set_components(CreateComponents::default())
                })
        })
        .await;
}
