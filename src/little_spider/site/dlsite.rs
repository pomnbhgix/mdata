use scraper::{ElementRef, Html, Selector};

#[derive(Debug)]
pub struct Asmr {
    pub produce_id: String,
    pub name: String,
    pub date: String,
    pub author: String,
    pub series: String,
    pub tags: String,
    pub actors: String,
}
impl Asmr {
    fn new(produce_id: String) -> Asmr {
        return Asmr {
            produce_id: produce_id,
            name: String::new(),
            date: String::new(),
            author: String::new(),
            series: String::new(),
            tags: String::new(),
            actors: String::new(),
        };
    }

    fn load_data_from_net(&mut self, doc: &Html) {
        self.name = get_asmr_name(doc);
        self.author = get_asmr_author(doc);

        let selector = Selector::parse("#work_outline > tbody > tr");
        if let Ok(v) = selector {
            let mut select = doc.select(&v);
            while let Some(n) = select.next() {
                self.handle_data(&n);
            }
        }
    }

    fn handle_data(&mut self, tr: &ElementRef) {
        let th_select = Selector::parse("th").unwrap();
        let key = tr.select(&th_select).next().unwrap().inner_html();
        match key.as_str() {
            "販売日" => self.date = get_first_a_element_inner_html(tr),
            "シリーズ名" => self.series = get_first_a_element_inner_html(tr),
            "声優" => self.actors = super::join_all_a_elements_inner_html(tr),
            "ジャンル" => self.tags = super::join_all_a_elements_inner_html(tr),
            _ => {}
        }
    }
}

fn get_first_a_element_inner_html(element: &ElementRef) -> String {
    get_select_first_element_inner_html(element, "a")
}

fn get_select_first_element_inner_html(element: &ElementRef, selector: &str) -> String {
    if let Ok(s) = Selector::parse(selector) {
        if let Some(n) = element.select(&s).next() {
            return n.inner_html();
        }
    }
    String::new()
}

pub fn get_asmr_info(produce_id: String) -> Result<Asmr, Box<dyn std::error::Error>> {
    let url = format!(
        "https://www.dlsite.com/maniax/work/=/product_id/{}.html",
        produce_id
    );
    let body = reqwest::blocking::get(url)?.text()?;

    let document = Html::parse_document(&body);

    let mut data = Asmr::new(produce_id);

    data.load_data_from_net(&document);

    Ok(data)
}

fn get_asmr_name(doc: &Html) -> String {
    return get_select_element_inner_html(doc, "#work_name");
}

fn get_asmr_author(doc: &Html) -> String {
    return get_select_element_inner_html(doc, "#work_maker > tbody > tr > td > span > a");
}

fn get_select_element_inner_html(doc: &Html, select: &'_ str) -> String {
    let selector = Selector::parse(select);
    if let Ok(v) = selector {
        let name = doc.select(&v).next();
        if let Some(n) = name {
            return n.inner_html();
        }
    }
    return String::new();
}
