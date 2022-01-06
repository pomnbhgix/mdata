use serde::{Deserialize, Serialize};

use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct ConfyConfig {
  db_path: String,
}

impl Default for ConfyConfig {
  fn default() -> Self {
    ConfyConfig {
      db_path: "check.db".to_string(),
    }
  }
}

pub fn get_database_path() -> Result<String, Error> {
  if let Ok(config) = confy::load_path("mdata.config") {
    let c: ConfyConfig = config;
    if Path::new(&c.db_path).exists() {
      println!("dbpath:{}", c.db_path);
      return Ok(c.db_path);
    }
  }
  Err(Error::from(ErrorKind::NotFound))
}
