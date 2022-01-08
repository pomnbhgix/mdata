#[macro_use]
extern crate lazy_static;
extern crate reqwest;
extern crate scraper;
extern crate rusqlite;
extern crate confy;
extern crate serde;


pub mod little_spider;
pub mod sqlite_handler;
pub mod file;
pub mod config;


#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        println!("hello world");
        assert_eq!(2 + 2, 4);
    }
}

