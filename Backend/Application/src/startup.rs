use std::{env, net::TcpListener};

use actix_session::{
    config::{CookieContentSecurity, PersistentSession},
    storage::CookieSessionStore,
    SessionMiddleware,
};
use actix_web::{
    cookie::{time::Duration, Key},
    dev::Server,
    web, App, HttpServer,
};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

use crate::environments::Environments;

pub fn run(
    listener: TcpListener,
    routes_config: fn(&mut web::ServiceConfig),
    environtment: Environments,
    module_name: &str,
) -> Result<Server, std::io::Error> {
    let secure = match environtment {
        Environments::PRO => true,
        _ => false,
    };

    setup_logger(module_name).expect("Failed to set up logger");
    let socket_addr = listener.local_addr().expect("Failed to get local address");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_content_security(CookieContentSecurity::Signed)
                    .cookie_name(String::from("TB"))
                    .cookie_http_only(secure)
                    .cookie_secure(secure)
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(Duration::seconds(604_800)),
                    )
                    .build(),
            )
            .app_data(web::PayloadConfig::new(1024 * 1024 * 1024))
            .app_data(web::JsonConfig::default().limit(1024 * 1024 * 1024))
            .configure(routes_config)
    })
    .listen(listener)?
    .run();

    dbg!(format!("Program started on {socket_addr}"));

    Ok(server)
}

fn setup_logger(module_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = env::current_dir()?;

    let rolling_file = tracing_appender::rolling::daily(current_dir.join("Logs"), module_name);

    let formatting_layer = BunyanFormattingLayer::new(module_name.into(), rolling_file);

    let env_filter = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("warn"))?;

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}
