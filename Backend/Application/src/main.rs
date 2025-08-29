use std::net::TcpListener;

use application::{
    config::{get_config, load_settings},
    routes::routes_config,
    startup::run,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_settings().await;
    let url = get_config().await.url;
    let port = get_config().await.port;

    let listener =
        TcpListener::bind(format!("{url}:{port}")).expect("Failed to bind local address");

    run(
        listener,
        routes_config,
        get_config().await.environment,
        "Api",
    )?
    .await
}
