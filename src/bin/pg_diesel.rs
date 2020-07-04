extern crate diesel;
extern crate rust_web;

use chrono::Utc;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use rust_web::models::*;
use rust_web::schema::*;

fn create_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Database URL not set.");
    PgConnection::establish(&database_url).expect("Error connecting to database!")
}

fn create_user(conn: &PgConnection) -> UserEntity {
    let u = User {name: "Jim".to_string(), email: "james@test.com".to_string(), age: 26};
    diesel::insert_into(users::table).values(&u).get_result(conn).expect("Error saving user")
}

fn create_article(conn: &PgConnection, user_id: i32) -> ArticleEntity {
    let a = Article { title: "A Great Article!".to_string(), body: "Read this article on Haskell.".to_string(), published_at: Utc::now(), author_id: user_id};
    diesel::insert_into(articles::table).values(&a).get_result(conn).expect("Error saving article")
}

fn fetch_articles(connection: &PgConnection, uid: i32) -> Vec<ArticleEntity> {
    use rust_web::schema::articles::dsl::*;
    // let all_users = users.load::<UserEntity>(&connection);
    articles.filter(author_id.eq(uid)).order(title).load::<ArticleEntity>(connection).expect("Error loading users")
}

fn fetch_all_users_articles(connection: &PgConnection) -> Vec<(String, String)> {
    use rust_web::schema::users::dsl::*;
    use rust_web::schema::articles::dsl::*;
    users.inner_join(articles.on(author_id.eq(rust_web::schema::users::dsl::id))).select((name, title)).load(connection).expect("Couldn't join!")
}

fn main() {
    let connection = create_connection();
    let user_entity = create_user(&connection);
    let _article_entity = create_article(&connection, user_entity.id);
    let articles = fetch_articles(&connection, user_entity.id);
    for article in articles {
        println!("{}", article.title);
    }
}