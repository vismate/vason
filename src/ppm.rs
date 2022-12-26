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

    // instead of calling write on all pixels, we create chunks of 8192 bytes.
    // this significantly increases performance even without the use of BufWriters.
    // the number 8192 is taken from std::sys_common::io::DEFAULT_BUF_SIZE
    // TODO: find a more reliable way of choosing a default chunk size (that also works well on other targets)
    let mut data_chunks = buffer
        .iter()
        .flat_map(|p| p.to_be_bytes().into_iter().skip(1))
        .array_chunks::<8192>();

    for chunk in data_chunks.by_ref() {
        w.write_all(chunk.as_slice())?;
    }

    if let Some(chunk) = data_chunks.into_remainder() {
        w.write_all(chunk.as_slice())?;
    }

    Ok(())
}
