use futures::Future;
use std::time::Duration;
use tokio::time::sleep;
use rand::Rng;

pub async fn handle_api_ratelimit<F, Fu, V, E>(mut attempts: u8, f: F) -> Result<V, E>
where
    F: Fn() -> Fu,
    Fu: Future<Output = Result<V, E>>,
{
    loop {
        match f().await {
            Ok(v) => return Ok(v),
            Err(e) if attempts == 1 => return Err(e),
            _ => {
                attempts -= 1; 
                sleep(Duration::from_secs(rand::thread_rng().gen_range(1..4))).await;
            }
        };
    }
}