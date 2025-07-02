use crate::{prelude::Result, pkg::conf::settings, pkg::state::AppState};
use uuid::Uuid;
use serde::Deserialize;

#[derive(Deserialize)]
struct Joke {
    setup: String,
    punchline: String,
}

pub async fn handler(_state: AppState, _req_id: Option<&str>) -> Result<String> {
    let res = reqwest::get("https://official-joke-api.appspot.com/jokes/random")
        .await?
        .json::<Joke>()
        .await?;

    Ok(format!("{} - {}", res.setup, res.punchline).to_vec())
}
