use crate::little_spider::{http,site::*};
use crate::sqlite_handler;
use anyhow::{Context, Result};
use crossbeam_channel::unbounded;
use crossbeam_utils::thread;

pub fn get_recent_videos(page_count: Option<usize>) {
    let (snd1, rcv1) = unbounded::<String>();
    let source = page_count.unwrap_or(3);
    let n_workers = 5;
    thread::scope(|scope| {
        for i in 1..source {
            let snd1 = snd1.clone();
            scope.spawn(move |_| {
                if let Ok(urls) = javbus::get_recent_video_urls(i) {
                    for u in urls {
                        crate::skip_fail!(snd1.send(u));
                    }
                }
            });
        }

        for _ in 0..n_workers {
            let rcv1 = rcv1.clone();
            scope.spawn(move |_| {
                for msg in rcv1.iter() {
                    if let Err(e) = spider_video_and_save(msg) {
                        println!("{:?}", e);
                    }
                }
            });
        }
        // 关闭信道，否则接收器不会关闭
        // 退出 for 循坏
        drop(snd1);
    })
    .unwrap()
}

fn spider_video_and_save(url: String) -> Result<()> {
    let id = http::get_url_filename(&url).context("url id not found")?;

    //if !sqlite_handler::check_video_exist(&id) {
        let video_info = javbus::get_video_info(id)?;

        if !video_info.trashed{
            println!("{:?}",video_info);
        }
        sqlite_handler::test();
        //sqlite_handler::save_video_data(&video_info);
    //}

    Ok(())
}
