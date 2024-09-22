mod cli;
mod gui;
mod state;

use chrono::Local;
use druid::PlatformError;
use gui::gui;
use std::path::Path;

use clap::Parser;
use cli::{Args, Subcomm};
use state::{load_state, save_state, State};

fn update_state(state: &mut State, args: &Args) {
    match args.subcommand {
        Subcomm::Water => {
            state.water.push(Local::now());
            println!("Just drank water!💖💖💖");
        }
        Subcomm::Sleep { from } => {
            let now = Local::now();
            assert!(from <= now, "You come from the future bro?");
            let sleep = (from, now - from);
            state.sleep.push(sleep);
            println!("Just woke up!🕒🕒🕒");
        }
        Subcomm::Gui => unreachable!(),
    }
}

fn main() -> Result<(), PlatformError> {
    let state_file: &Path = Path::new("State");

    let args = Args::parse();
    let mut state = load_state(state_file).expect("Failed to load state");

    if args.subcommand == Subcomm::Gui {
        return gui(state);
    } else {
        update_state(&mut state, &args);

        #[cfg(debug_assertions)]
        println!("Saving state... {:?}", &state);

        save_state(&state, state_file).expect("Failed to save state");
    }

    Ok(())
}
