#[macro_use]
extern crate lazy_static;
extern crate confy;
extern crate reqwest;
extern crate rusqlite;
extern crate scraper;
extern crate serde;

pub mod config;
pub mod file;
pub mod little_spider;
pub mod sqlite_handler;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn exploration() {
        for data in little_spider::site::wikipedia::get_a1c_works() {
            sqlite_handler::save_anime_data(&data);
        }
    }
}
