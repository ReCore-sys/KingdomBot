use mongodb::{Client, Database};
use mongodb::options::ClientOptions;

#[path = "database/factions.rs"]
pub(crate) mod factions;
#[path = "database/tiles.rs"]
pub(crate) mod tiles;
#[path = "database/users.rs"]
pub(crate) mod users;

/// Gets a MongoDB database connection
///
///
/// # Returns
/// ```Database```: The database connection
///

pub async fn get_db() -> Database {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();
    client_options.app_name = Some("data".to_string());
    let client = Client::with_options(client_options).unwrap();
    client.database("data")
}
