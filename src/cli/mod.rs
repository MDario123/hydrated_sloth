use chrono::{DateTime, Local};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name="Hydrated Sloth", version="0.1.0", author="Manuel & Alberto", about="Minimalistic widget to track sleep and hydration.", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub subcommand: Subcomm,
}

#[derive(Subcommand, Clone, Debug, PartialEq)]
pub enum Subcomm {
    #[command(about = "Start the gui.")]
    Gui,
    #[command(about = "Update water information.")]
    Water,
    #[command(about = "Update sleep information.")]
    Sleep {
        #[arg(
            long_help = "Went to sleep at this point in time. Assumed to wake up right now.
Must be in format RFC 3339. Recommended to generate with `date`.
e.g. `date -d '8 hours ago' --rfc-3339 c`"
        )]
        from: DateTime<Local>,
    },
}
