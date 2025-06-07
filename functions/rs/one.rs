use crate::{prelude::Result, conf::settings, pkg::state::AppState};

fn handler(state: AppState) -> Result<Vec<u8>>{
  Ok("one".into())
}
