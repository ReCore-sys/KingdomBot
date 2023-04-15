use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use mongodb::Database;

use crate::db;
use crate::types::users::User;

pub(crate) async fn user_exists(uuid: String) -> Result<bool, mongodb::error::Error> {
    let db = db::get_db().await?;
    Ok(internal_user_exists(&db, uuid).await?)
}

pub(crate) async fn internal_user_exists(
    db: &Database,
    uuid: String,
) -> Result<bool, mongodb::error::Error> {
    let collection = db.collection::<User>("users");
    let filter = doc! {"uuid": uuid};
    let options = FindOptions::builder().limit(1).build();
    let cursor = collection.find(filter, options).await?;
    let all: Vec<User> = cursor.try_collect().await?;
    if all.len() == 0 {
        Ok(false)
    } else {
        Ok(true)
    }
}

pub(crate) async fn get_user(uuid: String) -> Result<User, mongodb::error::Error> {
    let db = db::get_db().await?;
    Ok(internal_get_user(&db, uuid).await?)
}

pub(crate) async fn internal_get_user(
    db: &Database,
    uuid: String,
) -> Result<User, mongodb::error::Error> {
    let collection = db.collection::<User>("users");
    let filter = doc! {"uuid": uuid};
    let options = FindOptions::builder().limit(1).build();
    let cursor = collection.find(filter, options).await?;
    let all: Vec<User> = cursor.try_collect().await?;
    Ok(all[0].clone())
}

pub(crate) async fn save_user(user: User) -> Result<(), mongodb::error::Error> {
    let db = db::get_db().await?;
    Ok(internal_save_user(&db, user).await?)
}

pub(crate) async fn internal_save_user(
    db: &Database,
    user: User,
) -> Result<(), mongodb::error::Error> {
    let collection = db.collection::<User>("users");
    if user_exists(user.uuid.clone()).await? {
        let filter = doc! {"uuid": user.uuid.clone()};
        collection.replace_one(filter, user, None).await?;
    } else {
        collection.insert_one(user, None).await?;
    }
    Ok(())
}

pub(crate) async fn get_all() -> Result<Vec<User>, mongodb::error::Error> {
    let db = db::get_db().await?;
    let collection = db.collection::<User>("users");
    let cursor = collection.find(None, None).await?;
    let all: Vec<User> = cursor.try_collect().await?;
    Ok(all)
}

pub(crate) async fn set_many(users: Vec<User>) -> Result<(), mongodb::error::Error> {
    for user in users {
        save_user(user).await?;
    }
    Ok(())
}