use crate::{prelude::Result, pkg::conf::settings, pkg::state::AppState};
use standard_error::StandardError;
use reqwest;

pub async fn handler(_state: AppState, _req_id: Option<&str>) -> Result<Vec<u8>> {
    let res = reqwest::get("https://official-joke-api.appspot.com/jokes/random")
        .await.map_err(|e|StandardError::new("ERR-001"))?
        .bytes()
        .await.map_err(|e| StandardError::new("ERR-002"))?;
    Ok(res.into())
}

