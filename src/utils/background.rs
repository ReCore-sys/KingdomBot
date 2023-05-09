use std::collections::HashMap;
use std::time::Duration;

use tokio::time::{sleep, Instant};

use crate::db;

pub async fn background_loop() {
    let mut wait_times: HashMap<&str, Instant> = HashMap::new();
    wait_times.insert("production", Instant::now());
    wait_times.insert("clean", Instant::now());
    wait_times.insert("economy", Instant::now());
    sleep(Duration::from_millis(500)).await;
    loop {
        if Instant::now()
            .duration_since(wait_times["production"])
            .as_secs()
            >= 10
        {
            wait_times.insert("production", Instant::now());
            trace!("Updating production stats");
            db::build_production().await.unwrap();
        }

        if Instant::now().duration_since(wait_times["clean"]).as_secs() >= 30 {
            wait_times.insert("clean", Instant::now());
            trace!("Cleaning database");
            db::cleaners::clean_factions().await.unwrap();
            db::cleaners::clean_tiles().await.unwrap();
            db::cleaners::clean_users().await.unwrap();
        }

        if Instant::now()
            .duration_since(wait_times["economy"])
            .as_secs()
            >= 7
        {
            wait_times.insert("economy", Instant::now());
            trace!("Updating economy");
            db::update_economy().await.unwrap();
        }
    }
}