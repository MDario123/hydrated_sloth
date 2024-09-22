mod gui;

use chrono::{DateTime, Duration, Local};
use druid::{Data, PlatformError};
use gui::gui;

#[derive(Clone, Data)]
struct State {
    #[data(eq)]
    sleep: Vec<(DateTime<Local>, Duration)>,
    #[data(eq)]
    water: Vec<DateTime<Local>>,
}

fn main() -> Result<(), PlatformError> {
    let data = State {
        sleep: vec![
            (Local::now() + Duration::hours(20), Duration::hours(8)),
            (Local::now() - Duration::hours(4), Duration::hours(8)),
        ],
        water: vec![DateTime::parse_from_rfc3339("2024-09-15T23:59:59+02:00")
            .unwrap()
            .into()],
    };

    gui(data)
}
