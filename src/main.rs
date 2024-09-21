mod cli;
mod state;

use std::path::Path;

use chrono::Local;
use clap::Parser;
use cli::{Args, Subcomm};
use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc};
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
        let main_window = WindowDesc::new(ui_builder());
        let data = 0_u32;
        return AppLauncher::with_window(main_window)
            .log_to_console()
            .launch(data);
    } else {
        update_state(&mut state, &args);

        #[cfg(debug_assertions)]
        println!("Saving state... {:?}", &state);

        save_state(&state, state_file).expect("Failed to save state");
    }

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
