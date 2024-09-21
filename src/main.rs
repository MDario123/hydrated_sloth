use clap::Parser;
use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc};

#[derive(Parser, Debug)]
#[command(version="0.1.0", about="Minimalistic widget to track sleep and hydration.", long_about = None)]
struct Args {
    #[arg(short, long)]
    water: bool,

    #[arg(short, long)]
    gui: bool,
}

fn main() -> Result<(), PlatformError> {
    let args = Args::parse();

    if args.gui {
        let main_window = WindowDesc::new(ui_builder());
        let data = 0_u32;
        return AppLauncher::with_window(main_window)
            .log_to_console()
            .launch(data);
    }

    if args.water {
        println!("Just drank water!");
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
