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
        self.actors = self.get_all_a(p_node_date);
    }

    fn set_video_tags(&mut self, p_node_date: &ElementRef) {
        self.tags = self.get_all_a(p_node_date);
    }

    fn get_all_a(&self, p_node_date: &ElementRef) -> String {
        if let Ok(a_selector) = Selector::parse("a") {
            return p_node_date
                .select(&a_selector)
                .collect::<Vec<_>>()
                .iter()
                .map(|t| t.inner_html())
                .collect::<Vec<_>>()
                .join(",");
        }
        return String::new();
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

pub fn get_video_info(produce_id: String) -> Result<Video, Box<dyn std::error::Error>> {
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
    let url = format!("https://www.javbus.com/{}", produce_id);

    let body = reqwest::blocking::get(url)?.text()?;

    let document = Html::parse_document(&body);

    let mut result = Video::new(produce_id);

    result.init(&document);

    if result.name.is_empty() && !name.is_empty() {
        result.name = name;
    }

    return Ok(result);
}
