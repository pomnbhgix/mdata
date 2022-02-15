#[macro_use]
extern crate lazy_static;
extern crate confy;
extern crate reqwest;
extern crate rusqlite;
extern crate scraper;
extern crate serde;
#[macro_use]
extern crate anyhow;
extern crate crossbeam_channel;
extern crate crossbeam_utils;

pub mod config;
pub mod data_manager;
pub mod file;
pub mod little_spider;
pub mod sqlite_handler;

#[macro_export]
#[allow(unused_macros)]
macro_rules! skip_fail {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                println!("An error: {:?}; skipped.", e);
                continue;
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::file;

    #[test]
    fn exploration() {



    }
}
