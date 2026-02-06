use application::{handler::Engines, startup::run};
use models::enums::LifecycleState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = run()?;
    let handle = server.handle();

    Engines::blank()
        .set_engine_state(LifecycleState::Running)
        .await
        .unwrap();

    let cancel_token = Engines::blank().get_engine_token_core();

    tokio::select! {
        _ = server => {
            dbg!("Server exited unexpectedly");
        }
        _ = cancel_token.cancelled() => {
            dbg!("Shutdown requested");
            handle.stop(true).await;
            dbg!("Engine stopped cleanly");
        }
    }

    Ok(())
}
