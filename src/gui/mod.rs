mod calendar;
mod day_square;

use crate::gui::calendar::calendar;
use crate::State;
use druid::{AppLauncher, PlatformError, WindowDesc};

pub fn gui(state: State) -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(calendar(state));
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(())
}
