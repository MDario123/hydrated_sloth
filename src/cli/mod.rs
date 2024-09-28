use std::str::FromStr;

use chrono::{DateTime, Local, TimeZone};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name="Hydrated Sloth", 
    version="0.2.0", 
    author="Manuel & Alberto", 
    about="Minimalistic widget to track sleep and hydration.", 
    long_about = None
)]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: Subcomm,
}

#[derive(Subcommand, Clone, Debug, PartialEq)]
pub enum Subcomm {
    #[command(about = "Start the gui.")]
    Gui,
    #[command(about = "Update water information.")]
    Water {
        #[arg(
            long_help = "Drank at this point in time. Supports formats like '20 minutes ago' or '5 min ago'."
        )]
        at: Option<Time>,
    },
    #[command(about = "Update sleep information.")]
    Sleep {
        #[arg(
            long_help = "Went to sleep at this point in time. Assumed to wake up right now. Supports formats like '11:00 PM yesterday'"
        )]
        from: Time,
    },
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Time {
    pub date: DateTime<Local>,
}

impl FromStr for Time {
    type Err = fuzzydate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date = fuzzydate::parse(s)?;
        let date = Local.from_local_datetime(&date).unwrap();
        Ok(Time { date })
    }
}
