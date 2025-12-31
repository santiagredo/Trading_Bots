use application::{cache::configurations::ACTIVE_SHUTDOWN, startup::run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = run()?;
    let handle = server.handle();

    tokio::select! {
        _ = server => {
            dbg!("Server exited unexpectedly");
        }
        _ = ACTIVE_SHUTDOWN.notified() => {
            dbg!("Shutdown requested");
            handle.stop(true).await;
        }
    }

    dbg!("Engine stopped cleanly");
    Ok(())
}
