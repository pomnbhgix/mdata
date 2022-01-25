use mdata::little_spider::site::dlsite;
use mdata::little_spider::site::javbus;
use mdata::sqlite_handler;

fn main() {
    let opt = Opt::from_args();
    match opt.pattern.as_str() {
        "ca" => check_asmr(opt.rest),
        "cai" => check_actor_info(opt.rest),
        "cv" => check_video(opt.rest),
        _=>{},
    }
}

fn check_asmr(data: Vec<String>) {
    for id in data {
        let r = dlsite::get_asmr_info(id.to_uppercase()).unwrap();
        sqlite_handler::save_asmr_data(&r);
    }
}

fn check_actor_info(data: Vec<String>) {
    for id in data {
        let r = javbus::get_actor_info(&id).expect("get actor info fail");
        sqlite_handler::save_actor_info_data(&r);

        for work in javbus::get_actor_works(&id).expect("get works info fail"){
            if let Ok(info) = javbus::get_video_info(work) {
                sqlite_handler:: save_video_data(&info)
            }
        }

    }
}

fn check_video(data: Vec<String>) {
    for id in data {
        let info = javbus::get_video_info(id).unwrap();
        sqlite_handler::save_video_data(&info);
    }
}

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "arguments")]
struct Opt {
    pattern: String,

    #[structopt(name = "ARGUMENTS")]
    rest: Vec<String>,
}
