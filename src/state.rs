use chrono::{DateTime, Duration, Local};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::io::Write;
use std::path::Path;

#[serde_as]
#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct State {
    #[serde_as(as = "Vec<(_, serde_with::DurationSeconds<i64>)>")]
    pub sleep: Vec<(DateTime<Local>, Duration)>,
    pub water: Vec<DateTime<Local>>,
}

pub fn load_state(state_file: &Path) -> Result<State, std::io::Error> {
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(state_file)?;
    file.write_all(b"")?;
    let raw_state = std::fs::read_to_string(state_file)?;
    let state: State =
        serde_json::from_str(raw_state.as_str()).unwrap_or_else(|_| State::default());
    Ok(state)
}

pub fn save_state(state: &State, state_file: &Path) -> Result<(), std::io::Error> {
    let raw_state = serde_json::to_string(&state).expect("Failed to serialize state");
    std::fs::write(state_file, raw_state)?;
    Ok(())
}
