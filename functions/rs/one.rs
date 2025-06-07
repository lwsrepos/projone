use crate::{prelude::Result, pkg::conf::settings, pkg::state::AppState};

pub async fn handler(state: AppState) -> Result<Vec<u8>>{
  Ok("one".into())
}
