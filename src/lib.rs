#[macro_use]
extern crate lazy_static;
extern crate confy;
extern crate reqwest;
extern crate rusqlite;
extern crate scraper;
extern crate serde;
#[macro_use]
extern crate anyhow;

pub mod config;
pub mod file;
pub mod little_spider;
pub mod sqlite_handler;

#[cfg(test)]
mod tests {
    use super::little_spider::site::*;
    use super::sqlite_handler::*;

    #[test]
    fn exploration() {
        let info = javbus::get_actor_works("三上悠亜");
        match info {
            Ok(r) => println!("{:?}", r),
            Err(e) => println!("{:?}", e),
        }
    }
}
