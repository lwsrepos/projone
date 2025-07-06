use crate::{prelude::Result, pkg::state::AppState};
use standard_error::StandardError;

pub async fn handler(_state: AppState, _req_id: Option<&str>) -> Result<Vec<u8>> {
    let res = reqwest::get("https://official-joke-api.appspot.com/jokes/random")
        .await?
        .bytes()
        .await?; 
    Ok(res.into())
} 

