use crate::Color;

#[cfg(feature = "pen-api")]
use crate::pen::Pen;

#[cfg(feature = "shape-api")]
use crate::shape::Draw;

pub struct Canvas<'a> {
    buffer: &'a mut [u32],
    width: usize,
    height: usize,
    clamped_width: i32,
    clamped_height: i32,
}

impl<'a> Canvas<'a> {
    /// Creates a new [`Canvas`] with giver width and height.
    /// # Panics
    /// This function panics if the supplied width and height does not match the buffer size.
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    #[must_use]
    pub fn new(buffer: &'a mut [u32], width: usize, height: usize) -> Self {
        assert!(buffer.len() == width * height);
        Self {
            buffer,
            width,
            height,
            clamped_width: width.min(i32::MAX as usize) as i32,
            clamped_height: height.min(i32::MAX as usize) as i32,
        }
    }

    /// Returns the width of this [`Canvas`].
    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of this [`Canvas`].
    #[must_use]
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns a reference to the buffer of this [`Canvas`].
    #[must_use]
    pub fn buffer(&self) -> &[u32] {
        self.buffer
    }

    /// Returns a mutable reference to the buffer of this [`Canvas`].
    #[must_use]
    pub fn buffer_mut(&mut self) -> &mut [u32] {
        self.buffer
    }

    #[cfg(feature = "pen-api")]
    #[must_use]
    pub fn pen(&mut self) -> Pen<'_, 'a> {
        Pen::new(self)
    }

    #[cfg(feature = "shape-api")]
    #[inline]
    pub fn draw(&mut self, drawable: &impl Draw) {
        drawable.draw_to(self);
    }

    /// Clear the entire buffer with supplied color.
    pub fn clear(&mut self, color: impl Into<Color>) {
        let raw_color = u32::from(color.into());
        self.buffer.fill(raw_color);
    }

    /// Sets the pixel at (x, y) of this [`Canvas`] to supplied color.
    #[inline]
    pub fn set_pixel(&mut self, x: i32, y: i32, color: impl Into<Color>) {
        if 0 <= x && x < self.clamped_width && 0 <= y && y < self.clamped_height {
            // SAFETY: idx is known to be positive and within bounds.
            unsafe {
                self.set_pixel_unchecked_raw_i32(x, y, u32::from(color.into()));
            }
        }
    }

    /// Sets the pixel at (x, y) of this [`Canvas`] to supplied color.
    /// # Safety
    /// x and y must be positive and smaller than canvas width and height respectively.
    #[inline]
    pub unsafe fn set_pixel_unchecked(&mut self, x: i32, y: i32, color: impl Into<Color>) {
        self.set_pixel_unchecked_raw_i32(x, y, u32::from(color.into()));
    }

    /// Returns an iterator of pixels and their corresponding x and y coordinates.
    /// ```rust
    /// use vason::{Canvas, Color};
    /// let mut buffer = [0u32; 4];
    /// let mut canvas = Canvas::new(&mut buffer, 2, 2);
    /// canvas.set_pixel(0, 1, Color::RED);
    /// let mut iter = canvas.pixel_iter();
    ///
    /// assert_eq!(Some((0,0,0)), iter.next());
    /// assert_eq!(Some((1,0,0)), iter.next());
    /// assert_eq!(Some((0,1,u32::from(Color::RED))), iter.next());
    /// assert_eq!(Some((1,1,0)), iter.next());
    /// assert_eq!(None, iter.next());   
    /// ```
    pub fn pixel_iter(&self) -> impl Iterator<Item = (usize, usize, u32)> + '_ {
        self.buffer
            .iter()
            .enumerate()
            .map(|(i, p)| (i % self.width, i / self.width, *p))
    }

    /// Returns an iterator of mutable references to pixels and their corresponding x and y coordinates.
    /// ```rust
    /// use vason::{Canvas, Color};
    /// let mut buffer = [0u32; 4];
    /// let mut canvas = Canvas::new(&mut buffer, 2, 2);
    ///
    /// canvas.pixel_iter_mut()
    ///     .filter_map(|(x,y,p)| (x != y).then(|| p))
    ///     .for_each(|p| *p = Color::RED.into());
    ///
    /// assert_eq!(0, buffer[0]); // 0, 0
    /// assert_eq!(u32::from(Color::RED), buffer[1]); // 1, 0
    /// assert_eq!(u32::from(Color::RED), buffer[2]); // 0, 1
    /// assert_eq!(0, buffer[3]); // 1, 1
    /// ```
    pub fn pixel_iter_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut u32)> + '_ {
        self.buffer
            .iter_mut()
            .enumerate()
            .map(|(i, p)| (i % self.width, i / self.width, p))
    }

    /// Fills a rectangle shaped region in this [`Canvas`]. If width or height is <= 0 nothing is drawn.
    /// ``` rust
    /// use vason::{Canvas, Color};
    /// let mut buffer = [0u32; 256];
    /// let mut canvas = Canvas::new(&mut buffer, 16, 16);
    /// canvas.fill_rect(3, 3, 7, 7, Color::RED);
    /// ```
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

    /// Renders the outline of a rectangle shaped region in this [`Canvas`]. If width or height is <= 0 nothing is drawn.
    /// ``` rust
    /// use vason::{Canvas, Color};
    /// let mut buffer = [0u32; 256];
    /// let mut canvas = Canvas::new(&mut buffer, 16, 16);
    /// canvas.outline_rect(3, 3, 7, 7, Color::RED);
    /// ```
    #[allow(clippy::cast_sign_loss)]
    pub fn outline_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: impl Into<Color>) {
        // consistency with fill_rect
        if w <= 0 || h <= 0 {
            return;
        }

        let raw_color = u32::from(color.into());

        let x1 = x;
        let x2 = x + w - 1;
        let y1 = y;
        let y2 = y + h - 1;

        if x2 >= 0 && y1 < self.clamped_height {
            let from_x = x1.clamp(0, self.clamped_width - 1) as usize;
            // draw the last pixel
            let to_x = (x2 + 1).min(self.clamped_width) as usize;

            if 0 <= y1 {
                let offset = y1 as usize * self.width;
                self.buffer[offset + from_x..offset + to_x].fill(raw_color);
            }

            if 0 <= y2 && y2 < self.clamped_height {
                let offset = y2 as usize * self.width;
                self.buffer[offset + from_x..offset + to_x].fill(raw_color);
            }
        }

        if y2 >= 0 && x1 < self.clamped_width {
            let from_y = y1.clamp(0, self.clamped_height - 1);
            let to_y = y2.min(self.clamped_height);

            if 0 <= x1 {
                for j in from_y..to_y {
                    unsafe {
                        self.set_pixel_unchecked_raw_i32(x1, j, raw_color);
                    }
                }
            }

            if 0 <= x2 && x2 < self.clamped_width {
                for j in from_y..to_y {
                    unsafe {
                        self.set_pixel_unchecked_raw_i32(x2, j, raw_color);
                    }
                }
            }
        }
    }

    /// Renders the outline of a rectangle shaped region with a given thickness in this [`Canvas`]. If the width, height or thickness is <= 0 nothing is drawn.
    /// ``` rust
    /// use vason::{Canvas, Color};
    /// let mut buffer = [0u32; 256];
    /// let mut canvas = Canvas::new(&mut buffer, 16,16);
    /// canvas.thick_outline_rect(3, 3, 7, 7, 2, Color::RED);
    /// ```
    #[allow(clippy::cast_sign_loss)]
    pub fn thick_outline_rect(
        &mut self,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        thickness: i32,
        color: impl Into<Color>,
    ) {
        // consistency with fill_rect
        if w <= 0 || h <= 0 || thickness <= 0 {
            return;
        } else if thickness == 1 {
            self.outline_rect(x, y, w, h, color);
            return;
        }

        let raw_color = u32::from(color.into());

        let x1 = x;
        let x2 = x + w;
        let y1 = y;
        let y2 = y + h;

        let half_thickness = thickness / 2;

        if x2 + half_thickness >= 0 && y1 - half_thickness < self.clamped_height {
            let from_x = (x1 - half_thickness).clamp(0, self.clamped_width - 1) as usize;
            let to_x = (x2 + half_thickness).min(self.clamped_width) as usize;

            if 0 <= y1 + half_thickness {
                for j in
                    (y1 - half_thickness).max(0)..(y1 + half_thickness).min(self.clamped_height)
                {
                    let offset = j as usize * self.width;
                    self.buffer[offset + from_x..offset + to_x].fill(raw_color);
                }
            }

            if 0 <= y2 + half_thickness && y2 - half_thickness < self.clamped_height {
                for j in
                    (y2 - half_thickness).max(0)..(y2 + half_thickness).min(self.clamped_height)
                {
                    let offset = j as usize * self.width;
                    self.buffer[offset + from_x..offset + to_x].fill(raw_color);
                }
            }
        }

        if y2 + half_thickness >= 0 && x1 - half_thickness < self.clamped_width {
            let from_y = y1.clamp(0, self.clamped_height - 1);
            let to_y = y2.min(self.clamped_height);

            if 0 <= x1 + half_thickness {
                for j in from_y..to_y {
                    for i in
                        (x1 - half_thickness).max(0)..(x1 + half_thickness).min(self.clamped_width)
                    {
                        unsafe {
                            self.set_pixel_unchecked_raw_i32(i, j, raw_color);
                        }
                    }
                }
            }

            if 0 <= x2 + half_thickness && x2 - half_thickness < self.clamped_width {
                for j in from_y..to_y {
                    for i in
                        (x2 - half_thickness).max(0)..(x2 + half_thickness).min(self.clamped_width)
                    {
                        unsafe {
                            self.set_pixel_unchecked_raw_i32(i, j, raw_color);
                        }
                    }
                }
            }
        }
    }

    /// Fills a circle shaped region in this [`Canvas`]. The radius must be positive.
    /// ``` rust
    /// use vason::{Canvas, Color};
    /// let mut buffer = [0u32; 256];
    /// let mut canvas = Canvas::new(&mut buffer, 16, 16);
    /// canvas.fill_circle(8, 8, 4, Color::GREEN);
    /// ```
    #[allow(clippy::cast_sign_loss, clippy::many_single_char_names)]
    pub fn fill_circle(&mut self, x: i32, y: i32, mut r: i32, color: impl Into<Color>) {
        if r < 1 {
            return;
        }

        let raw_color = u32::from(color.into());

        let mut i = -r;
        let mut j = 0;
        let mut err = 2 - 2 * r;
        loop {
            let y1 = y - j;
            let y2 = y + j;
            //i is negative
            let from_x = (x + i).clamp(0, self.clamped_width - 1);
            let to_x = (x - i).clamp(from_x, self.clamped_width);

            if 0 <= y1 && y1 < self.clamped_height {
                let offset = y1 as usize * self.width;
                let range = offset + from_x as usize..offset + to_x as usize;
                self.buffer[range].fill(raw_color);
            }

            if 0 <= y2 && y2 < self.clamped_height {
                let offset = y2 as usize * self.width;
                let range = offset + from_x as usize..offset + to_x as usize;
                self.buffer[range].fill(raw_color);
            }
            r = err;
            if r <= j {
                j += 1;
                err += j * 2 + 1;
            }
            if r > i || err > j {
                i += 1;
                err += i * 2 + 1;
            }

            if i >= 0 {
                break;
            }
        }
    }

    /// Renders the outline of a circle shaped region in this [`Canvas`]. The radius must be positive,
    /// ``` rust
    /// use vason::{Canvas, Color};
    /// let mut buffer = [0u32; 256];
    /// let mut canvas = Canvas::new(&mut buffer, 16,16);
    /// canvas.outline_circle(8, 8, 8, Color::YELLOW);
    /// ```
    #[allow(clippy::cast_sign_loss, clippy::many_single_char_names)]
    pub fn outline_circle(&mut self, x: i32, y: i32, mut r: i32, color: impl Into<Color>) {
        if r < 1 {
            return;
        }

        let raw_color = u32::from(color.into());
        let mut i = -r;
        let mut j = 0;
        let mut err = 2 - 2 * r;
        loop {
            let x1 = x - i;
            let x2 = x + i;
            let y1 = y - j;
            let y2 = y + j;

            // TODO: benchmark this with precise tooling against just using self.set_pixel()
            // flamegraph shows a siginificant difference, but I'm not convinced.
            if 0 <= x1 && x1 < self.clamped_width {
                if 0 <= y1 && y1 < self.clamped_height {
                    unsafe {
                        self.set_pixel_unchecked_raw_i32(x1, y1, raw_color);
                    }
                }
                if 0 <= y2 && y2 < self.clamped_height {
                    unsafe {
                        self.set_pixel_unchecked_raw_i32(x1, y2, raw_color);
                    }
                }
            }
            if 0 <= x2 && x2 < self.clamped_width {
                if 0 <= y1 && y1 < self.clamped_height {
                    unsafe {
                        self.set_pixel_unchecked_raw_i32(x2, y1, raw_color);
                    }
                }
                if 0 <= y2 && y2 < self.clamped_height {
                    unsafe {
                        self.set_pixel_unchecked_raw_i32(x2, y2, raw_color);
                    }
                }
            }

            r = err;
            if r <= j {
                j += 1;
                err += j * 2 + 1;
            }
            if r > i || err > j {
                i += 1;
                err += i * 2 + 1;
            }

            if i >= 0 {
                break;
            }
        }
    }

    /// Renders the outline of a circle shaped region with a given thickness in this [`Canvas`]. The radius must be positive.
    /// The stroke witdth grows symmetrically (inwards and outwards), that is the supplied radius will be the center of the stroke.
    /// ``` rust
    /// use vason::{Canvas, Color};
    /// let mut buffer = [0u32; 256];
    /// let mut canvas = Canvas::new(&mut buffer, 16, 16);
    /// canvas.thick_outline_circle(4, 8, 8, 2, Color::CYAN);
    /// ```
    #[allow(clippy::similar_names)]
    pub fn thick_outline_circle(
        &mut self,
        x: i32,
        y: i32,
        r: i32,
        thickness: i32,
        color: impl Into<Color>,
    ) {
        if thickness == 1 {
            self.outline_circle(x, y, r, color);
            return;
        } else if thickness <= 0 || r < 1 {
            return;
        }

        let raw_color = u32::from(color.into());

        let half_thickness = thickness / 2;

        let ro = r + half_thickness;
        let ri = ro - thickness + 1;

        let mut xo = ro;
        let mut xi = ri;
        let mut j = 0;
        let mut erro = 1 - xo;
        let mut erri = 1 - xi;

        while xo >= j {
            // TODO: inline these calls manually to do fewer checks.
            self.hline(y + j, x + xi, x + xo, raw_color);
            self.vline(x + j, y + xi, y + xo, raw_color);
            self.hline(y + j, x - xo, x - xi, raw_color);
            self.vline(x - j, y + xi, y + xo, raw_color);
            self.hline(y - j, x - xo, x - xi, raw_color);
            self.vline(x - j, y - xo, y - xi, raw_color);
            self.hline(y - j, x + xi, x + xo, raw_color);
            self.vline(x + j, y - xo, y - xi, raw_color);

            j += 1;

            if erro < 0 {
                erro += 2 * j + 1;
            } else {
                xo -= 1;
                erro += 2 * (j - xo) + 1;
            }

            if j > ri {
                xi = j;
            } else if erri < 0 {
                erri += 2 * j + 1;
            } else {
                xi -= 1;
                erri += 2 * (j - xi) + 1;
            }
        }
    }

    /// Fills an ellipse shaped region in this [`Canvas`]. The radii must be positive.
    /// ``` rust
    /// use vason::{Canvas, Color};
    /// let mut buffer = [0u32; 256];
    /// let mut canvas = Canvas::new(&mut buffer, 16, 16);
    /// canvas.fill_ellipse(8, 8, 8, 4, Color::RED);
    /// ```
    #[allow(clippy::many_single_char_names, clippy::cast_sign_loss)]
    pub fn fill_ellipse(&mut self, x: i32, y: i32, a: i32, b: i32, color: impl Into<Color>) {
        if a < 1 || b < 1 {
            return;
        }

        let raw_color = u32::from(color.into());

        let mut i = -a;
        let mut j = 0;

        // change to larger integers to avoid overflow.
        let b2 = i64::from(b) * i64::from(b);
        let a2 = i64::from(a) * i64::from(a);
        let mut err = i64::from(i) * (2 * b2 + i64::from(i)) + b2;

        loop {
            let y1 = y - j;
            let y2 = y + j;
            //i is non-positive
            let from_x = (x + i).clamp(0, self.clamped_width - 1);
            let to_x = (x - i).clamp(from_x, self.clamped_width);

            if 0 <= y1 && y1 < self.clamped_height {
                let offset = y1 as usize * self.width;
                let range = offset + from_x as usize..offset + to_x as usize;
                self.buffer[range].fill(raw_color);
            }

            if 0 <= y2 && y2 < self.clamped_height {
                let offset = y2 as usize * self.width;
                let range = offset + from_x as usize..offset + to_x as usize;
                self.buffer[range].fill(raw_color);
            }

            let e2 = 2 * err;
            if e2 >= i64::from(i * 2 + 1) * b2 {
                i += 1;
                err += i64::from(i * 2 + 1) * b2;
            }

            if e2 <= i64::from(j * 2 + 1) * a2 {
                j += 1;
                err += i64::from(j * 2 + 1) * a2;
            }

            if i > 0 {
                break;
            }
        }

        while j < b {
            j += 1;
            if 0 <= x && x < self.clamped_width {
                let y1 = y + j;
                let y2 = y - j;
                if 0 <= y1 && y1 < self.clamped_height {
                    unsafe {
                        self.set_pixel_unchecked_raw_i32(x, y1, raw_color);
                    }
                }
                if 0 <= y2 && y2 < self.clamped_height {
                    unsafe {
                        self.set_pixel_unchecked_raw_i32(x, y2, raw_color);
                    }
                }
            }
        }
    }

    /// Renders the outline of an ellipse shaped region in this [`Canvas`]. The radii must be positive.
    /// ``` rust
    /// use vason::{Canvas, Color};
    /// let mut buffer = [0u32; 256];
    /// let mut canvas = Canvas::new(&mut buffer, 16, 16);
    /// canvas.outline_ellipse(8, 8, 8, 4, Color::RED);
    /// ```
    #[allow(clippy::many_single_char_names)]
    pub fn outline_ellipse(&mut self, x: i32, y: i32, a: i32, b: i32, color: impl Into<Color>) {
        if a < 1 || b < 1 {
            return;
        }

        let raw_color = u32::from(color.into());

        let mut i = -a;
        let mut j = 0;

        // change to larger integers to avoid overflow.
        let b2 = i64::from(b) * i64::from(b);
        let a2 = i64::from(a) * i64::from(a);
        let mut err = i64::from(i) * (2 * b2 + i64::from(i)) + b2;

        loop {
            let x1 = x - i;
            let x2 = x + i;
            let y1 = y - j;
            let y2 = y + j;

            // TODO: benchmark this with precise tooling against just using self.set_pixel()
            // flamegraph shows a siginificant difference, but I'm not convinced.
            if 0 <= x1 && x1 < self.clamped_width {
                if 0 <= y1 && y1 < self.clamped_height {
                    unsafe {
                        self.set_pixel_unchecked_raw_i32(x1, y1, raw_color);
                    }
                }
                if 0 <= y2 && y2 < self.clamped_height {
                    unsafe {
                        self.set_pixel_unchecked_raw_i32(x1, y2, raw_color);
                    }
                }
            }
            if 0 <= x2 && x2 < self.clamped_width {
                if 0 <= y1 && y1 < self.clamped_height {
                    unsafe {
                        self.set_pixel_unchecked_raw_i32(x2, y1, raw_color);
                    }
                }
                if 0 <= y2 && y2 < self.clamped_height {
                    unsafe {
                        self.set_pixel_unchecked_raw_i32(x2, y2, raw_color);
                    }
                }
            }

            let e2 = 2 * err;
            if e2 >= i64::from(i * 2 + 1) * b2 {
                i += 1;
                err += i64::from(i * 2 + 1) * b2;
            }

            if e2 <= i64::from(j * 2 + 1) * a2 {
                j += 1;
                err += i64::from(j * 2 + 1) * a2;
            }

            if i > 0 {
                break;
            }
        }

        while j < b {
            j += 1;
            if 0 <= x && x < self.clamped_width {
                let y1 = y + j;
                let y2 = y - j;
                if 0 <= y1 && y1 < self.clamped_height {
                    unsafe {
                        self.set_pixel_unchecked_raw_i32(x, y1, raw_color);
                    }
                }
                if 0 <= y2 && y2 < self.clamped_height {
                    unsafe {
                        self.set_pixel_unchecked_raw_i32(x, y2, raw_color);
                    }
                }
            }
        }
    }

    #[allow(clippy::too_many_lines, clippy::similar_names)]
    pub fn thick_outline_ellipse(
        &mut self,
        x: i32,
        y: i32,
        a: i32,
        b: i32,
        thickness: i32,
        color: impl Into<Color>,
    ) {
        // source: https://stackoverflow.com/questions/55980376/midpoint-thick-ellipse-drawing-algorithm

        // TODO: inline hline calls
        // TODO: explore using larger integers to awoid overflow (like in outline_ellipse)
        // TODO: organize the code better, replace macros with functions perhaps
        // TODO: apply fix that makes horizontal and vertical thickness more uniform (after inlining hline calls)

        if thickness == 1 {
            self.outline_ellipse(x, y, a, b, color);
            return;
        } else if a < 1 || b < 1 || thickness <= 0 {
            return;
        }

        let raw_color = u32::from(color.into());

        let half_thickness = thickness / 2;
        let (inner_a, inner_b) = (a - half_thickness, b - half_thickness);
        let (outer_a, outer_b) = (a + half_thickness, b + half_thickness);

        let (mut px, mut py) = (outer_a, 0);

        let (mut outer_dx, mut outer_dy) = (2 * outer_b * outer_b * px, 2 * outer_a * outer_a * py);

        macro_rules! outer_err_yx {
            () => {
                outer_a * outer_a - outer_b * outer_b * outer_a + (outer_b * outer_b) / 4
            };
        }

        macro_rules! outer_err_xy {
            () => {
                outer_a * outer_a * (py * py + py) + outer_b * outer_b * (px - 1) * (px - 1)
                    - outer_b * outer_b * outer_a * outer_a
            };
        }

        let mut inner_x = inner_a;
        let (mut inner_dx, mut inner_dy) =
            (2 * inner_b * inner_b * inner_x, 2 * inner_a * inner_a * py);

        macro_rules! inner_err_yx {
            () => {
                inner_a * inner_a - inner_b * inner_b * inner_a + (inner_b * inner_b) / 4
            };
        }

        macro_rules! inner_err_xy {
            () => {
                inner_a * inner_a * (py * py + py)
                    + inner_b * inner_b * (inner_x - 1) * (inner_x - 1)
                    - inner_b * inner_b * inner_a * inner_a
            };
        }

        let mut outer_err = outer_err_yx!();
        let mut inner_err = inner_err_yx!();

        macro_rules! outer_step_yx {
            () => {
                py += 1;

                if outer_err < 0 {
                    outer_dy += 2 * outer_a * outer_a;
                    outer_err += outer_dy + outer_a * outer_a;
                } else {
                    px -= 1;
                    outer_dy += 2 * outer_a * outer_a;
                    outer_dx -= 2 * outer_b * outer_b;
                    outer_err += outer_dy - outer_dx + outer_a * outer_a;
                }
            };
        }

        macro_rules! inner_step_yx {
            () => {
                if inner_err < 0 {
                    inner_dy += 2 * inner_a * inner_a;
                    inner_err += inner_dy + inner_a * inner_a;
                } else {
                    inner_x -= 1;
                    inner_dy += 2 * inner_a * inner_a;
                    inner_dx -= 2 * inner_b * inner_b;
                    inner_err += inner_dy - inner_dx + inner_a * inner_a;
                }
            };
        }

        macro_rules! outer_step_xy {
            () => {
                loop {
                    px -= 1;
                    if px < 0 {
                        break;
                    }

                    if outer_err > 0 {
                        outer_dx -= 2 * outer_b * outer_b;
                        outer_err += outer_b * outer_b - outer_dx;
                    } else {
                        py += 1;
                        outer_dy += 2 * outer_a * outer_a;
                        outer_dx -= 2 * outer_b * outer_b;
                        outer_err += outer_dy - outer_dx + outer_b * outer_b;
                        break;
                    }
                }
            };
        }

        macro_rules! inner_step_xy {
            () => {
                loop {
                    inner_x -= 1;
                    if inner_x < 0 {
                        break;
                    }

                    if inner_err > 0 {
                        inner_dx -= 2 * inner_b * inner_b;
                        inner_err += inner_b * inner_b - inner_dx;
                    } else {
                        inner_dy += 2 * inner_a * inner_a;
                        inner_dx -= 2 * inner_b * inner_b;
                        inner_err += inner_dy - inner_dx + inner_b * inner_b;
                        break;
                    }
                }
            };
        }

        // 1st phase

        while outer_dy < outer_dx && inner_dy < inner_dx {
            // TODO: manually inline hline calls
            self.hline(y + py, x - px, x - inner_x, raw_color);
            self.hline(y + py, x + px, x + inner_x, raw_color);
            self.hline(y - py, x - px, x - inner_x, raw_color);
            self.hline(y - py, x + px, x + inner_x, raw_color);

            outer_step_yx!();
            inner_step_yx!();
        }

        // 2nd phase

        if outer_dy < outer_dx {
            inner_err = inner_err_xy!();

            while outer_dy < outer_dx && inner_x >= 0 {
                self.hline(y + py, x - px, x - inner_x, raw_color);
                self.hline(y + py, x + px, x + inner_x, raw_color);
                self.hline(y - py, x - px, x - inner_x, raw_color);
                self.hline(y - py, x + px, x + inner_x, raw_color);

                outer_step_yx!();
                inner_step_xy!();
            }

            while outer_dy < outer_dx {
                self.hline(y + py, x - px, x + px, raw_color);
                self.hline(y - py, x - px, x + px, raw_color);
                outer_step_yx!();
            }
        } else {
            outer_err = outer_err_xy!();

            while inner_dy < inner_dx {
                let (px_, py_) = (px, py);

                outer_step_xy!();
                inner_step_yx!();

                let inner_x_ = x.min(inner_x);
                self.hline(y + py_, x - px_, x - inner_x_, raw_color);
                self.hline(y + py_, x + px_, x + inner_x_, raw_color);
                self.hline(y - py_, x - px_, x - inner_x_, raw_color);
                self.hline(y - py_, x + px_, x + inner_x_, raw_color);
            }
        }

        // 3rd phase

        outer_err = outer_err_xy!();
        inner_err = inner_err_xy!();

        while inner_x >= 0 {
            let (px_, py_) = (px, py);
            outer_step_xy!();
            let inner_x_ = x.min(inner_x);

            self.hline(y + py_, x - px_, x - inner_x_, raw_color);
            self.hline(y + py_, x + px_, x + inner_x_, raw_color);
            self.hline(y - py_, x - px_, x - inner_x_, raw_color);
            self.hline(y - py_, x + px_, x + inner_x_, raw_color);

            inner_step_xy!();
        }

        // 4th phase

        while px >= 0 {
            self.hline(y + py, x - px, x + px + 1, raw_color);
            self.hline(y - py, x - px, x + px + 1, raw_color);
            outer_step_xy!();
        }
    }

    /// Renders a triangle in this [`Canvas`].
    /// ``` rust
    /// use vason::{Canvas, Color};
    /// let mut buffer = [0u32; 256];
    /// let mut canvas = Canvas::new(&mut buffer, 16, 16);
    /// canvas.fill_triangle(1, 0, 12, 0, 13, 15, Color::RED);
    /// ```
    #[allow(
        clippy::too_many_arguments,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation
    )]
    pub fn fill_triangle(
        &mut self,
        mut x1: i32,
        mut y1: i32,
        mut x2: i32,
        mut y2: i32,
        mut x3: i32,
        mut y3: i32,
        color: impl Into<Color>,
    ) {
        use std::mem::swap;
        let raw_color = u32::from(color.into());

        // Sort points vertically
        if y2 > y3 {
            swap(&mut x2, &mut x3);
            swap(&mut y2, &mut y3);
        }

        if y1 > y2 {
            swap(&mut x1, &mut x2);
            swap(&mut y1, &mut y2);
        }

        if y2 > y3 {
            swap(&mut x2, &mut x3);
            swap(&mut y2, &mut y3);
        }

        let dx_far = f64::from(x3 - x1) / f64::from(y3 - y1 + 1);
        let dx_upper = f64::from(x2 - x1) / f64::from(y2 - y1 + 1);
        let dx_low = f64::from(x3 - x2) / f64::from(y3 - y2 + 1);
        let mut xf = f64::from(x1);
        let mut xt = xf + dx_upper;

        for y in y1..=y3.min(self.clamped_height - 1) {
            if y >= 0 {
                let offset = y as usize * self.width;
                {
                    let from_x = xf.max(0.0) as usize;
                    let to_x = if xt < f64::from(self.clamped_width) {
                        xt as usize
                    } else {
                        (self.clamped_width - 1) as usize
                    };

                    let range = offset + from_x..=offset + to_x;

                    if !range.is_empty() {
                        self.buffer[range].fill(raw_color);
                    }
                }

                {
                    let from_x = xt.max(0.0) as usize;
                    let to_x = if xf < f64::from(self.clamped_width) {
                        xf as usize
                    } else {
                        self.clamped_width as usize - 1
                    };

                    let range = offset + from_x..=offset + to_x;
                    if !range.is_empty() {
                        self.buffer[range].fill(raw_color);
                    }
                }
            }

            xf += dx_far;
            if y < y2 {
                xt += dx_upper;
            } else {
                xt += dx_low;
            }
        }
    }

    /// Renders the outline of a triangle in this [`Canvas`].
    /// ``` rust
    /// use vason::{Canvas, Color};
    /// let mut buffer = [0u32; 256];
    /// let mut canvas = Canvas::new(&mut buffer, 16, 16);
    /// canvas.outline_triangle(1, 0, 12, 0, 13, 15, Color::RED);
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn outline_triangle(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        color: impl Into<Color>,
    ) {
        let raw_color = u32::from(color.into());

        self.line(x1, y1, x2, y2, raw_color);
        self.line(x1, y1, x3, y3, raw_color);
        self.line(x2, y2, x3, y3, raw_color);
    }

    /// Renders the outline of a triangle with thickness in this [`Canvas`]. Joints are covered by rounded ends (circles).
    /// ``` rust
    /// use vason::{Canvas, Color};
    /// let mut buffer = [0u32; 256];
    /// let mut canvas = Canvas::new(&mut buffer, 16, 16);
    /// canvas.thick_outline_triangle(1, 0, 12, 0, 13, 15, 3, Color::RED);
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn thick_outline_triangle(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        x3: i32,
        y3: i32,
        thickness: i32,
        color: impl Into<Color>,
    ) {
        if thickness < 0 {
            return;
        } else if thickness == 1 {
            self.outline_triangle(x1, y1, x2, y2, x3, y3, color);
            return;
        }

        let raw_color = u32::from(color.into());

        let half_thickness = thickness / 2;

        self.thick_line(x1, y1, x2, y2, thickness, raw_color);
        self.thick_line(x1, y1, x3, y3, thickness, raw_color);
        self.thick_line(x2, y2, x3, y3, thickness, raw_color);
        self.fill_circle(x1, y1, half_thickness, raw_color);
        self.fill_circle(x2, y2, half_thickness, raw_color);
        self.fill_circle(x3, y3, half_thickness, raw_color);
    }

    /// Renders a horizontal line. Should be preferred when explicitly drawing horizontal lines.
    /// ``` rust
    /// use vason::{Canvas, Color};
    /// let mut buffer = [0u32;256];
    /// let mut canvas = Canvas::new(&mut buffer, 16, 16);
    /// canvas.hline(10, 0, 16, Color::RED);
    /// ```
    #[allow(clippy::cast_sign_loss)]
    #[inline]
    pub fn hline(&mut self, y: i32, x1: i32, x2: i32, color: impl Into<Color>) {
        let raw_color = u32::from(color.into());

        if 0 <= y && y < self.clamped_height {
            let (x1, x2) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
            let from_x = x1.clamp(0, self.clamped_width - 1);
            let to_x = (x2 + 1).clamp(from_x, self.clamped_width);
            let offset = y as usize * self.width;
            let range = offset + from_x as usize..offset + to_x as usize;
            self.buffer[range].fill(raw_color);
        }
    }

    /// Renders a vertical line. Should be preferred when explicitly drawing vertical lines.
    /// ``` rust
    /// use vason::{Canvas, Color};
    /// let mut buffer = [0u32; 256];
    /// let mut canvas = Canvas::new(&mut buffer, 16, 16);
    /// canvas.vline(10, 0, 16, Color::RED);
    /// ```
    #[allow(clippy::cast_sign_loss)]
    #[inline]
    pub fn vline(&mut self, x: i32, y1: i32, y2: i32, color: impl Into<Color>) {
        let raw_color = u32::from(color.into());

        if 0 <= x && x < self.clamped_width {
            let (y1, y2) = if y1 > y2 { (y2, y1) } else { (y1, y2) };

            let from_y = y1.clamp(0, self.clamped_height - 1);
            let to_y = (y2 + 1).clamp(from_y, self.clamped_height);

            for y in from_y..to_y {
                unsafe { self.set_pixel_unchecked_raw_i32(x, y, raw_color) }
            }
        }
    }

    /// Renders a horizontal line with thickness. Should be preferred when explicitly drawing thick horizontal lines.
    /// ``` rust
    /// use vason::{Canvas, Color};
    /// let mut buffer = [0u32; 256];
    /// let mut canvas = Canvas::new(&mut buffer, 16, 16);
    /// canvas.thick_hline(10, 0, 16, 2, Color::RED);
    /// ```
    #[inline]
    pub fn thick_hline(
        &mut self,
        y: i32,
        x1: i32,
        x2: i32,
        thickness: i32,
        color: impl Into<Color>,
    ) {
        let thickness = thickness.max(0);
        let (x1, x2) = if x1 > x2 { (x2, x1) } else { (x1, x2) };
        self.fill_rect(x1, y + thickness / 2, x2 - x1, thickness, color);
    }

    /// Renders a vertical line with thickness. Should be preferred when explicitly drawing thick vertical lines.
    /// ``` rust
    /// use vason::{Canvas, Color};
    /// let mut buffer = [0u32; 256];
    /// let mut canvas = Canvas::new(&mut buffer, 16, 16);
    /// canvas.thick_vline(10, 0, 16, 2, Color::RED);
    #[inline]
    pub fn thick_vline(
        &mut self,
        x: i32,
        y1: i32,
        y2: i32,
        thickness: i32,
        color: impl Into<Color>,
    ) {
        let thickness = thickness.max(0);
        let (y1, y2) = if y1 > y2 { (y2, y1) } else { (y1, y2) };
        self.fill_rect(x - thickness / 2, y1, thickness, y2 - y1, color);
    }

    /// Renders a line. Should be preferred when mostly drawing non axis-aligned lines.
    /// If there is a substantial chance of drawing axis-aligned (hline or vline) consider using [`line_maybe_axis_aligned`](struct.Canvas.html#method.line_maybe_axis_aligned) instead
    /// ``` rust
    /// use vason::{Canvas, Color};
    /// let mut buffer = [0u32; 256];
    /// let mut canvas = Canvas::new(&mut buffer, 16, 16);
    /// canvas.line(10, 2, 10, 12, Color::RED);
    /// ```
    pub fn line(&mut self, mut x1: i32, mut y1: i32, x2: i32, y2: i32, color: impl Into<Color>) {
        let raw_color = u32::from(color.into());

        let dx = (x2 - x1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };

        let dy = -(y2 - y1).abs();
        let sy = if y1 < y2 { 1 } else { -1 };

        let mut err = dx + dy;

        loop {
            if 0 <= x1 && x1 < self.clamped_width && 0 <= y1 && y1 < self.clamped_height {
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

    /// Renders a line. Should be preferred when mostly drawing axis-aligned lines.
    /// If it is not very likely you'll draw a lot of axis-aligned lines prefer [`line`](struct.Canvas.html#method.line) instead.
    /// ``` rust
    /// use vason::{Canvas, Color};
    /// let mut buffer = [0u32; 256];
    /// let mut canvas = Canvas::new(&mut buffer, 16,16);
    /// // axis aligned
    /// canvas.line_maybe_axis_aligned(10, 2, 10, 12, Color::RED);
    /// // not axis aligned
    /// canvas.line_maybe_axis_aligned(12, 4, 5, 7, Color::BLUE);
    /// ```
    #[inline]
    pub fn line_maybe_axis_aligned(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: impl Into<Color>,
    ) {
        if x1 == x2 {
            self.vline(x1, y1, y2, color);
        } else if y1 == y2 {
            self.hline(y1, x1, x2, color);
        } else {
            self.line(x1, y1, x2, y2, color);
        }
    }

    /// Renders a line with thickness. Should be preferred when mostly drawing non axis-aligned lines.
    /// If there is a substantial chance of drawing axis-aligned (hline or vline) consider using [`thick_line_maybe_axis_aligned`](struct.Canvas.html#method.thick_line_maybe_axis_aligned) instead
    /// ``` rust
    /// use vason::{Canvas, Color};
    /// let mut buffer = [0u32; 256];
    /// let mut canvas = Canvas::new(&mut buffer, 16, 16);
    /// canvas.thick_line(10, 2, 10, 12, 4, Color::RED);
    /// ```
    #[allow(clippy::similar_names, clippy::cast_possible_truncation)]
    pub fn thick_line(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        thickness: i32,
        color: impl Into<Color>,
    ) {
        if thickness < 0 {
            return;
        } else if thickness == 1 {
            self.line(x1, y1, x2, y2, color);
            return;
        }

        let raw_color = u32::from(color.into());

        let dx = f64::from(x2 - x1);
        let dy = f64::from(y2 - y1);
        let length = (dx * dx + dy * dy).sqrt();

        let half_thickness = f64::from(thickness) * 0.5;

        let px = ((-dy / length) * half_thickness) as i32;
        let py = ((dx / length) * half_thickness) as i32;

        let v1x = x1 + px;
        let v1y = y1 + py;

        let v2x = x1 - px;
        let v2y = y1 - py;

        let v3x = x2 + px;
        let v3y = y2 + py;

        let v4x = x2 - px;
        let v4y = y2 - py;

        self.fill_triangle(v1x, v1y, v2x, v2y, v3x, v3y, raw_color);
        self.fill_triangle(v2x, v2y, v4x, v4y, v3x, v3y, raw_color);
    }

    /// Renders a line with thickness. Should be preferred when mostly drawing axis-aligned lines.
    /// If it is not very likely you'll draw a lot of axis-aligned lines prefer [`thick_line`](struct.Canvas.html#method.thick_line) instead.
    /// ``` rust
    /// use vason::{Canvas, Color};
    /// let mut buffer = [0u32; 256];
    /// let mut canvas = Canvas::new(&mut buffer, 16,16);
    /// // axis aligned
    /// canvas.thick_line_maybe_axis_aligned(10, 2, 10, 12, 3, Color::RED);
    /// // not axis aligned
    /// canvas.thick_line_maybe_axis_aligned(12, 4, 5, 7, 4, Color::BLUE);
    /// ```
    #[inline]
    pub fn thick_line_maybe_axis_aligned(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        thickness: i32,
        color: impl Into<Color>,
    ) {
        if x1 == x2 {
            self.thick_vline(x1, y1, y2, thickness, color);
        } else if y1 == y2 {
            self.thick_hline(y1, x1, x2, thickness, color);
        } else {
            self.thick_line(x1, y1, x2, y2, thickness, color);
        }
    }

    /// Starts a flood fill from supplied coordinate filling the area with the color provided.
    #[allow(clippy::cast_sign_loss)]
    pub fn flood_fill(&mut self, x: i32, y: i32, color: impl Into<Color>) {
        if 0 <= x && x < self.clamped_width && 0 <= y && y < self.clamped_height {
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

    #[allow(clippy::similar_names)]
    #[inline]
    fn clamp_rect_i32(&self, xmin: i32, xmax: i32, ymin: i32, ymax: i32) -> (i32, i32, i32, i32) {
        let from_x = xmin.clamp(0, self.clamped_width - 1);
        let to_x = xmax.clamp(from_x, self.clamped_width);

        let from_y = ymin.clamp(0, self.clamped_height - 1);
        let to_y = ymax.clamp(from_y, self.clamped_height);

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
