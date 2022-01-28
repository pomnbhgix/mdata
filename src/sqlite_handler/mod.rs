use crate::config;
use crate::little_spider::site::dlsite;
use crate::little_spider::site::javbus;
use crate::little_spider::site::wikipedia;

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
    if let Ok(path) = config::get_database_path() {
        if let Ok(v) = Connection::open(path) {
            if let Ok(_n) = v.execute_batch("PRAGMA key='data'") {
                println!("connect success");
                return Some(v);
            }
        }
    }
    println!("create fail");
    None
}

fn insert_or_replace(connect: Connection, data: &javbus::Video) -> Result<usize> {
    return connect.execute(
        "INSERT OR REPLACE INTO avInfo(product_id,name,actors,date,tags,author,series,state) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![data.produce_id,data.name,data.actors,data.date,data.tags,data.author,data.series,"uncheck"]);
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
        "INSERT OR REPLACE INTO asmrInfo(product_id,name,voice_actors,date,tags,author,series,state) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![data.produce_id,data.name,data.actors,data.date,data.tags,data.author,data.series,"uncheck"]);
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

fn insert_or_replace_anime(connect: Connection, data: &wikipedia::A1CWork) -> Result<usize> {
    return connect.execute(
        "INSERT OR REPLACE INTO animeR18Info(name,author,date,company) VALUES (?1, ?2, ?3, ?4)",
        params![
            data.name,
            data.author,
            data.date,
            "エイ・ワン・シー (a1c. Co., Ltd.)"
        ],
    );
}

pub fn save_anime_data(data: &wikipedia::A1CWork) {
    if let Some(connect) = get_connect!() {
        if let Err(n) = insert_or_replace_anime(connect, data) {
            println!("action fail err:{}", n);
        } else {
            println!("action success");
            println!("{:?}", data);
        }
    }
}

fn insert_or_replace_anime_info(connect: Connection, data: &javbus::ActorInfo) -> Result<usize> {
    return connect.execute(
        "INSERT OR REPLACE INTO actorInfo(name,born,height,bra,hobby,body_measurements) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![data.name,data.born,data.height,data.bra,data.hobby,data.get_body_measurements()]);
}

fn insert_or_replace_mangaka_info(connect: Connection, name: String, alias: String) -> Result<usize> {
    return connect.execute(
        "INSERT OR REPLACE INTO mangakaInfo(name,alias,gender) VALUES (?1, ?2, ?3)",
        params![name,alias,"woman"]);
}

pub fn save_actor_mangaka_data(name: String, alias: String){
    if let Some(connect) = get_connect!() {
        if let Err(n) = insert_or_replace_mangaka_info(connect, name, alias) {
            println!("action fail err:{}", n);
        } else {
            println!("action success");
        }
    }
}

pub fn save_actor_info_data(data: &javbus::ActorInfo) {
    if let Some(connect) = get_connect!() {
        if let Err(n) = insert_or_replace_anime_info(connect, data) {
            println!("action fail err:{}", n);
        } else {
            println!("action success");
            println!("{:?}", data);
        }
    }
}
