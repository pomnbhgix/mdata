use crate::little_spider;
use crate::little_spider::http;
use anyhow::Result;
use scraper::{ElementRef, Html, Selector};

#[derive(Debug)]
pub struct A1CWork {
    pub name: String,
    pub author: String,
    pub date: String,
    pub episodes: String,
}

impl A1CWork {
    fn new(name: String, author: String, date: String, episodes: String) -> A1CWork {
        A1CWork {
            name: name,
            author: author,
            date: date,
            episodes: episodes,
        }
    }
}

fn get_author(node: &ElementRef) -> String {
    get_dt_select_inner_html(node)
}

fn get_episodes(node: &ElementRef) -> Vec<String> {
    if let Some(lis) = get_li_selects(node) {
        return super::get_elements_inner_html(lis);
    }
    return Vec::new();
}

fn get_works_data<'a>(doc: &'a ElementRef) -> Option<Vec<ElementRef<'a>>> {
    let selector = Selector::parse("dd > ul > li");
    if let Ok(v) = selector {
        return Some(doc.select(&v).collect::<Vec<_>>());
    }
    return None;
}

fn get_li_selects<'a>(doc: &'a ElementRef) -> Option<Vec<ElementRef<'a>>> {
    let selector = Selector::parse("li");
    if let Ok(v) = selector {
        return Some(doc.select(&v).collect::<Vec<_>>());
    }
    return None;
}

fn get_dt_select_inner_html(doc: &ElementRef) -> String {
    if let Ok(selector) = Selector::parse("dt") {
        if let Some(r) = doc.select(&selector).next() {
            return r.inner_html();
        }
    }
    return String::new();
}

pub fn get_a1c_works() -> Vec<A1CWork> {
    let url =
        "https://ja.wikipedia.org/wiki/%E3%82%A8%E3%82%A4%E3%83%BB%E3%83%AF%E3%83%B3%E3%83%BB%E3%82%B7%E3%83%BC";
    let response = http::get_text_response(url).unwrap();
    let document = Html::parse_document(&response);
    let mut result: Vec<A1CWork> = Vec::new();
    let work_groups = get_d1_selects(&document).unwrap();

    for group in work_groups {
        let work_list = get_works_data(&group).unwrap();
        for work in work_list {
            let eps = get_episodes(&work);
            let data = A1CWork::new(
                element_text_without_child(&work),
                get_author(&group),
                get_date(&eps),
                eps.join(","),
            );

            if data.author.eq("Shelf") {
                break;
            }
            println!("{:?}", data);
            result.push(data);
        }
    }

    return result;
}

fn element_text_without_child(doc: &ElementRef) -> String {
    let text = doc.text();
    let mut s: Vec<String> = Vec::new();
    for e in text.collect::<Vec<_>>() {
        let st = e.to_string();
        if st.contains("\n") {
            s.push(st);
            break;
        } else {
            s.push(st);
        }
    }
    s.join("")
}

fn get_date(eps: &Vec<String>) -> String {
    if let Some(r) = eps.first() {
        if let Some(t) = r.split("（").collect::<Vec<_>>().last() {
            let mut ts = t.to_string();
            ts.pop();
            return ts;
        }
    }
    return String::new();
}

fn get_d1_selects(doc: &Html) -> Option<Vec<ElementRef>> {
    let selector = Selector::parse("#mw-content-text > div.mw-parser-output > dl");
    if let Ok(v) = selector {
        return Some(doc.select(&v).collect());
    }
    return None;
}

pub fn get_info_text(name: &str) -> Result<String> {
    let url = format!("https://ja.wikipedia.org/wiki/{}", name);

    let response = http::get_text_response(&url)?;

    let document = Html::parse_document(&response);

    let select = little_spider::get_selector("table.infobox")?;

    let node = document
        .select(&select)
        .next()
        .ok_or(anyhow!("not found node"))?;

    //#mw-content-text > div.mw-parser-output > table:nth-child(3) > tbody > tr:nth-child(1) > th

    Ok(node.inner_html())
}
