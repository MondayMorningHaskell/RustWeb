use postgres::{Client, NoTls, Error};
use chrono::prelude::Utc;

fn pg_main() -> Result<(), Error> {
    let conn_string = "host=localhost port=5432 dbname=mmrust user=postgres password=postgres";
    let mut client= Client::connect(conn_string, NoTls)?;

    client.batch_execute("\
        CREATE TABLE users (
            id    SERIAL PRIMARY KEY,
            name  TEXT NOT NULL,
            email TEXT NOT NULL,
            age   INTEGER NOT NULL
        )
    ")?;

    client.batch_execute("\
        CREATE TABLE articles (
            id             SERIAL PRIMARY KEY,
            title          TEXT NOT NULL,
            body           TEXT NOT NULL,
            published_at   TIMESTAMP WITH TIME ZONE NOT NULL,
            author_id      INTEGER REFERENCES users(id)
        )
    ")?;

    let name = "James";
    let email = "james@test.com";
    let age = 26;
    client.execute(
        "INSERT INTO users (name, email, age) VALUES ($1, $2, $3)",
        &[&name, &email, &age],
    )?;

    for row in client.query("SELECT * FROM users", &[])? {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let email: &str = row.get(2);
        let age: i32 = row.get(3);

        println!("Found person: {} {} {} {}", id, name, email, age);

        let title: &str = "A great article!";
        let body: &str = "What should you be reading about?";
        let cur_time = Utc::now();
        client.execute(
            "INSERT INTO articles (title, body, published_at, author_id) VALUES ($1, $2, $3, $4)",
            &[&title, &body, &cur_time, &id]
        )?;
    }

    return Ok(());
}

fn main() {
    let res = pg_main();
    match res {
        Ok(_) => {println!("Succeeded!");},
        Err(e) => {println!("Error: {}!", e);}
    }
}
