#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

extern crate crypto;

use crypto::digest::Digest;
use crypto::sha3::Sha3;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use rocket::http::{Status, Cookies, Cookie};
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket::State;
use rocket_contrib::json::Json;
use std::env;

use rust_web::models::*;
use rust_web::schema::*;

fn local_conn_string() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("Database URL not set.")
}

#[derive(FromForm, Deserialize)]
struct CreateInfo {
    name: String,
    email: String,
    age: i32,
    password: String
}

#[derive(FromForm, Deserialize)]
struct LoginInfo {
    username: String,
    password: String,
}

#[derive(Debug)]
enum LoginError {
    InvalidData,
    UsernameDoesNotExist,
    WrongPassword
}

struct AuthenticatedUser {
    user_id: i32
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthenticatedUser {
    type Error = LoginError;
    fn from_request(request: &'a Request<'r>) -> Outcome<AuthenticatedUser, LoginError> {
        let username = request.headers().get_one("username");
        let password = request.headers().get_one("password");
        match (username, password) {
            (Some(u), Some(p)) => {
                let conn_str = local_conn_string();
                let maybe_user = fetch_user_by_email(&conn_str, &String::from(u));
                match maybe_user {
                    Some(user) => {
                        let maybe_auth_info = fetch_auth_info_by_user_id(&conn_str, user.id);
                        match maybe_auth_info {
                            Some(auth_info) => {
                                let hash = hash_password(&String::from(p));
                                if hash == auth_info.password_hash {
                                    Outcome::Success(AuthenticatedUser{user_id: user.id})
                                } else {
                                    Outcome::Failure((Status::Forbidden, LoginError::WrongPassword))
                                }
                            }
                            None => {
                                Outcome::Failure((Status::MovedPermanently, LoginError::WrongPassword))
                            }
                        }
                    }
                    None => Outcome::Failure((Status::NotFound, LoginError::UsernameDoesNotExist))
                }
            },
            _ => Outcome::Failure((Status::BadRequest, LoginError::InvalidData))
        }
    }
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

fn fetch_auth_info_by_user_id(database_url: &String, uid: i32) -> Option<AuthInfoEntity> {
    use rust_web::schema::auth_infos::dsl::*;
    let connection = PgConnection::establish(&database_url).expect("Error connecting to database!");
    let mut auth_infos_by_uid: Vec<AuthInfoEntity> = auth_infos.filter(user_id.eq(uid))
        .load::<AuthInfoEntity>(&connection).expect("Error loading auth infos");
    if auth_infos_by_uid.len() == 0 {
        None
    } else {
        let first_info = auth_infos_by_uid.remove(0);
        Some(first_info)
    }
}

fn fetch_user_by_email(database_url: &String, input_email: &String) -> Option<UserEntity> {
    use rust_web::schema::users::dsl::*;
    let connection = PgConnection::establish(&database_url).expect("Error connecting to database!");
    let mut users_by_id: Vec<UserEntity> = users.filter(email.eq(input_email))
        .load::<UserEntity>(&connection).expect("Error loading user");
    if users_by_id.len() == 0 {
        None
    } else {
        let first_user = users_by_id.remove(0);
        Some(first_user)
    }
}

fn hash_password(password: &String) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str(password);
    hasher.result_str()
}

#[post("/users/create", format="json", data="<create_info>")]
fn create(database_url: State<String>, create_info: Json<CreateInfo>) -> Json<i32> {
    let user: User = User {name: create_info.name.clone(), email: create_info.email.clone(), age: create_info.age};
    let connection = PgConnection::establish(&database_url).expect("Error connecting to database!");
    let user_entity: UserEntity = diesel::insert_into(users::table)
        .values(user)
        .get_result(&connection).expect("Error saving user");

    let password_hash = hash_password(&create_info.password);
    let auth_info: AuthInfo = AuthInfo {user_id: user_entity.id, password_hash: password_hash};
    let auth_info_entity: AuthInfoEntity = diesel::insert_into(auth_infos::table)
        .values(auth_info)
        .get_result(&connection).expect("Error saving auth info");
    Json(user_entity.id)
}

#[get("/users/<uid>")]
fn login(database_url: State<String>, user: AuthenticatedUser, uid: i32) -> Json<Option<UserEntity>> {
    Json(fetch_user_by_id(&database_url, uid))
}

#[post("/users/login", format="json", data="<login_info>")]
fn login_post(database_url: State<String>, login_info: Json<LoginInfo>, mut cookies: Cookies) -> Json<Option<i32>> {
    let maybe_user = fetch_user_by_email(&database_url, &login_info.username);
    match maybe_user {
        Some(user) => {
            let maybe_auth = fetch_auth_info_by_user_id(&database_url, user.id);
            match maybe_auth {
                Some(auth_info) => {
                    let hash = hash_password(&login_info.password);
                    if hash == auth_info.password_hash {
                        cookies.add_private(Cookie::new("user_id", user.id.to_string()));
                        Json(Some(user.id))
                    } else {
                        Json(None)
                    }
                }
                None => Json(None)
            }
        }
        None => Json(None)
    }
}

#[post("/users/logout", format="json")]
fn logout(mut cookies: Cookies) -> () {
    cookies.remove_private(Cookie::named("user_id"));
}

#[get("/users/cookies/<uid>")]
fn fetch_special(database_url: State<String>, uid: i32, mut cookies: Cookies) -> Json<Option<UserEntity>> {
  // Run matching on cookies and return info
    let logged_in_user = cookies.get_private("user_id");
    match logged_in_user {
        Some(c) => {
            let logged_in_uid = c.value().parse::<i32>().unwrap();
            if logged_in_uid == uid {
                Json(fetch_user_by_id(&database_url, uid))
            } else {
                Json(None)
            }
        },
        None => Json(None)
    }
}

fn main () {
    rocket::ignite()
        .mount("/", routes![create, login, login_post, fetch_special, logout])
        .manage(local_conn_string())
        .launch();
}
