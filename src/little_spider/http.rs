use anyhow::Result;
use reqwest;

pub fn get_response(url: &str) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let res = reqwest::blocking::get(url)?;

    Ok(res)
}

pub fn get_text_response(url: &str) -> Result<String> {
    let response = get_response(url)?;

    if response.status() == reqwest::StatusCode::OK {
        return Ok(response.text()?);
    }

    bail!("response has some question:{}", response.status());
}

pub fn get_url_filename(url: &String) -> Option<String> {
    if let Some(s) = std::path::Path::new(url).file_name() {
        if let Some(st) = s.to_str() {
            return Some(st.to_string());
        }
    }
    Option::None
}
