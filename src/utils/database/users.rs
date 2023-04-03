use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::Database;
use mongodb::options::FindOptions;

use crate::db;
use crate::types::users::User;

pub(crate) async fn user_exists(uuid: String) -> bool {
    let db = db::get_db().await;
    internal_user_exists(&db, uuid).await
}

pub(crate) async fn internal_user_exists(db: &Database, uuid: String) -> bool {
    let collection = db.collection::<User>("users");
    let filter = doc! {"uuid": uuid};
    let options = FindOptions::builder().limit(1).build();
    let cursor = collection.find(filter, options).await.unwrap();
    let all: Vec<User> = cursor.try_collect().await.unwrap();
    if all.len() == 0 {
        false
    } else {
        true
    }
}

pub(crate) async fn get_user(uuid: String) -> User {
    let db = db::get_db().await;
    internal_get_user(&db, uuid).await
}

pub(crate) async fn internal_get_user(db: &Database, uuid: String) -> User {
    let collection = db.collection::<User>("users");
    let filter = doc! {"uuid": uuid};
    let options = FindOptions::builder().limit(1).build();
    let cursor = collection.find(filter, options).await.unwrap();
    let all: Vec<User> = cursor.try_collect().await.unwrap();
    all[0].clone()
}

pub(crate) async fn save_user(user: User) -> Result<(), mongodb::error::Error> {
    let db = db::get_db().await;
    internal_save_user(&db, user).await
}

pub(crate) async fn internal_save_user(
    db: &Database,
    user: User,
) -> Result<(), mongodb::error::Error> {
    let collection = db.collection::<User>("users");
    if user_exists(user.uuid.clone()).await {
        let filter = doc! {"uuid": user.uuid.clone()};
        let err = collection.replace_one(filter, user, None).await;
        match err {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    } else {
        let err = collection.insert_one(user, None).await;
        match err {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
