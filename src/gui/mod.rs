mod calendar;
mod day_square;

use crate::gui::calendar::calendar;
use crate::State;
use calendar::{weeks_in_month, HORIZONTAL_PADDING, VERTICAL_PADDING};
use chrono::Local;
use day_square::{CONTAINER_HEIGHT, CONTAINER_WIDTH};
use druid::{AppLauncher, PlatformError, WindowDesc, WindowSizePolicy};

pub fn gui(state: State) -> Result<(), PlatformError> {
    let weeks_in_month = weeks_in_month(&Local::now()) as f64;
    let window_size: (f64, f64) = (
        7.0 * CONTAINER_WIDTH + 2.0 * HORIZONTAL_PADDING,
        weeks_in_month * CONTAINER_HEIGHT + (weeks_in_month + 1.0) * 2.0 * VERTICAL_PADDING,
    );

    let main_window = WindowDesc::new(calendar(state))
        .window_size_policy(WindowSizePolicy::User)
        .window_size(window_size)
        .resizable(false);
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(())
}
