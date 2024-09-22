use crate::gui::day_square::day_square;
use crate::State;
use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, NaiveTime, TimeZone, Timelike};
use druid::widget::Flex;
use druid::{Widget, WidgetExt};

const HORIZONTAL_PADDING: f64 = 0.0;

const VERTICAL_PADDING: f64 = 10.0;

pub(crate) fn calendar(state: State) -> impl Widget<()> {
    let mut calendar = Flex::column();
    let now = Local::now();
    let weeks_in_month = weeks_in_month(&now);
    let calendar_first_date = get_calendar_first_date(&now);

    let mut validated_state = State {
        sleep: vec![],
        water: state.water.clone(),
    };

    for &sleep in &state.sleep {
        let sleep_end = sleep.0 + sleep.1;
        if sleep.0.day() != sleep_end.day() {
            let next_date = sleep_end
                .with_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
                .unwrap();
            validated_state.sleep.push((sleep.0, next_date - sleep.0));
            validated_state.sleep.push((
                next_date,
                Duration::seconds(sleep_end.num_seconds_from_midnight() as i64),
            ));
        } else {
            validated_state.sleep.push(sleep);
        }
    }

    for week in 0..weeks_in_month {
        let mut week_container = Flex::row();
        for day_of_week in 0..7 {
            let date = calendar_first_date + Duration::days((week * 7 + day_of_week) as i64);
            week_container = week_container.with_child(
                day_square(&validated_state, &date, week * 7 + day_of_week)
                    .padding((HORIZONTAL_PADDING, VERTICAL_PADDING)),
            );
        }
        calendar = calendar.with_child(week_container);
    }
    calendar
}

fn get_last_day_of_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => unreachable!(),
    }
}

fn weeks_in_month(datetime: &DateTime<Local>) -> u32 {
    // Get the year and month
    let year = datetime.year();
    let month = datetime.month();

    let first_day_of_month = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let last_day_of_month =
        NaiveDate::from_ymd_opt(year, month, get_last_day_of_month(year, month)).unwrap();

    // Calculate the week numbers for the first and last days
    let first_week = first_day_of_month.iso_week().week();
    let last_week = last_day_of_month.iso_week().week();

    // Handle the case where weeks cross over the end of the year (e.g., December)
    if last_week < first_week {
        last_week + (52 - first_week + 1)
    } else {
        last_week - first_week + 1
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn get_calendar_first_date(datetime: &DateTime<Local>) -> DateTime<Local> {
    let year = datetime.year();
    let month = datetime.month();
    let first_day_of_month = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let first_day_of_month_weekday = first_day_of_month.weekday().num_days_from_monday() as i32;
    let calendar_first_date =
        first_day_of_month - Duration::days(first_day_of_month_weekday as i64);
    Local
        .with_ymd_and_hms(
            calendar_first_date.year(),
            calendar_first_date.month(),
            calendar_first_date.day(),
            0,
            0,
            0,
        )
        .unwrap()
}
