use mdata::little_spider::site::javbus;

fn main() {
    let info = javbus::get_video_info(String::from("ABW-182")).unwrap();

    println!("{:?}", info);
}
