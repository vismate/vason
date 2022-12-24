use crate::Color;

pub struct Canvas {
    buffer: Box<[u32]>,
    width: u32,
    height: u32,
}

impl Canvas {
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            buffer: vec![0; (width * height) as usize].into_boxed_slice(),
            width,
            height,
        }
    }

    /// Creates a canvas from a pre-allocated buffer.
    ///
    /// # Errors
    ///
    /// This function will return an error if width and height does not match the size of the supplied buffer.
    pub fn from_buffer(buffer: Box<[u32]>, width: u32, height: u32) -> Result<Self, String> {
        if (width * height) as usize != buffer.len() {
            return Err("buffer size does not match supplied width and height".into());
        }

        Ok(Self {
            buffer,
            width,
            height,
        })
    }

    #[must_use]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[must_use]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[must_use]
    pub fn buffer(&self) -> &[u32] {
        &self.buffer
    }

    #[must_use]
    pub fn buffer_mut(&mut self) -> &mut [u32] {
        &mut self.buffer
    }

    /// Clear the entire buffer with supplied color.
    pub fn clear<C: Into<Color>>(&mut self, color: C) {
        let raw_color = u32::from(color.into());
        self.buffer.fill(raw_color);
    }
}
