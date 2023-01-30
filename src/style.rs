use fltk::enums::Color;
use fltk::image::{BmpImage, JpegImage, PngImage, RgbImage};
use std::fmt;
use std::fmt::{Debug, Formatter};

/// A window icon.
///
/// # Examples
///
/// ```
/// use panicui::fltk::image::PngImage;
/// use panicui::style::Icon;
///
/// const ICON_DATA: &[u8] = include_bytes!("../assets/images/icon.png");
///
/// let icon = Icon::Png(PngImage::from_data(ICON_DATA).unwrap());
/// ```
#[derive(Clone)]
pub enum Icon {
    Png(PngImage),
    Jpeg(JpegImage),
    Bmp(BmpImage),
    Rgb(RgbImage),
}

impl Debug for Icon {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Png(_) => "PNG icon",
                Self::Jpeg(_) => "JPEG icon",
                Self::Bmp(_) => "BMP icon",
                Self::Rgb(_) => "RGB icon",
            }
        )
    }
}

/// UI styling struct.
///
/// Provides a light theme in the [Default] implementation.
#[derive(Debug, Clone)]
pub struct Style {
    fg: Color,
    bg: Color,
    bg_2: Color,
    title: String,
    icon: Option<Icon>,
}

impl Style {
    /// # Arguments
    ///
    /// 1. `fg` - the foreground text color
    /// 2. `bg` - the background text color
    /// 3. `bg_2` - the text display background color
    /// 4. `title` - the window title
    /// 5. `icon` - the window icon
    pub const fn new(fg: Color, bg: Color, bg_2: Color, title: String, icon: Option<Icon>) -> Self {
        Self {
            fg,
            bg,
            bg_2,
            title,
            icon,
        }
    }

    /// Get the window title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get the foreground text color.
    pub const fn fg(&self) -> Color {
        self.fg
    }

    /// Get the background text color.
    pub const fn bg(&self) -> Color {
        self.bg
    }

    /// Get the text display background color.
    pub const fn bg_2(&self) -> Color {
        self.bg_2
    }

    /// Get the window icon.
    pub const fn icon(&self) -> Option<&Icon> {
        self.icon.as_ref()
    }

    /// Set the foreground text color.
    pub fn set_fg(&mut self, fg: Color) {
        self.fg = fg;
    }

    /// Set the background text color.
    pub fn set_bg(&mut self, bg: Color) {
        self.bg = bg;
    }

    /// Set the background color of the text display.
    pub fn set_bg_2(&mut self, bg_2: Color) {
        self.bg_2 = bg_2;
    }

    /// Set the window title.
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// Set the window icon.
    pub fn set_icon(&mut self, icon: Option<Icon>) {
        self.icon = icon;
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new(
            Color::from_hex(0x222222),
            Color::from_hex(0xf0f0f0),
            Color::from_hex(0xffffff),
            "Crash Report".to_string(),
            None,
        )
    }
}
