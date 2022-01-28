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
    "クリムゾン_(漫画家)",
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
struct Mangaka {
    name: String,
    alias: String,
    group: String,
    gender:String,
    r18:bool,
    born:String
}

pub fn get_ero_manage_woman_authors() {
    for name in DATA_WKI {
        let author_name = handle_author_name(&name);
        // if let Ok(d) = site::wikipedia::get_info_text(author_name) {
        //     println!("{}", d);
        // }
    }
}

fn handle_author_name(name: &str) -> &str {
    if name.contains("（") {
        return name;
    } else {
        let ns = name.split("（").collect::<Vec<_>>();
        if let Some(r) = ns.first() {
            return r;
        }
        return "";
    }
}
