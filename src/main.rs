use std::io::Write;
use std::path::Path;

use chrono::{DateTime, Duration, Local};
use clap::Parser;
use druid::widget::{Button, Flex, Label};
use druid::Data;
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Clone, Data, Deserialize, Serialize, Debug, Default)]
struct State {
    #[data(eq)]
    #[serde_as(as = "Vec<(_, serde_with::DurationSeconds<i64>)>")]
    sleep: Vec<(DateTime<Local>, Duration)>,
    #[data(eq)]
    water: Vec<DateTime<Local>>,
}

#[derive(Parser, Debug)]
#[command(version="0.1.0", about="Minimalistic widget to track sleep and hydration.", long_about = None)]
struct Args {
    #[arg(short, long)]
    water: bool,

    #[arg(short, long)]
    gui: bool,
}

fn main() -> Result<(), PlatformError> {
    let state_file: &Path = Path::new("State");

    let args = Args::parse();

    // Load state from file
    {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(state_file)
            .expect("Failed to open state file");
        file.write_all(b"").expect("Failed to write state file");
    }
    let raw_state = std::fs::read_to_string(state_file).expect("Failed to read state file");

    let mut state: State =
        serde_json::from_str(raw_state.as_str()).unwrap_or_else(|_| State::default());

    if args.gui {
        let main_window = WindowDesc::new(ui_builder());
        let data = 0_u32;
        return AppLauncher::with_window(main_window)
            .log_to_console()
            .launch(data);
    }

    if args.water {
        state.water.push(Local::now());
        println!("Just drank water!💖💖💖");
    }

    #[cfg(debug_assertions)]
    println!("Saving state... {:?}", &state);

    // Write state to a file
    let raw_state = serde_json::to_string(&state).expect("Failed to serialize state");
    std::fs::write(state_file, raw_state).expect("Failed to write state file");

    Ok(())
}

fn ui_builder() -> impl Widget<u32> {
    // The label text will be computed dynamically based on the current locale and count
    let text =
        LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());
    let label = Label::new(text).padding(5.0).center();
    let button = Button::new("increment")
        .on_click(|_ctx, data, _env| *data += 1)
        .padding(5.0);

    Flex::column().with_child(label).with_child(button)
}
