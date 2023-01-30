use panicui::app::PanicApplication;
use panicui::fltk::image::PngImage;
use panicui::style::{Icon, Style};
use panicui::window::PanicWindow;
use std::backtrace::Backtrace;
use std::panic;
use std::panic::PanicInfo;

const ICON_DATA: &[u8] = include_bytes!("../assets/images/icon.png");

fn fmt_crash_text(info: &PanicInfo, backtrace: Backtrace) -> String {
    format!(
        "The process quit unexpectedly.\n\n### Panic Information\n\n{}\n\n### Backtrace\n\n{}",
        info, backtrace
    )
}

fn panic_hook(info: &PanicInfo) {
    let icon = Icon::Png(PngImage::from_data(ICON_DATA).unwrap());

    let mut style = Style::default();
    style.set_icon(Some(icon));

    let backtrace = Backtrace::force_capture();
    let crash_text = fmt_crash_text(info, backtrace);

    let window = PanicWindow::new(style, crash_text);

    let mut app = PanicApplication::new(window);
    app.run().expect("Unable to run panic application.");
}

fn main() {
    panic::set_hook(Box::new(panic_hook));

    // Cause some random panic.
    panic!("the disco. No? Ok, I'll leave.");
}
