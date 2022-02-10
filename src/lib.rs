#[macro_use]
extern crate lazy_static;
extern crate confy;
extern crate reqwest;
extern crate rusqlite;
extern crate scraper;
extern crate serde;
#[macro_use]
extern crate anyhow;
extern crate crossbeam_utils;

pub mod config;
pub mod file;
pub mod little_spider;
pub mod sqlite_handler;

#[cfg(test)]
mod tests {
    use super::little_spider;

    #[test]
    fn exploration() {

        little_spider::site::javbus::get_recent_videos(Option::None);

    }
}
