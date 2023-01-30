use crate::window::PanicWindow;
use fltk::app::App;
use fltk::prelude::{FltkError, WidgetExt};

/// A panic application.
///
/// # Examples
///
/// ```
/// use panicui::app::PanicApplication;
/// use panicui::style::Style;
/// use panicui::window::PanicWindow;
/// use std::backtrace::Backtrace;
/// use std::panic;
/// use std::panic::PanicInfo;
///
/// fn panic_hook(info: &PanicInfo) {
///     let backtrace = Backtrace::force_capture();
///     let crash_text = format!("{info}\n{backtrace}");
///
///     let win = PanicWindow::new(Style::default(), crash_text);
///
///     let mut app = PanicApplication::new(win);
///     app.run().expect("Unable to run application.");
/// }
///
/// fn main() {
///     panic::set_hook(Box::new(panic_hook));
///
///     // Cause some random panic.
///     Result::<(), _>::Err(5i32).unwrap();
/// }
/// ```
pub struct PanicApplication {
    window: PanicWindow,
    app: App,
}

impl PanicApplication {
    /// Run the application.
    ///
    /// You should be able to unwrap or discard this result.
    pub fn run(&mut self) -> Result<(), FltkError> {
        let mut fltk_win = self.window.build_fltk_win();
        fltk_win.show();

        self.app.run()
    }

    /// Create a new `PanicApplication`.
    pub fn new(window: PanicWindow) -> Self {
        Self {
            window,
            app: App::default(),
        }
    }
}
