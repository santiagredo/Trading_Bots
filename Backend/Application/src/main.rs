use application::{cache::engines::ACTIVE_SHUTDOWN, handler::Engines, startup::run};
use models::enums::LifecycleState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = run()?;
    let handle = server.handle();

    Engines::new(LifecycleState::Running)
        .set_engine_status()
        .await
        .unwrap();

    tokio::select! {
        _ = server => {
            dbg!("Server exited unexpectedly");
        }
        _ = ACTIVE_SHUTDOWN.notified() => {

            Engines::new(LifecycleState::Stopping)
                .set_engine_status()
                .await
                .unwrap();

            dbg!("Shutdown requested");
            handle.stop(true).await;
        }
    }

    dbg!("Engine stopped cleanly");
    Ok(())
}
