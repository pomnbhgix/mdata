use reqwest;
use anyhow::Result;
use std::collections::HashMap;


pub fn get_response(url: &str) -> Result<reqwest::blocking::Response,reqwest::Error> {
    let res = reqwest::blocking::get(url)?;

    Ok(res)
}

pub fn get_text_response(url: &str) -> Result<String> {

    let response = get_response(url)?;

    if response.status() == reqwest::StatusCode::OK {
       return Ok(response.text()?)
    }

    Ok(String::new())
}
