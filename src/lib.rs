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
    use super::little_spider::site::*;
    use super::sqlite_handler::*;

    #[test]
    fn exploration() {

        let info = javbus::get_actor_info("三上悠亜").unwrap();
        save_actor_info_data(&info);
        // let works = javbus::get_actor_works("三上悠亜").unwrap();

        // for work in works {
        //     if let Ok(info) = javbus::get_video_info(work) {
        //         save_video_data(&info)
        //     }
        // }
    }
}
