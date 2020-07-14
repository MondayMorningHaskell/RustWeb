#[macro_use]
extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_derive;

pub mod models;
pub mod schema;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
