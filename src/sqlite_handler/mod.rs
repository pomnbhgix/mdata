use crate::config;
use crate::little_spider::site::{dlsite, javbus, wikipedia};
use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex, MutexGuard};

static CONN: OnceCell<Connection> = OnceCell::new();
use std::sync::OnceCell;

lazy_static! {
    static ref DATABASE: Mutex<Connection> = Mutex::new(create_connect().unwrap());
}

pub fn get() -> &'static Mutex<Option<Connection>> {
    if DATABASE.lock().unwrap().is_some() {
        &DATABASE
    } else {
        panic!("Singleton must be initialized before use");
    }
}

// macro_rules! get_connect {
//     () => {
//         //let conn = DATABASE.lock();

//         match DATABASE.lock() {
//             Ok(mut v) => {
//                 (if v.is_none() {
//                     *v = create_connect().ok();
//                 });
//                 match &mut *v {
//                     Some(x) => Some(x),
//                     None => None,
//                 }
//             }
//             Err(_e) => {
//                 println!("get_connect error and ignore");
//                 None
//             }
//         }
//     };
// }

pub fn test() {
    //let conn = get_connect!();
}

fn create_connect() -> Result<Connection> {
    let path = config::get_database_path()?;
    let conn = Connection::open(path)?;
    conn.execute_batch("PRAGMA key='data'")?;
    Ok(conn)
}

pub fn get_connect() -> MutexGuard<'static, Option<Connection>> {
    let mut connect = DATABASE.lock().unwrap();
    if connect.is_none() {
        *connect = create_connect().ok();
    };
    let r = Arc::clone(DATABASE);
    connect
}

pub fn get_connect_() -> &'static Connection {
    let mut connect = get_connect();
    let ref_n: &Connection = match &mut *connect {
        Some(ref x) => x,
        None => panic!("Error."),
    };

    return ref_n;
}

// pub fn check_video_exist(produce_id: &String) -> bool {
//     if let Ok(r) = get_ids(produce_id) {
//         if r.len() > 0 {
//             return true;
//         }
//     }
//     false
// }

fn get_ids(produce_id: &String) -> Result<Vec<String>> {
    let mut connect = get_connect();
    let ref_n: &Connection = match &mut *connect {
        Some(ref x) => x,
        None => panic!("Error."),
    };
    let mut stmt = ref_n.prepare("Select * from avInfo where product_id == :id")?;
    let rows = stmt.query_map(&[(":id", &produce_id)], |row| row.get(0))?;

    let mut names = Vec::new();
    for name_result in rows {
        names.push(name_result?);
    }

    Ok(names)
}

// fn insert_or_replace(connect: Connection, data: &javbus::Video) -> Result<usize> {
//     return connect.execute(
//         "INSERT OR REPLACE INTO avInfo(product_id,name,actors,date,tags,author,series,state) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
//         params![data.produce_id,data.name,data.actors,data.date,data.tags,data.author,data.series,is_trashed(data.trashed)])
//         .map_err(|e|anyhow::Error::new(e));
// }

// fn is_trashed(flag: bool) -> String {
//     if flag {
//         return String::from("uncheck");
//     }
//     return String::from("trashed");
// }

// pub fn save_video_data(data: &javbus::Video) {
//     let connect = get_connect!().context("get_connect fail").unwrap();
//     if let Err(n) = insert_or_replace(connect, data) {
//         println!("save_video_data fail err:{}", n);
//     } else {
//         println!("save_video_data success");
//     }
// }

// fn insert_or_replace_asmr(connect: Connection, data: &dlsite::Asmr) -> Result<usize> {
//     return connect.execute(
//         "INSERT OR REPLACE INTO asmrInfo(product_id,name,voice_actors,date,tags,author,series,state) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
//         params![data.produce_id,data.name,data.actors,data.date,data.tags,data.author,data.series,"uncheck"])
//         .map_err(|e|anyhow::Error::new(e));
// }

// pub fn save_asmr_data(data: &dlsite::Asmr) {
//     let connect = get_connect!().context("get_connect fail").unwrap();
//     if let Err(n) = insert_or_replace_asmr(connect, data) {
//         println!("action fail err:{}", n);
//     } else {
//         println!("action success");
//         println!("{:?}", data);
//     }
// }

// fn insert_or_replace_anime(connect: Connection, data: &wikipedia::A1CWork) -> Result<usize> {
//     return connect
//         .execute(
//             "INSERT OR REPLACE INTO animeR18Info(name,author,date,company) VALUES (?1, ?2, ?3, ?4)",
//             params![
//                 data.name,
//                 data.author,
//                 data.date,
//                 "エイ・ワン・シー (a1c. Co., Ltd.)"
//             ],
//         )
//         .map_err(|e| anyhow::Error::new(e));
// }

// pub fn save_anime_data(data: &wikipedia::A1CWork) {
//     let connect = get_connect!().context("get_connect fail").unwrap();
//     if let Err(n) = insert_or_replace_anime(connect, data) {
//         println!("action fail err:{}", n);
//     } else {
//         println!("action success");
//         println!("{:?}", data);
//     }
// }

// fn insert_or_replace_anime_info(connect: Connection, data: &javbus::ActorInfo) -> Result<usize> {
//     return connect.execute(
//         "INSERT OR REPLACE INTO actorInfo(name,born,height,bra,hobby,body_measurements) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
//         params![data.name,data.born,data.height,data.bra,data.hobby,data.get_body_measurements()])
//         .map_err(|e|anyhow::Error::new(e));
// }

// fn insert_or_replace_mangaka_info(
//     connect: Connection,
//     name: String,
//     alias: String,
// ) -> Result<usize> {
//     return connect
//         .execute(
//             "INSERT OR REPLACE INTO mangakaInfo(name,alias,gender) VALUES (?1, ?2, ?3)",
//             params![name, alias, "woman"],
//         )
//         .map_err(|e| anyhow::Error::new(e));
// }

// pub fn save_actor_mangaka_data(name: String, alias: String) {
//     let connect = get_connect!().context("get_connect fail").unwrap();
//     if let Err(n) = insert_or_replace_mangaka_info(connect, name, alias) {
//         println!("action fail err:{}", n);
//     } else {
//         println!("action success");
//     }
// }

// pub fn save_actor_info_data(data: &javbus::ActorInfo) {
//     let connect = get_connect!().context("get_connect fail").unwrap();
//     if let Err(n) = insert_or_replace_anime_info(connect, data) {
//         println!("action fail err:{}", n);
//     } else {
//         println!("action success");
//         println!("{:?}", data);
//     }
// }
