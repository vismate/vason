#![doc = include_str!("../README.md")]
pub mod canvas;
pub mod color;
pub mod ppm;

#[cfg(feature = "pen-api")]
pub mod pen;
#[cfg(feature = "pen-api")]
pub use pen::Pen;

#[cfg(feature = "shape-api")]
pub mod shape;

pub use canvas::Canvas;
pub use color::Color;
