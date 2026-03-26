use std::time::Duration;

use apalis::layers::WorkerBuilderExt;
use apalis::prelude::{Monitor, WorkerBuilder};
use tokio::signal::unix::SignalKind;
use tracing::{Level, info};

use identity_core::jobs_storage;

mod config;
mod handlers;
mod ip_geo;
mod mailer;

#[tokio::main]
async fn main() {
    let tracing_level = if cfg!(debug_assertions) {
        Level::DEBUG
    } else {
        Level::INFO
    };

    tracing_subscriber::fmt().with_max_level(tracing_level).init();

    info!("Monitor starting");

    let mut sigint = tokio::signal::unix::signal(SignalKind::interrupt()).expect("Could not create sigint listener");
    let mut sigterm = tokio::signal::unix::signal(SignalKind::terminate()).expect("Could not create sigterm listener");

    let jobs_storage = jobs_storage().await;

    let finished_session_worker = |index| {
        WorkerBuilder::new(format!("finished-session-{index}"))
            .backend(jobs_storage.finished_session.clone())
            .concurrency(1)
            .enable_tracing()
            .build(handlers::finished_session)
    };

    let new_confirmation_worker = |index| {
        WorkerBuilder::new(format!("new-confirmation-{index}"))
            .backend(jobs_storage.new_confirmation.clone())
            .concurrency(1)
            .enable_tracing()
            .build(handlers::new_confirmation)
    };

    let new_session_worker = |index| {
        WorkerBuilder::new(format!("new-session-{index}"))
            .backend(jobs_storage.new_session.clone())
            .concurrency(1)
            .enable_tracing()
            .build(handlers::new_session)
    };

    let new_user_worker = |index| {
        WorkerBuilder::new(format!("new-user-{index}"))
            .backend(jobs_storage.new_user.clone())
            .concurrency(1)
            .enable_tracing()
            .build(handlers::new_user)
    };

    let password_changed_worker = |index| {
        WorkerBuilder::new(format!("password-changed-{index}"))
            .backend(jobs_storage.password_changed.clone())
            .concurrency(1)
            .enable_tracing()
            .build(handlers::password_changed)
    };

    Monitor::new()
        .register(finished_session_worker)
        .register(new_confirmation_worker)
        .register(new_session_worker)
        .register(new_user_worker)
        .register(password_changed_worker)
        .shutdown_timeout(Duration::from_millis(10000))
        .run_with_signal(async {
            info!("Monitor started");

            tokio::select! {
                _ = sigint.recv() => info!("Received SIGINT."),
                _ = sigterm.recv() => info!("Received SIGTERM."),
            };

            info!("Monitor shutting down");

            Ok(())
        })
        .await
        .expect("Monitor failed");

    info!("Monitor shutdown complete");
}
