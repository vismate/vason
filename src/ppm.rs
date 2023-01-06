//! The ppm module allows the user to save a canvas or a plain buffer to a file (or write it to anything that implements Write)
//! The ppm file format is one of the most simple ones. That is why it's included here. Not all image viewers support the format
//! but the major ones usually do. (so do web browsers)
//! # Example
//! ```rust
//! use vason::{Canvas, Color, ppm::encode_canvas};
//! use std::fs::File;
//!
//! let mut buffer = vec![0u32; 64*64];
//! let mut canvas = Canvas::new(&mut buffer, 64, 64);
//! canvas.clear(Color::BLUE);
//! // ...
//!
//! let mut file = File::create("canvas.ppm").expect("could not create file");
//! encode_canvas(&canvas, &mut file).expect("could not write image to file");
//! ```

use crate::Canvas;
use std::io::{Result, Write};

/// Convenience function to encode a canvas to ppm format.
/// ppm is supported by some main-stream image editors.
///
/// # Errors
///
/// This function will return an error if there was an i/o error whilest writing.
pub fn encode_canvas(canvas: &Canvas, w: &mut dyn Write) -> Result<()> {
    encode_buffer(canvas.buffer(), canvas.width(), canvas.height(), w)
}

/// Encodes a buffer to ppm format.
/// ppm is supported by some main-stream image editors.
///
/// # Errors
///
/// This function will return an error if there was an i/o error whilest writing.
pub fn encode_buffer(buffer: &[u32], width: usize, height: usize, w: &mut dyn Write) -> Result<()> {
    #[allow(clippy::uninlined_format_args)]
    writeln!(w, "P6 {} {} 255", width, height)?;

    // instead of calling write on all pixels, we create chunks.
    // this significantly increases performance even without the use of BufWriters.
    // TODO: find a more reliable way of choosing a default chunk size (that also works well on other targets)

    // every pixel is represiented with three bytes so we skip the alpha channel.
    // so our write chunk size is 2048 * 3 = 6144
    // TODO: is this too janky? Should just the user use BufWriters?
    let mut tmp_buffer = vec![0u8; 6144];
    for chunk in buffer.chunks(2048) {
        chunk
            .iter()
            .flat_map(|p| p.to_be_bytes().into_iter().skip(1))
            .enumerate()
            .for_each(|(i, b)| tmp_buffer[i] = b);
        w.write_all(&tmp_buffer)?;
    }

    Ok(())
}
