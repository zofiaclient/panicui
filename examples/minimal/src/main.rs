#![windows_subsystem = "windows"]

use panicui::app::PanicApplication;
use panicui::style::Style;
use panicui::window::PanicWindow;
use std::backtrace::Backtrace;
use std::panic;
use std::panic::PanicInfo;

fn panic_hook(info: &PanicInfo) {
    let backtrace = Backtrace::force_capture();
    let crash_text = format!("{info}\n{backtrace}");

    let win = PanicWindow::new(Style::default(), crash_text);

    let mut app = PanicApplication::new(win);
    app.run().expect("Unable to run application.");
}

fn main() {
    panic::set_hook(Box::new(panic_hook));

    // Cause some random panic.
    Result::<(), _>::Err(5i32).unwrap();
}
