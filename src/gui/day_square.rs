use crate::State;
use chrono::{DateTime, Duration, Local, Timelike};
use druid::widget::{SizedBox, ZStack};
use druid::{Color, UnitPoint, Vec2, Widget, WidgetExt};

const CONTAINER_WIDTH: f64 = 100.0;

const CONTAINER_HEIGHT: f64 = 50.0;

const BACKGROUND_COLOR: Color = Color::rgb8(0xFF, 0xFF, 0xFF);

const MULTIPLIER: f64 = 0.95;

const SLEEP_COLOR: Color = Color::rgb8(0x00, 0x80, 0x00);

const WATER_COLOR: Color = Color::rgb8(0x00, 0x00, 0xFF);

const WATER_WIDTH: f64 = 5.0;

pub(crate) fn day_square(state: &State, date: &DateTime<Local>, parity: u32) -> impl Widget<()> {
    let (background_color, water_color, sleep_color) = if parity % 2 == 0 {
        (BACKGROUND_COLOR, WATER_COLOR, SLEEP_COLOR)
    } else {
        (
            get_multiplied_color(BACKGROUND_COLOR),
            get_multiplied_color(WATER_COLOR),
            get_multiplied_color(SLEEP_COLOR),
        )
    };

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

fn get_multiplied_color(color: Color) -> Color {
    let (r, g, b, a) = color.as_rgba();
    Color::rgba((r * MULTIPLIER), (g * MULTIPLIER), (b * MULTIPLIER), a)
}
