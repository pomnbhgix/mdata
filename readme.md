# little spider

TODO

## code collection

``` rust
macro_rules! skip_fail {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                println!("An error: {:?}; skipped.", e);
                continue;
            }
        }
    };
}


pub fn get_recent_videos_v1(page_count: Option<usize>) {
    let p_count = page_count.unwrap_or(3);
    let infos: Vec<Video> = Vec::new();
    let connections = Arc::new(Mutex::new(infos));
    //let mut infos: Vec<String> = Vec::new();
    thread::scope(|scope| {
        let conn = &connections;
        for p in 1..p_count {
            scope.spawn(move |_| {
                let url = format!("{}/{}", BASE_URL, "page");
                let urls = get_one_page_works_url(&url, p).unwrap();
                for u in urls {
                    let filename = get_url_filename(u);
                    if let Some(name) = filename {
                        let info = get_video_info(name).unwrap();
                        conn.lock().unwrap().push(info);
                        break;
                    }
                }
                //people.push("aa".to_string());
            });
            break;
        }
    })
    .unwrap();

    println!("{:?}", connections.lock().unwrap());
}

pub fn get_recent_videos_v2(page_count: Option<usize>) {
    let p_count = page_count.unwrap_or(3);
    let mut infos: Vec<Video> = Vec::new();

    thread::scope(|scope| {
        let infos = &mut infos;
        for p in 1..p_count {
            scope.spawn(move |_| {
                let url = format!("{}/{}", BASE_URL, "page");
                let urls = get_one_page_works_url(&url, p).unwrap();
                for u in urls {
                    let filename = get_url_filename(u);
                    if let Some(name) = filename {
                        let info = get_video_info(name).unwrap();
                        infos.push(info);
                        break;
                    }
                }
            });
            break;
        }
    })
    .unwrap();

    println!("{:?}", infos);
}


pub fn get_recent_videos(page_count: Option<usize>) -> anyhow::Result<Vec<Video>> {
    let (snd1, rcv1) = unbounded::<String>();
    let source = page_count.unwrap_or(3);
    let n_workers = 5;
    let infos: Vec<Video> = Vec::new();
    let connections = Arc::new(Mutex::new(infos));
    thread::scope(|scope| {
        for i in 1..source {
            let snd1 = snd1.clone();
            scope.spawn(move |_| {
                if let Ok(urls) = get_recent_video_urls(i) {
                    for u in urls {
                        snd1.send(u).map_err(|e| anyhow::Error::new(e)).sikp_fail();
                    }
                }
            });
        }
        let conn = &connections;
        for _ in 0..n_workers {
            let rcv1 = rcv1.clone();
            scope.spawn(move |_| {
                for msg in rcv1.iter() {
                    get_video_info_and_push(conn, msg).sikp_fail();
                }
            });
        }
        // 关闭信道，否则接收器不会关闭
        // 退出 for 循坏
        drop(snd1);
    })
    .map_err(|e| anyhow!("{:?}", e))?;
    let result = Arc::try_unwrap(connections)
        .map_err(|e| anyhow!("{:?}", e))?
        .into_inner()?;
    Ok(result)
}

fn get_video_info_and_push(
    conn: &std::sync::Arc<Mutex<Vec<Video>>>,
    message: String,
) -> anyhow::Result<()> {
    let filename = get_url_filename(message).ok_or(anyhow!("get_url_filename err"))?;
    let video_info = get_video_info(filename)?;
    conn.lock()
        .map_err(|e| anyhow!("{:?}", e))?
        .push(video_info);
    Ok(())
}

fn join_all_li_elements_inner_html(element: &ElementRef) -> String {
    if let Ok(a_selector) = Selector::parse("li") {
        return join_elements_inner_html(element.select(&a_selector).collect::<Vec<_>>());
    }
    return String::new();
}
```
