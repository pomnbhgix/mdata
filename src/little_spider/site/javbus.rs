use crate::little_spider;
use crate::little_spider::http;
use anyhow::{Context, Result};
use reqwest;
use scraper::{ElementRef, Html, Selector};

#[derive(Debug)]
pub struct Video {
    pub produce_id: String,
    pub name: String,
    pub date: String,
    pub author: String,
    pub series: String,
    pub tags: String,
    pub actors: String,
}

impl Video {
    fn new(produce_id: String) -> Video {
        return Video {
            produce_id: produce_id,
            name: String::new(),
            date: String::new(),
            author: String::new(),
            series: String::new(),
            tags: String::new(),
            actors: String::new(),
        };
    }

    fn init(&mut self, doc: &Html) {
        self.name = self.get_video_name(doc);

        let selector = Selector::parse("Div.info p");

        if let Ok(v) = selector {
            let mut select = doc.select(&v);
            let span_select = Selector::parse("span").unwrap();

            while let Some(n) = select.next() {
                if let Some(m) = n.select(&span_select).next() {
                    self.handle_data(&m, &n, &mut select);
                }
            }
        }
    }

    fn get_video_name(&self, doc: &Html) -> String {
        let selector = Selector::parse("h3");
        if let Ok(v) = selector {
            let name = doc.select(&v).next();
            if let Some(n) = name {
                return n.inner_html();
            }
        }
        return String::new();
    }

    fn handle_data(
        &mut self,
        span_element: &ElementRef,
        p_element: &ElementRef,
        p_select: &mut scraper::html::Select,
    ) -> String {
        let key = span_element.inner_html();
        match key.as_str() {
            "發行日期:" => {
                self.set_video_date(&p_element);
            }
            "製作商:" => {
                self.set_video_author(&p_element);
            }
            "系列:" => {
                self.set_video_series(&p_element);
            }
            "演員" => {
                if let Some(node) = p_select.next() {
                    self.set_video_actors(&node);
                }
            }
            _ => {
                if p_element.inner_html().contains("類別:") {
                    if let Some(node) = p_select.next() {
                        self.set_video_tags(&node);
                    }
                }
            }
        };

        String::new()
    }

    fn set_video_date(&mut self, p_node_date: &ElementRef) {
        let data = p_node_date.inner_html();
        let vec: Vec<&str> = data.split("</span>").collect();
        if let Some(n) = vec.last() {
            self.date = n.to_string();
        }
    }

    fn set_video_author(&mut self, p_node_date: &ElementRef) {
        self.author = self.get_a_tag_text(p_node_date)
    }

    fn set_video_series(&mut self, p_node_date: &ElementRef) {
        self.series = self.get_a_tag_text(p_node_date)
    }

    fn set_video_actors(&mut self, p_node_date: &ElementRef) {
        self.actors = super::join_all_a_elements_inner_html(p_node_date);
    }

    fn set_video_tags(&mut self, p_node_date: &ElementRef) {
        self.tags = super::join_all_a_elements_inner_html(p_node_date);
    }

    fn get_a_tag_text(&self, p_node_date: &ElementRef) -> String {
        if let Ok(a_selector) = Selector::parse("a") {
            if let Some(n) = p_node_date.select(&a_selector).next() {
                return n.inner_html();
            }
        }
        String::new()
    }
}

pub fn get_video_info(produce_id: String) -> Result<Video> {
    let url = format!("https://www.javbus.com/{}", produce_id);

    let body = reqwest::blocking::get(url)?.text()?;

    let document = Html::parse_document(&body);

    let mut result = Video::new(produce_id);

    result.init(&document);

    return Ok(result);
}

pub fn get_video_info_with_name(
    produce_id: String,
    name: String,
) -> Result<Video, Box<dyn std::error::Error>> {
    let mut result = get_video_info(produce_id)?;

    if result.name.is_empty() && !name.is_empty() {
        result.name = name;
    }

    return Ok(result);
}

#[derive(Debug)]
pub struct ActorInfo {
    pub name: String,
    pub alias: String,
    pub born: String,
    pub height: String,
    pub weight: String,
    pub bra: String,
    pub hobby: String,
    pub bust: String,
    pub waist: String,
    pub hip: String,
    pub debut_date: String,
}

impl ActorInfo {
    /// Creates new header given it's name and value.
    fn new(name: String) -> Self {
        ActorInfo {
            name: name,
            alias: String::new(),
            born: String::new(),
            height: String::new(),
            weight: String::new(),
            bra: String::new(),
            hobby: String::new(),
            bust: String::new(),
            waist: String::new(),
            hip: String::new(),
            debut_date: String::new(),
        }
    }

    fn set_value_by_raw(&mut self, value: Vec<String>) {
        for d in value {
            let vec = d.split(":").collect::<Vec<&str>>();

            if let Some(key) = vec.get(0) {
                match key.as_ref() {
                    "生日" => self.born = get_vec_data(&vec, 1),
                    "身高" => self.height = get_vec_data(&vec, 1),
                    "罩杯" => self.bra = get_vec_data(&vec, 1),
                    "愛好" => self.hobby = get_vec_data(&vec, 1),
                    "胸圍" => self.bust = get_vec_data_with_remove_cm(&vec, 1),
                    "腰圍" => self.waist = get_vec_data_with_remove_cm(&vec, 1),
                    "臀圍" => self.hip = get_vec_data_with_remove_cm(&vec, 1),
                    _ => {}
                }
            }
        }
    }

    pub fn get_body_measurements(&self) -> String {
        format!("{}-{}-{}", self.bust, self.waist, self.hip)
    }
}

fn get_vec_data_with_remove_cm(vec: &Vec<&str>, index: usize) -> String {
    let value = get_vec_data(vec, index);
    String::from(value.replace("cm", "").trim())
}

fn get_vec_data(vec: &Vec<&str>, index: usize) -> String {
    if let Some(v) = vec.get(index) {
        return String::from(*v);
    }
    String::new()
}

fn get_actor_info_url(actor_name: &str) -> Result<String> {
    let url = format!("https://www.javbus.com/searchstar/{}", actor_name);
    let body = http::get_text_response(&url)?;

    let document = Html::parse_document(&body);

    let select = little_spider::get_selector("#waterfall > div > a")?;

    let data = document
        .select(&select)
        .next()
        .context(format!("select document return none :{:?}", select))?
        .value()
        .attr("href")
        .context(format!("read attr attr return none"))?;

    Ok(data.to_string())
}

pub fn get_actor_info(actor_name: &str) -> Result<ActorInfo, Box<dyn std::error::Error>> {
    let data_url = get_actor_info_url(actor_name)?;

    let info_body = http::get_text_response(&data_url)?;

    let info_document = Html::parse_document(&info_body);

    let info_select = little_spider::get_selector("div.photo-info")?;

    let actor_info = info_document
        .select(&info_select)
        .next()
        .context(format!("select document return none :{:?}", info_select))?;

    let ptag_select = little_spider::get_selector("p")?;

    let infos: Vec<ElementRef> = actor_info.select(&ptag_select).collect();

    let info_vec = super::get_elements_inner_html(infos);

    let mut result = ActorInfo::new(String::from(actor_name));

    result.set_value_by_raw(info_vec);

    Ok(result)
}

fn get_one_page_works_url(url: &String, index: usize) -> Result<Vec<String>> {
    let work_url = format!("{}/{}", url, index);
    let info_body = http::get_text_response(&work_url)?;
    let info_document = Html::parse_document(&info_body);
    let work_select = little_spider::get_selector("#waterfall > div > a")?;
    let actor_infos: Vec<ElementRef> = info_document.select(&work_select).collect();
    let href_vec = super::get_elements_attr(actor_infos, "href");
    return Ok(href_vec);
}

pub fn get_actor_works(actor_name: &str) -> Result<Vec<String>> {
    let data_url = get_actor_info_url(actor_name)?;
    let mut result: Vec<String> = Vec::new();
    let mut index: usize = 1;
    while let Ok(mut urls) = get_one_page_works_url(&data_url, index) {
        result.append(&mut urls);
        index += 1;
    }

    let r = result
        .iter()
        .map(|t| http::get_url_filename(&t).unwrap_or(String::new()))
        .collect::<Vec<_>>();

    return Ok(r);
}

const BASE_URL: &str = "https://www.javbus.com/";

const FILTER_TAGS: &'static [&'static str] = &["成熟的女人"];

pub fn get_recent_video_urls(page_index: usize) -> Result<Vec<String>> {
    let url = format!("{}{}", BASE_URL, "page");
    get_one_page_works_url(&url, page_index)
}

pub fn get_video_info_by_url(url: String) -> Result<Video> {
    let filename = http::get_url_filename(&url).ok_or(anyhow!("get_url_filename err"))?;
    get_video_info(filename)
}

pub fn check_filter(video: &Video) -> bool {
    for f in FILTER_TAGS {
        if video.tags.contains(f) {
            return false;
        }
    }
    true
}
