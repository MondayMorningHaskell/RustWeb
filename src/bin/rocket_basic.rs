#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket::http::RawStr;
use rocket_contrib::json::Json;

#[get("/hello")]
fn index() -> String {
    String::from("Hello, world!")
}

#[get("/hello/<name>")]
fn hello(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}

#[get("/math/<first>/<second>")]
fn add(first: i32, second: i32) -> String {
    String::from(format!("{}", first + second))
}

#[get("/math?<first>&<second>")]
fn multiply(first: i32, second: i32) -> String {
    String::from(format!("{}", first * second))
}

#[derive(FromForm, Deserialize, Serialize)]
struct User {
    name: String,
    email: String,
    age: i32
}

#[post("/users/create", format= "application/json", data = "<user>")]
fn create_user(user: Json<User>) -> String {
    String::from(format!("Created user: {} {} {} {}", 1, user.name, user.email, user.age))
}

fn main() {
    rocket::ignite().mount("/", routes![index, hello, add, multiply, create_user]).launch();
}