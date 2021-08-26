use anyhow::Result;
use celery::beat::DeltaSchedule;
use celery::prelude::*;
use commons::{config, tasks};
use dotenv::dotenv;
use tokio::time::Duration;

// Tasks on the producer side don't need to know the implementation of what the consumer
// runs, only the method signature. Therefore, methods should be declared but the body
// can be left as `unimplemented!()
// Reference: fn add(x: i32, y: i32) -> TaskResult<i32> {

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let mut beat = celery::beat!(
        broker = RedisBroker { config::redis_uri() },
        tasks = [
            "add" => {
                tasks::add::add,
                schedule = DeltaSchedule::new(Duration::from_secs(3)),
                args = (1, 2),
            },
            "check_incoming_eth" => {
                tasks::logs::check_incoming_eth,
                schedule = DeltaSchedule::new(Duration::from_secs(5)),
                args = ("0xd6f5Bef6bb4acD235CF85c0ce196316d10785d67".to_string(),),
            }
        ],
        task_routes = [
            "*" => tasks::QUEUE_NAME,
        ],
    )
    .await?;

    beat.start().await?;
    Ok(())
}
