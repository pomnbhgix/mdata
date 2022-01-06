use mdata::little_spider::site::dlsite;
use mdata::little_spider::site::javbus;
use mdata::sqlite_handler;

fn main() {
    let opt = Opt::from_args();

    if opt.pattern.eq("c") || opt.pattern.eq("c") {
        for id in opt.rest {
            if id.contains("rj") || id.contains("RJ") {
                let r = dlsite::get_asmr_info(id.to_uppercase()).unwrap();
                sqlite_handler::save_asmr_data(&r);
            } else {
                let info = javbus::get_video_info(id).unwrap();
                sqlite_handler::save_video_data(&info);
            }
        }
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
