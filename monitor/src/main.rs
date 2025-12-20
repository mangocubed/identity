use std::marker::{Send, Sync};
use std::time::Duration;

use apalis::layers::{ErrorHandlingLayer, WorkerBuilderExt};
use apalis::prelude::{BoxDynError, Event, Monitor, WorkerBuilder, WorkerFactoryFn};
use tokio::signal::unix::SignalKind;
use tracing::{error, info};

use identity_core::jobs_storage::jobs_storage;

mod jobs;
mod mailer;

trait ApalisError<T> {
    fn or_apalis_error(self) -> Result<T, apalis::prelude::Error>;
}

impl<T, E: std::error::Error + Send + Sync + 'static> ApalisError<T> for Result<T, E> {
    fn or_apalis_error(self) -> Result<T, apalis::prelude::Error> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => Err(apalis::prelude::Error::from(Box::new(err) as BoxDynError)),
        }
    }
}

#[tokio::main]
async fn main() {
    info!("Monitor starting");

    let mut sigint = tokio::signal::unix::signal(SignalKind::interrupt()).expect("Could not create sigint listener");
    let mut sigterm = tokio::signal::unix::signal(SignalKind::terminate()).expect("Could not create sigterm listener");

    let jobs_storage = jobs_storage().await;

    let finished_session_worker = WorkerBuilder::new("finished-session")
        .layer(ErrorHandlingLayer::new())
        .enable_tracing()
        .backend(jobs_storage.finished_session.clone())
        .build_fn(jobs::finished_session_job);

    let new_confirmation_worker = WorkerBuilder::new("new-confirmation")
        .layer(ErrorHandlingLayer::new())
        .enable_tracing()
        .backend(jobs_storage.new_confirmation.clone())
        .build_fn(jobs::new_confirmation_job);

    let new_session_worker = WorkerBuilder::new("new-session")
        .layer(ErrorHandlingLayer::new())
        .enable_tracing()
        .backend(jobs_storage.new_session.clone())
        .build_fn(jobs::new_session_job);

    let new_user_worker = WorkerBuilder::new("new-user")
        .layer(ErrorHandlingLayer::new())
        .enable_tracing()
        .backend(jobs_storage.new_user.clone())
        .build_fn(jobs::new_user_job);

    let password_changed_worker = WorkerBuilder::new("password-changed")
        .layer(ErrorHandlingLayer::new())
        .enable_tracing()
        .backend(jobs_storage.password_changed.clone())
        .build_fn(jobs::password_changed_job);

    let refreshed_authorization_worker = WorkerBuilder::new("refreshed-authorization")
        .layer(ErrorHandlingLayer::new())
        .enable_tracing()
        .backend(jobs_storage.refreshed_authorization.clone())
        .build_fn(jobs::refreshed_authorization_job);

    let webhook_event_worker = WorkerBuilder::new("webhook-event")
        .layer(ErrorHandlingLayer::new())
        .enable_tracing()
        .backend(jobs_storage.webhook_event.clone())
        .build_fn(jobs::webhook_event_job);

    Monitor::new()
        .register(finished_session_worker)
        .register(new_confirmation_worker)
        .register(new_session_worker)
        .register(new_user_worker)
        .register(password_changed_worker)
        .register(refreshed_authorization_worker)
        .register(webhook_event_worker)
        .on_event(|e| {
            let worker_id = e.id();
            match e.inner() {
                Event::Engage(task_id) => {
                    info!("Worker [{worker_id}] got a job with id: {task_id}");
                }
                Event::Error(e) => {
                    error!("Worker [{worker_id}] encountered an error: {e}");
                }

                Event::Exit => {
                    info!("Worker [{worker_id}] exited");
                }
                Event::Idle => {
                    info!("Worker [{worker_id}] is idle");
                }
                Event::Start => {
                    info!("Worker [{worker_id}] started");
                }
                Event::Stop => {
                    info!("Worker [{worker_id}] stopped");
                }
                _ => {}
            }
        })
        .shutdown_timeout(Duration::from_millis(5000))
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
