use crate::style::{Icon, Style};
use fltk::app;
use fltk::enums::{Color, Font, FrameType};
use fltk::frame::Frame;

use fltk::prelude::{DisplayExt, WidgetExt, WindowExt};
use fltk::text::{TextBuffer, TextDisplay};
use fltk::window::Window;

fn calc_safe_size() -> (i32, i32) {
    let (w, h) = app::screen_size();
    (((w / 2.4) as i32), (h / 2.4) as i32)
}

/// A panic window, usually displaying information about a panic.
pub struct PanicWindow {
    style: Style,
    text: String,
}

impl PanicWindow {
    /// Create a FLTK [Window].
    pub fn build_fltk_win(&self) -> Window {
        app::set_scrollbar_size(13);

        let (w, h) = calc_safe_size();

        let mut window = Window::default()
            .with_size(w, h)
            .center_screen()
            .with_label(self.style.title());

        let (r, g, b) = self.style.bg().to_rgb();
        app::background(r, g, b);

        if let Some(icon) = self.style.icon() {
            match icon.clone() {
                Icon::Png(i) => window.set_icon(Some(i)),
                Icon::Jpeg(i) => window.set_icon(Some(i)),
                Icon::Bmp(i) => window.set_icon(Some(i)),
                Icon::Rgb(i) => window.set_icon(Some(i)),
            }
        }

        let mut buf = TextBuffer::default();
        buf.set_text(&self.text);

        let frame = Frame::default()
            // Downsize for padding effect.
            .with_size(w - 14, h - 12)
            .center_of_parent();

        let mut display = TextDisplay::default().size_of(&frame).center_of(&frame);

        display.set_buffer(buf);
        display.set_text_font(Font::Helvetica);
        display.set_selection_color(Color::from_hex(0xb3daff));
        display.set_frame(FrameType::EngravedBox);
        display.set_text_color(self.style.fg());
        display.set_color(self.style.bg_2());

        window
    }

    /// Get the crash text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get the style of the window.
    pub const fn style(&self) -> &Style {
        &self.style
    }

    /// Create a new `PanicWindow`.
    pub const fn new(style: Style, text: String) -> Self {
        Self { style, text }
    }
}
