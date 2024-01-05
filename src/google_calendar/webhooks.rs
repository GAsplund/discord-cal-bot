use tokio::task;
use localtunnel_client::{open_tunnel, broadcast, ClientConfig};
use std::{net::SocketAddr, collections::HashMap};
use axum::{response::Html, routing::{get, post}, Router, http::header::HeaderMap, extract::Query};
use uuid::Uuid;

pub fn run_webhook_server() -> String {
    // build our application with a route
    let app = Router::new()
        .route("/webhook", post(webhook_handler))
        .route("/oauth", get(oauth_handler));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    task::spawn(async move {
        println!("Webhook server is listening on http://{}/", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await.expect("Server has crashed.");
    });

    addr.to_string()
}

async fn webhook_handler(headers: HeaderMap) -> Result<String, String> {
    let resource_state = headers.get("x-goog-resource-state").expect("No resource state");
    if resource_state == "sync" {
        return Ok("Sync received".to_string());
    }

    //let resource_id = headers.get("x-goog-resource-id").expect("No resource ID");
    //let channel_id = headers.get("x-goog-channel-id").expect("No channel ID");
    let channel_token = headers.get("x-goog-channel-token").expect("No channel token");

    println!("Received webhook for id {}", channel_token.to_str().unwrap());

    let cal = crate::google_calendar::calendar::Calendar::from_id(channel_token.to_str().unwrap().to_string()).await;
    if cal.is_err() {
        return Err("No calendar found".to_string());
    }

    cal.unwrap().notify_newly_changed().await;
    
    Ok("Webhook received".to_string())
}

async fn oauth_handler(Query(params): Query<HashMap<String, String>>) -> Html<&'static str> {
    let _code = params.get("code").unwrap();
    let _state = params.get("state").unwrap();
    Html("Go back to your terminal :)")
}

pub fn run_localtunnel() -> String {
    let (notify_shutdown, _) = broadcast::channel(16);

    let uuid = Uuid::new_v4();

    let config = ClientConfig {
        server: Some("https://loca.lt".to_string()),
        subdomain: Some(uuid.to_string()),
        local_host: Some("localhost".to_string()),
        local_port: 3000,
        shutdown_signal: notify_shutdown.clone(),
        max_conn: 10,
        credential: None,
    };

    task::spawn(async move {
        println!("Opening tunnel on https://{}.loca.lt/", uuid);
        open_tunnel(config).await.expect("Failed to open tunnel");
        //let _ = notify_shutdown.send(());
    });

    format!("https://{}.loca.lt", uuid)
}
