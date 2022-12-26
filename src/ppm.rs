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
    writeln!(w, "P6 {width} {height} 255")?;

    for p in buffer {
        let [b, g, r, _] = p.to_le_bytes();
        w.write_all(&[r, g, b])?;
    }

    Ok(())
}
