use mongodb::options::ClientOptions;
use mongodb::{Client, Database};

use crate::Error;

#[path = "database/cleaners.rs"]
pub(crate) mod cleaners;
#[path = "database/factions.rs"]
pub(crate) mod factions;
#[path = "database/tiles.rs"]
#[allow(dead_code)]
pub(crate) mod tiles;
#[path = "database/users.rs"]
pub(crate) mod users;

/// Gets a MongoDB database connection
///
///
/// # Returns
/// ```Database```: The database connection
///

pub async fn get_db() -> Result<Database, mongodb::error::Error> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    client_options.connect_timeout = Some(std::time::Duration::from_secs(1));
    client_options.app_name = Some("data".to_string());
    let client = Client::with_options(client_options)?;
    Ok(client.database("data"))
}