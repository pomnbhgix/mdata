use mdata::file;
use mdata::little_spider::site::javbus;
use mdata::sqlite_handler;

fn main() {

    let path = "unhandle_data/asmrcheck.md";
    let data = file::read_asmr_data(path);
    // for id in data {
    //     let info = javbus::get_video_info(id).unwrap();
    //     sqlite_handler::save_video_data(&info);
    // }
    
}
