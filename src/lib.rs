//! # Description
//!
//! [panicui](https://crates.io/crate/panicui) provides a simple solution to easier debugging by utilizing user interface.
//!
//! ## 1. Extremely lightweight
//!
//! Average binary size (no strip, with debug symbols): ~1,010KB (0.98MB)
//!
//! ## 2. Fast build times
//!
//! panicui only uses one direct dependency, [fltk-rs](https://crates.io/crate/fltk).
//!
//! ## 3. Portable across every platform
//!
//! panicui uses [fltk](https://www.fltk.org/) for UI, which works across practically every platform. This includes:
//!
//! - UNIX
//! - Linux (X11)
//! - Windows
//! - MacOS X
//!
//! # Getting started
//!
//! Get started by creating a [PanicApplication](app::PanicApplication).

pub extern crate fltk;

pub mod app;
pub mod style;
pub mod window;

#[cfg(test)]
mod tests;
