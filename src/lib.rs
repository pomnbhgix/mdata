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
    use super::little_spider::site::*;

    #[test]
    fn exploration() {
        javbus::get_actor_info("三上悠亜");
    }
}
