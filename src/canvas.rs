use crate::{Color, Pen};

pub struct Canvas {
    buffer: Box<[u32]>,
    width: usize,
    height: usize,
}

impl Canvas {
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height].into_boxed_slice(),
            width,
            height,
        }
    }

    /// Creates a canvas from a pre-allocated buffer.
    ///
    /// # Errors
    ///
    /// This function will return an error if width and height does not match the size of the supplied buffer.
    pub fn from_buffer(buffer: Box<[u32]>, width: usize, height: usize) -> Result<Self, String> {
        if width * height != buffer.len() {
            return Err("buffer size does not match supplied width and height".into());
        }

        Ok(Self {
            buffer,
            width,
            height,
        })
    }

    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    #[must_use]
    pub fn height(&self) -> usize {
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

    #[must_use]
    pub fn pen(&mut self) -> Pen<'_> {
        Pen::new(self)
    }

    /// Clear the entire buffer with supplied color.
    pub fn clear(&mut self, color: impl Into<Color>) {
        let raw_color = u32::from(color.into());
        self.buffer.fill(raw_color);
    }

    #[inline]
    pub fn set_pixel(&mut self, x: i32, y: i32, color: impl Into<Color>) {
        let (self_width, self_height) = self.dimensions_clamped_i32();

        if 0 <= x && x < self_width && 0 <= y && y < self_height {
            // SAFETY: idx is known to be positive and within bounds.
            unsafe {
                self.set_pixel_unchecked_raw_i32(x, y, u32::from(color.into()));
            }
        }
    }

    /// # Safety
    /// x and y must be positive and smaller than canvas width and height respectively.
    #[inline]
    pub unsafe fn set_pixel_unchecked(&mut self, x: i32, y: i32, color: impl Into<Color>) {
        self.set_pixel_unchecked_raw_i32(x, y, u32::from(color.into()));
    }

    #[allow(clippy::cast_sign_loss)]
    pub fn fill_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: impl Into<Color>) {
        let raw_color = u32::from(color.into());
        let (from_x, to_x, from_y, to_y) = self.clamp_rect_i32(x, x + w, y, y + h);

        let offset = from_y as usize * self.width;
        let mut from_idx = offset + from_x as usize;
        let mut to_idx = offset + to_x as usize;

        for _ in from_y..to_y {
            self.buffer[from_idx..to_idx].fill(raw_color);
            from_idx += self.width;
            to_idx += self.width;
        }
    }

    #[allow(clippy::cast_sign_loss)]
    pub fn fill_circle(&mut self, x: i32, y: i32, r: i32, color: impl Into<Color>) {
        let raw_color = u32::from(color.into());
        let (from_x, to_x, from_y, to_y) = self.clamp_rect_i32(x - r, x + r, y - r, y + r);

        let r2 = r * r;
        for j in from_y..to_y {
            let dy = j - y;
            let dy2 = dy * dy;

            let offset = j as usize * self.width;
            for i in from_x..to_x {
                let dx = i - x;

                if dx * dx + dy2 <= r2 {
                    // SAFETY: idx is known to be positive and within bounds.
                    unsafe {
                        *self.buffer.get_unchecked_mut(offset + i as usize) = raw_color;
                    }
                }
            }
        }
    }

    pub fn line(&mut self, mut x1: i32, mut y1: i32, x2: i32, y2: i32, color: impl Into<Color>) {
        let raw_color = u32::from(color.into());

        let dx = x2 - x1;
        let sx = dx.signum();
        let dx = dx.abs();

        let dy = y2 - y1;
        let sy = dy.signum();
        let dy = -dy.abs();

        let mut err = dx + dy;

        let (self_width, self_height) = self.dimensions_clamped_i32();

        loop {
            if 0 <= x1 && x1 < self_width && 0 <= y1 && y1 < self_height {
                unsafe {
                    self.set_pixel_unchecked_raw_i32(x1, y1, raw_color);
                }
            }

            if x1 == x2 && y1 == y2 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x1 += sx;
            }
            if e2 <= dx {
                err += dx;
                y1 += sy;
            }
        }
    }

    pub fn flood_fill(&mut self, x: i32, y: i32, color: impl Into<Color>) {
        //TODO: refactor this after merge
        let (clamped_width, clamped_height) = self.dimensions_clamped_i32();

        if 0 <= x && x < clamped_width && 0 <= y && y < clamped_height {
            let raw_color = u32::from(color.into());
            let xu = x as usize;
            let yu = y as usize;
            let seed_color = self.buffer[yu * self.width + xu];
            if seed_color != raw_color {
                self.flood_fill_start(xu, yu, seed_color, raw_color);
            }
        }
    }

    fn flood_fill_start(&mut self, mut x: usize, mut y: usize, seed_color: u32, raw_color: u32) {
        loop {
            let ox = x;
            let oy = y;

            while y != 0 && self.buffer[(y - 1) * self.width + x] == seed_color {
                y -= 1;
            }
            while x != 0 && self.buffer[y * self.width + (x - 1)] == seed_color {
                x -= 1;
            }

            if x == ox && y == oy {
                break;
            }
        }

        self.flood_fill_core(x, y, seed_color, raw_color);
    }

    fn flood_fill_core(&mut self, mut x: usize, mut y: usize, seed_color: u32, raw_color: u32) {
        let mut last_row_len = 0;

        loop {
            let mut row_len = 0;
            let mut sx = x;

            if last_row_len != 0 && self.buffer[y * self.width + x] != seed_color {
                loop {
                    last_row_len -= 1;
                    if last_row_len == 0 {
                        return;
                    }
                    x += 1;
                    if self.buffer[y * self.width + x] == seed_color {
                        break;
                    }
                }
                sx = x;
            } else {
                while x != 0 && self.buffer[y * self.width + x - 1] == seed_color {
                    x -= 1;
                    self.buffer[y * self.width + x] = raw_color;
                    if y != 0 && self.buffer[(y - 1) * self.width + x] == seed_color {
                        self.flood_fill_start(x, y - 1, seed_color, raw_color);
                    }
                    row_len += 1;
                    last_row_len += 1;
                }
            }

            while sx < self.width && self.buffer[y * self.width + sx] == seed_color {
                self.buffer[y * self.width + sx] = raw_color;
                row_len += 1;
                sx += 1;
            }

            if row_len < last_row_len {
                let end = x + last_row_len;

                loop {
                    sx += 1;
                    if sx >= end {
                        break;
                    }
                    if self.buffer[y * self.width + sx] == seed_color {
                        self.flood_fill_core(sx, y, seed_color, raw_color);
                    }
                }
            } else if row_len > last_row_len && y != 0 {
                let mut ux = x + last_row_len;
                loop {
                    ux += 1;
                    if ux >= sx {
                        break;
                    }
                    if self.buffer[(y - 1) * self.width + ux] == seed_color {
                        self.flood_fill_start(ux, y - 1, seed_color, raw_color);
                    }
                }
            }

            last_row_len = row_len;

            y += 1;
            if last_row_len == 0 || y >= self.height {
                break;
            }
        }
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    #[inline]
    fn dimensions_clamped_i32(&self) -> (i32, i32) {
        let w = self.width.clamp(0, i32::MAX as usize) as i32;
        let h = self.height.clamp(0, i32::MAX as usize) as i32;

        (w, h)
    }

    #[allow(clippy::similar_names)]
    #[inline]
    fn clamp_rect_i32(&self, xmin: i32, xmax: i32, ymin: i32, ymax: i32) -> (i32, i32, i32, i32) {
        let (self_width, self_height) = self.dimensions_clamped_i32();

        let from_x = xmin.clamp(0, self_width - 1);
        let to_x = xmax.clamp(from_x, self_width);

        let from_y = ymin.clamp(0, self_height - 1);
        let to_y = ymax.clamp(from_y, self_height);

        (from_x, to_x, from_y, to_y)
    }

    #[allow(clippy::cast_sign_loss)]
    #[inline]
    unsafe fn set_pixel_unchecked_raw_i32(&mut self, x: i32, y: i32, raw_color: u32) {
        debug_assert!(x >= 0 && y >= 0);
        let idx = y as usize * self.width + x as usize;

        debug_assert!(idx < self.buffer.len());
        *self.buffer.get_unchecked_mut(idx) = raw_color;
    }
}
