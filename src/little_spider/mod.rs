use anyhow::Result;
use scraper::Selector;

pub mod http;
pub mod site;

fn get_selector(selectors: &str) -> Result<scraper::Selector> {
    Ok(Selector::parse(selectors).map_err(|e| anyhow!("Selector parse fail : {:?}", e))?)
}

const DATA: &'static [&'static str] = &[
    "クリムゾン",
    "よしろん（鎖キャタピラ）",
    "長瀬徹（揚げナス）",
    "Hisasi",
    "小島紗",
    "佐伯",
    "岡田コウ",
    "Cuvie",
    "横槍メンゴ",
    "Redrop",
    "うどんや（鬼月あるちゅ）",
    "井ノ本リカ子",
    "いとうえい",
    "上田裕",
    "印度カリー",
    "Reco",
    "三上ミカ",
    "柚木N’",
    "亀吉いちこ（いちこ）",
    "宮本一佐",
    "かいとうぴんく",
    "百済内創（葉月京）",
    "和六里ハル",
    "ななせめるち",
    "うすべに桜子",
    "海野やよい",
    "鈴玉レンリ",
    "小川ひだり",
    "東鉄神",
    "あずまゆき",
    "知るかバカうどん",
    "うた乃",
    "雲吞めお",
];

const DATA_WKI: &'static [&'static str] = &[
    "クリムゾン",
    "よしろん（鎖キャタピラ）",
    "長瀬徹（揚げナス）",
    "Hisasi",
    "小島紗",
    "佐伯",
    "岡田コウ",
    "Cuvie",
    "横槍メンゴ",
    "Redrop",
    "うどんや（鬼月あるちゅ）",
    "井ノ本リカ子",
    "いとうえい",
    "上田裕",
    "印度カリー",
    "Reco",
    "三上ミカ",
    "柚木N’",
    "亀吉いちこ（いちこ）",
    "宮本一佐",
    "かいとうぴんく",
    "百済内創（葉月京）",
    "和六里ハル",
    "ななせめるち",
    "うすべに桜子",
    "海野やよい",
    "鈴玉レンリ",
    "小川ひだり",
    "東鉄神",
    "あずまゆき",
    "知るかバカうどん",
    "うた乃",
    "雲吞めお",
];

#[derive(Debug)]
pub struct Mangaka {
    pub name: String,
    pub alias: String,
    pub group: String,
    pub gender: String,
    pub r18: bool,
    pub born: String,
}

pub fn get_ero_manage_woman_authors() {
    for name in DATA_WKI {
        let (name1, name2) = handle_author_name(&name);

        crate::sqlite_handler::save_actor_mangaka_data(name1.to_string(), name2.to_string());
        // if let Ok(d) = site::wikipedia::get_info_text(author_name) {
        //     println!("{}", d);
        // }
    }
}

fn handle_author_name(name: &str) -> (&str, &str) {
    if name.contains("（") {
        return (name, "");
    } else {
        let ns = name.split("（").collect::<Vec<_>>();
        return (ns.get(0).unwrap(), if ns.get(1).is_none(){""}else{ns.get(1).unwrap()});
    }
}
