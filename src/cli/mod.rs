use clap::Parser;

#[derive(Parser, Debug)]
#[command(version="0.1.0", about="Minimalistic widget to track sleep and hydration.", long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub water: bool,

    #[arg(short, long)]
    pub gui: bool,
}
