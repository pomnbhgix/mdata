use mdata::little_spider::site::javbus;
use mdata::sqlite_handler;

fn main() {
    let info = javbus::get_video_info(String::from("ABW-182")).unwrap();
    sqlite_handler::save_video_data(&info);
}
