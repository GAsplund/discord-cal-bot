use localtunnel_client::{open_tunnel, broadcast, ClientConfig};
use std::net::SocketAddr;
use axum::{response::Html, routing::get, Router};
use uuid::Uuid;

pub async fn run_webhook_server() {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Webhook server is listening on http://{}/", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await.expect("Server has crashed.");
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

pub async fn run_localtunnel() {
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
    println!("Opening tunnel on https://{}.loca.lt/", uuid);
    open_tunnel(config).await.expect("Failed to open tunnel");

    let _ = notify_shutdown.send(());
}
