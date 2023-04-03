use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::Database;
use mongodb::options::FindOptions;

use crate::db;
use crate::types::factions::Faction;

pub(crate) async fn get_faction(tag: String) -> Faction {
    let db = db::get_db().await;
    internal_get_faction(&db, tag).await
}

pub(crate) async fn internal_get_faction(db: &Database, tag: String) -> Faction {
    let collection = db.collection::<Faction>("factions");
    let filter = doc! {"tag": tag};
    let options = FindOptions::builder().limit(1).build();
    let cursor = collection.find(filter, options).await.unwrap();
    let all: Vec<Faction> = cursor.try_collect().await.unwrap();
    all[0].clone()
}

pub(crate) async fn faction_exists(tag: String) -> bool {
    let db = db::get_db().await;
    internal_faction_exists(&db, tag).await
}

pub(crate) async fn internal_faction_exists(db: &Database, tag: String) -> bool {
    let collection = db.collection::<Faction>("factions");
    let filter = doc! {"tag": tag};
    let options = FindOptions::builder().limit(1).build();
    let cursor = collection.find(filter, options).await.unwrap();
    let all: Vec<Faction> = cursor.try_collect().await.unwrap();
    if all.len() == 0 {
        false
    } else {
        true
    }
}

pub(crate) async fn save_faction(faction: Faction) -> Result<(), mongodb::error::Error> {
    let db = db::get_db().await;
    internal_save_faction(&db, faction).await
}

pub(crate) async fn internal_save_faction(
    db: &Database,
    faction: Faction,
) -> Result<(), mongodb::error::Error> {
    let collection = db.collection::<Faction>("factions");
    if faction_exists(faction.tag.clone()).await {
        let filter = doc! {"tag": faction.tag.clone()};
        let err = collection.replace_one(filter, faction, None).await;
        match err {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    } else {
        let err = collection.insert_one(faction, None).await;
        match err {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
