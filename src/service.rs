use crate::models;
use sea_orm::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserID {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullUser {
    pub id: i32,
    pub name: String,
    pub email: String,
}

pub async fn put_user(user: User, db: &DatabaseConnection) -> Result<(), DbErr> {
    let new_user = models::ActiveModel {
        name: ActiveValue::set(user.name),
        email: ActiveValue::set(user.email),
        ..Default::default()
    };
    models::Entity::insert(new_user).exec(db).await.map(|_| ())
}

pub async fn get_user(id: i32, db: &DatabaseConnection) -> Result<Option<User>, DbErr> {
    models::Entity::find_by_id(id).one(db).await.map(|r| {
        r.map(|user| User {
            name: user.name,
            email: user.email,
        })
    })
}

pub async fn get_all_users(db: &DatabaseConnection) -> Result<Vec<models::Model>, DbErr> {
    models::Entity::find().all(db).await
}

pub async fn delete_user(id: i32, db: &DatabaseConnection) -> Result<(), DbErr> {
    models::ActiveModel {
        id: ActiveValue::set(id),
        ..Default::default()
    }
    .delete(db)
    .await
    .map(|_| ())
}
