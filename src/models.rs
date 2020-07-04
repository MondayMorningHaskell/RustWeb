use chrono::prelude::*;
use diesel::{Queryable, Insertable};
use super::schema::*;

#[derive(Insertable)]
#[table_name="users"]
pub struct User {
    pub name: String,
    pub email: String,
    pub age: i32,
}

#[derive(Queryable)]
pub struct UserEntity {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: i32,
}

#[derive(Insertable)]
#[table_name="articles"]
pub struct Article {
    pub title: String,
    pub body: String,
    pub published_at: DateTime<Utc>,
    pub author_id: i32,
}

#[derive(Queryable)]
pub struct ArticleEntity {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published_at: DateTime<Utc>,
    pub author_id: i32,
}
