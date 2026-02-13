use std::time::Duration;

pub async fn sleep(millis: u64) {
    let duration = Duration::from_millis(millis);

    gloo_timers::future::sleep(duration).await;
}
