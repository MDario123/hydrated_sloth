use crate::State;
use chrono::{DateTime, Datelike, Duration, Local, Timelike};
use druid::widget::{SizedBox, ZStack};
use druid::{Color, UnitPoint, Vec2, Widget, WidgetExt};

const CONTAINER_WIDTH: f64 = 80.0;

const CONTAINER_HEIGHT: f64 = 40.0;

const BACKGROUND_COLOR: Color = Color::rgb8(0x69, 0x69, 0x69);

const MULTIPLIER_PARITY: f64 = 0.95;

const MULTIPLIER_OFF_MONTH: f64 = 0.60;

const SLEEP_COLOR: Color = Color::rgb8(0xA0, 0xE0, 0xA0);

const WATER_COLOR: Color = Color::rgb8(0x40, 0xE0, 0xD0);

const WATER_WIDTH: f64 = 5.0;

pub(crate) fn day_square(state: &State, date: &DateTime<Local>, parity: u32) -> impl Widget<()> {
    let is_day_in_curr_month = date.month() == Local::now().month();
    let mult = if is_day_in_curr_month {
        1.0
    } else {
        MULTIPLIER_OFF_MONTH
    };
    let mult = mult
        * if parity % 2 == 1 {
            MULTIPLIER_PARITY
        } else {
            1.0
        };

    let (background_color, water_color, sleep_color) = (
        get_multiplied_color(BACKGROUND_COLOR, mult),
        get_multiplied_color(WATER_COLOR, mult),
        get_multiplied_color(SLEEP_COLOR, mult),
    );

    let mut zstack = ZStack::new(
        SizedBox::empty()
            .width(CONTAINER_WIDTH)
            .height(CONTAINER_HEIGHT)
            .background(background_color),
    );

    for water in &state.water {
        if water.date_naive() == date.date_naive() {
            let offset = get_units_from_date(water).min(CONTAINER_WIDTH - WATER_WIDTH);

            zstack = zstack.with_child(
                SizedBox::new(
                    SizedBox::empty()
                        .height(CONTAINER_HEIGHT)
                        .width(WATER_WIDTH)
                        .background(water_color),
                ),
                Vec2::new(1.0, 1.0),
                Vec2::ZERO,
                UnitPoint::LEFT,
                Vec2::new(offset, 0.0),
            );
        }
    }

    for sleep in &state.sleep {
        if sleep.0.date_naive() == date.date_naive() {
            let offset = get_units_from_date(&sleep.0);
            let width = get_units_from_duration(&sleep.1);

            zstack = zstack.with_child(
                SizedBox::empty()
                    .height(CONTAINER_HEIGHT)
                    .width(width)
                    .background(sleep_color),
                Vec2::new(1.0, 1.0),
                Vec2::ZERO,
                UnitPoint::LEFT,
                Vec2::new(offset, 0.0),
            );
        }
    }

    zstack
}

fn get_units_from_date(date: &DateTime<Local>) -> f64 {
    CONTAINER_WIDTH
        * ((date.hour() as f64 * 60.0 + date.minute() as f64) * 60.0 + date.second() as f64)
        / (24.0 * 60.0 * 60.0)
}

fn get_units_from_duration(duration: &Duration) -> f64 {
    CONTAINER_WIDTH * duration.num_seconds() as f64 / (24.0 * 60.0 * 60.0)
}

fn get_multiplied_color(color: Color, mult: f64) -> Color {
    let (r, g, b, a) = color.as_rgba();
    Color::rgba(r * mult, g * mult, b * mult, a)
}
