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

    let mut tmp_buffer = vec![0u8; 8192];
    for chunk in buffer.chunks(2048) {
        chunk
            .iter()
            .flat_map(|p| p.to_be_bytes().into_iter().skip(1))
            .enumerate()
            .for_each(|(i, b)| unsafe { *tmp_buffer.get_unchecked_mut(i) = b });
        w.write_all(&tmp_buffer)?;
    }

    Ok(())
}
