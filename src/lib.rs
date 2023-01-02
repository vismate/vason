#![doc = include_str!("../README.md")]
pub mod canvas;
pub mod color;
pub mod pen;
mod pixel_access;
pub mod ppm;

pub type Canvas<'a> = canvas::Canvas<'a, pixel_access::NoAlphaAccess>;
pub type CanvasWithAlpha<'a> = canvas::Canvas<'a, pixel_access::AlphaAccess>;

pub type Pen<'a, 'b> = pen::Pen<'a, 'b, pixel_access::NoAlphaAccess>;
pub type PenWithAlpha<'a, 'b> = pen::Pen<'a, 'b, pixel_access::AlphaAccess>;

pub use color::Color;
