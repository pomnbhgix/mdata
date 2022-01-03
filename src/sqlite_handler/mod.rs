use crate::little_spider::site::javbus;

use rusqlite::{params, Connection, Result};
use std::sync::Mutex;

lazy_static! {
    static ref DATABASE: Mutex<Option<Connection>> = Mutex::new(None);
}

pub fn create_connect() -> Option<Connection> {
    if let Ok(v) = Connection::open("D:/work/notes/assets/check.db") {
        return Some(v);
    }
    println!("create fail");
    None
}

macro_rules! get_connect {
    () => {
        match DATABASE.lock() {
            Ok(mut v) => {
                if v.is_none() {
                    *v = create_connect();
                    println!("sqlite connect create instance");
                }
                Ok(v)
            }
            Err(e) => Err(e),
        };
    };
}

pub fn save_video_data(data: &javbus::Video) {
    if let Ok(v) = get_connect!() {


    }
    if let Ok(_v) = get_connect!() {}

    println!("{:?}", data);
}
