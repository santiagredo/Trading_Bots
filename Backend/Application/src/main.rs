use application::{handler::Engines, startup::run};
use models::enums::LifecycleState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = run()?;
    let handle = server.handle();

    Engines::new(LifecycleState::Running)
        .set_engine_status()
        .await
        .unwrap();

    let cancel_token = Engines::default().get_engine_token();

    tokio::select! {
        _ = server => {
            dbg!("Server exited unexpectedly");
        }
        _ = cancel_token.cancelled() => {
            dbg!("Shutdown requested");
            handle.stop(true).await;
        }
    }

    dbg!("Engine stopped cleanly");
    Ok(())
}
