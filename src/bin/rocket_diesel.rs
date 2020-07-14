#![feature(proc_macro_hygiene, decl_macro)]

extern crate diesel;
extern crate rust_web;
#[macro_use] extern crate rocket;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use rocket::State;
use rocket_contrib::json::Json;
use std::env;

use rust_web::models::*;
use rust_web::schema::*;

fn local_conn_string() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("Database URL not set.")
}

#[get("/users/all")]
fn fetch_all_users(database_url: State<String>) -> Json<Vec<UserEntity>> {
    use rust_web::schema::users::dsl::*;
    let connection = PgConnection::establish(&database_url).expect("Error connecting to database!");
    Json(users.load::<UserEntity>(&connection).expect("Error loading users"))
}

#[get("/users/<uid>")]
fn fetch_user(database_url: State<String>, uid: i32) -> Option<Json<UserEntity>> {
    use rust_web::schema::users::dsl::*;
    let connection = PgConnection::establish(&database_url).expect("Error connecting to database!");
    let mut users_by_id: Vec<UserEntity> = users.filter(id.eq(uid))
        .load::<UserEntity>(&connection).expect("Error loading users");
    if users_by_id.len() == 0 {
        None
    } else {
        let first_user = users_by_id.remove(0);
        Some(Json(first_user))
    }
}

#[post("/users/create", format="application/json", data = "<user>")]
fn create_user(database_url: State<String>, user: Json<User>) -> Json<i32> {
    let connection = PgConnection::establish(&database_url).expect("Error connecting to database!");
    let user_entity: UserEntity = diesel::insert_into(users::table)
        .values(&*user)
        .get_result(&connection).expect("Error saving user");
    Json(user_entity.id)
}

#[put("/users/<uid>/update", format="application/json", data="<user>")]
fn update_user(database_url: State<String>, uid: i32, user: Json<User>) -> Json<UserEntity> {
    use rust_web::schema::users::dsl::*;
    let connection = PgConnection::establish(&database_url).expect("Error connecting to database!");
    let updated_user: UserEntity = diesel::update(users.filter(id.eq(uid)))
        .set((name.eq(&user.name), email.eq(&user.email), age.eq(user.age)))
        .get_result::<UserEntity>(&connection).expect("Error updating user");
    Json(updated_user)
}

#[delete("/users/<uid>")]
fn delete_user(database_url: State<String>, uid: i32) -> Json<i32> {
    use rust_web::schema::users::dsl::*;
    let connection = PgConnection::establish(&database_url).expect("Error connecting to database!");
    diesel::delete(users.filter(id.eq(uid))).execute(&connection).expect("Error deleting user");
    Json(uid)
}

fn main () {
    rocket::ignite().mount("/", routes![fetch_all_users, fetch_user, create_user, delete_user, update_user]).manage(local_conn_string()).launch();
}