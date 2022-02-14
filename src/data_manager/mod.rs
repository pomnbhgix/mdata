use crate::little_spider::site::*;
use crate::sqlite_handler;
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
                    let video = crate::skip_fail!(javbus::get_video_info_by_url(msg));
                    sqlite_handler::save_video_data(&video);
                }
            });
        }
        // 关闭信道，否则接收器不会关闭
        // 退出 for 循坏
        drop(snd1);
    })
    .unwrap()
}


