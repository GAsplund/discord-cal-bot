use std::time::Duration;

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

use crate::google_calendar::{calendar::list_calendars, google_oauth};

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
                .await;
        }
        Err(_) => {
            let _ = msg.reply(ctx, "Authentication timed out.").await;
        }
    }

    Ok(())
}

pub async fn bind_calendar_interaction(ctx: &Context, interaction: MessageComponentInteraction) {
    let choice = &interaction.data.values[0];

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
