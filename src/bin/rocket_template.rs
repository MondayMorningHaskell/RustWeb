#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use rust_web::models::*;
use std::env;

fn local_conn_string() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("Database URL not set.")
}

fn fetch_user_by_id(database_url: &String, uid: i32) -> Option<UserEntity> {
    use rust_web::schema::users::dsl::*;
    let connection = PgConnection::establish(&database_url).expect("Error connecting to database!");
    let mut users_by_id: Vec<UserEntity> = users.filter(id.eq(uid))
        .load::<UserEntity>(&connection).expect("Error loading users");
    if users_by_id.len() == 0 {
        None
    } else {
        let first_user = users_by_id.remove(0);
        Some(first_user)
    }
}

#[get("/")]
fn index() -> Template {
    let context: HashMap<&str, &str> = [("name", "Jonathan")]
        .iter().cloned().collect();
    Template::render("index", &context)
}

#[get("/users/<uid>")]
fn get_user(uid: i32) -> Template {
    let maybe_user = fetch_user_by_id(&local_conn_string(), uid);
    let context: HashMap<&str, String> = {
        match maybe_user {
            Some(u) => [("name", u.name.clone()), ("email", u.email.clone()), ("age", u.age.to_string())]
                .iter().cloned().collect(),
            None => [("name", String::from("Unknown")), ("email", String::from("Unknown")), ("age", String::from("Unknown"))]
                .iter().cloned().collect()
        }
    };
    Template::render("user", &context)
}

fn main() {
    rocket::ignite()
        .mount("/static", StaticFiles::from("static"))
        .mount("/", routes![index, get_user])
        .attach(Template::fairing())
        .launch();
}
