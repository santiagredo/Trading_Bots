use std::net::TcpListener;

use application::{
    config::{get_config, load_settings},
    routes::routes_config,
    startup::run, types::Executor,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8082").expect("Failed to bind local address");
    let app_port = listener.local_addr().unwrap().port();
    println!("Program started on port: {} \n", app_port);

    load_settings().await;

    Executor::run_tasks().await;

    run(
        listener,
        routes_config,
        get_config().await.environment,
        "Api",
    )?
    .await
}
