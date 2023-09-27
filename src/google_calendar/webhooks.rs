use tokio::task;
use localtunnel_client::{open_tunnel, broadcast, ClientConfig};
use std::{net::SocketAddr, collections::HashMap};
use axum::{response::Html, routing::get, Router, http::header::HeaderMap, extract::Query};
use uuid::Uuid;

pub fn run_webhook_server() -> String {
    // build our application with a route
    let app = Router::new()
        .route("/webhook", get(webhook_handler))
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

async fn webhook_handler(headers: HeaderMap) -> Html<&'static str> {
    let _resource_id = headers.get("x-goog-resource-id").unwrap();
    let _channel_token = headers.get("x-goog-channel-token").unwrap();
    let _channel_id = headers.get("x-goog-channel-id").unwrap();
    let _resource_state = headers.get("x-goog-resource-state").unwrap();
    Html("<h1>Hello, World!</h1>")
}

async fn oauth_handler(Query(params): Query<HashMap<String, String>>) -> Html<&'static str> {
    let _code = params.get("code").unwrap();
    let _state = params.get("state").unwrap();
    Html("Go back to your terminal :)")
}

pub fn run_localtunnel() -> String {
    let (notify_shutdown, _) = broadcast::channel(1);

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
        let _ = notify_shutdown.send(());
    });

    uuid.to_string()
}
