use crate::little_spider::site::dlsite;
use crate::little_spider::site::javbus;

use rusqlite::{params, Connection, Result};
use std::sync::Mutex;

lazy_static! {
    static ref DATABASE: Mutex<Option<Connection>> = Mutex::new(None);
}

macro_rules! get_connect {
    () => {
        match DATABASE.lock() {
            Ok(mut v) => {
                if v.is_none() {
                    *v = create_connect();
                    println!("sqlite connect create instance");
                }
                v.take()
            }
            Err(_e) => {
                println!("get_connect error and ignore");
                None
            }
        }
    };
}

pub fn create_connect() -> Option<Connection> {
    if let Ok(v) = Connection::open("D:/work/notes/assets/check.db") {
        if let Ok(_n) = v.execute_batch("PRAGMA key='data'") {
            return Some(v);
        }
    }
    println!("create fail");
    None
}

fn insert_or_replace(connect: Connection, data: &javbus::Video) -> Result<usize> {
    return connect.execute(
        "INSERT OR REPLACE INTO avInfo(product_id,name,actors,date,tags,author,series) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![data.produce_id,data.name,data.actors,data.date,data.tags,data.author,data.series]);
}

pub fn save_video_data(data: &javbus::Video) {
    if let Some(connect) = get_connect!() {
        if let Err(n) = insert_or_replace(connect, data) {
            println!("action fail err:{}", n);
        } else {
            println!("action success");
            println!("{:?}", data);
        }
    }
}

fn insert_or_replace_asmr(connect: Connection, data: &dlsite::Asmr) -> Result<usize> {
    return connect.execute(
        "INSERT OR REPLACE INTO asmrInfo(product_id,name,voice_actors,date,tags,author,series) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![data.produce_id,data.name,data.actors,data.date,data.tags,data.author,data.series]);
}

pub fn save_asmr_data(data: &dlsite::Asmr) {
    if let Some(connect) = get_connect!() {
        if let Err(n) = insert_or_replace_asmr(connect, data) {
            println!("action fail err:{}", n);
        } else {
            println!("action success");
            println!("{:?}", data);
        }
    }
}
