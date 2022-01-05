use mdata::file;
use mdata::little_spider::site::javbus;
use mdata::sqlite_handler;

fn main() {
    let info = javbus::get_video_info("HNDS-016".to_string()).unwrap();
    sqlite_handler::save_video_data(&info);
    // let path = "avcheck.md";
    // let data = file::read_data(path);
    // for (id, name) in data {
    //     let info = javbus::get_video_info_with_name(id,name).unwrap();
    //     sqlite_handler::save_video_data(&info);
    // }
    
}
