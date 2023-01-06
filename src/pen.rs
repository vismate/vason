//! The Pen-API allows you to play with ["turtle geometry"](https://people.eecs.berkeley.edu/~bh/v1ch10/turtle.html)
//! # Example
//! ```rust
//! use vason::{Canvas, Color};
//! let mut buffer = vec![0u32; 128*128];
//! let mut canvas = Canvas::new(&mut buffer, 128, 128);
//! let mut pen = canvas.pen();
//!
//! // draw a hexagon
//! pen.set_position(45.0, 32.0).set_thickness(2);
//! pen.repeat(6, |pen| {
//!   pen.forward(45.0).turn_right(60.0);
//! });
//! ```

use crate::{Canvas, Color};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy)]
pub struct PenState {
    pub position: (f32, f32),
    pub direction: f32,
    pub color: Color,
    pub thickness: i32,
    pub is_down: bool,
    pub bounds: Option<(f32, f32, f32, f32)>,
}

impl Default for PenState {
    fn default() -> Self {
        Self {
            position: (0.0, 0.0),
            direction: 0.0,
            color: Color::WHITE,
            thickness: 1,
            is_down: true,
            bounds: None,
        }
    }
}

pub struct Pen<'a, 'b> {
    canvas: &'a mut Canvas<'b>,
    state: PenState,
}

impl<'a, 'b> Pen<'a, 'b> {
    /// Creates a new [`Pen`].
    pub fn new(canvas: &'a mut Canvas<'b>) -> Self {
        Self::with_state(canvas, PenState::default())
    }

    /// Creates a new [`Pen`] from the supplied state.
    pub fn with_state(canvas: &'a mut Canvas<'b>, state: PenState) -> Self {
        let mut s = Self { canvas, state };
        s.bound_self();
        s
    }

    /// Sets the state of this [`Pen`].
    pub fn set_state(&mut self, state: PenState) -> &mut Self {
        self.state = state;
        self.bound_self();
        self
    }

    /// Returns the get state of this [`Pen`].
    #[must_use]
    pub fn get_state(&self) -> PenState {
        self.state
    }

    /// Sets the bounds of this [`Pen`]. The pen won't be able to move outside these boundaries.
    #[allow(clippy::similar_names)]
    pub fn set_bounds(&mut self, xmin: f32, xmax: f32, ymin: f32, ymax: f32) -> &mut Self {
        self.state.bounds = Some((xmin, xmax, ymin, ymax));
        self.bound_self();
        self
    }

    /// Sets the bounds of this [`Pen`] to the border of the canvas.
    #[allow(clippy::cast_precision_loss)]
    pub fn set_bounds_to_canvas(&mut self) -> &mut Self {
        self.set_bounds(
            0.0,
            (self.canvas.width() - 1) as f32,
            0.0,
            (self.canvas.height() - 1) as f32,
        )
    }

    /// Returns the get bounds of this [`Pen`].
    #[must_use]
    pub fn get_bounds(&self) -> Option<(f32, f32, f32, f32)> {
        self.state.bounds
    }

    /// Returns a reference to the canvas of this [`Pen`].
    #[must_use]
    pub fn canvas(&self) -> &Canvas<'b> {
        self.canvas
    }

    /// Returns a mutable reference to the canvas of this [`Pen`].
    #[must_use]
    pub fn canvas_mut(&mut self) -> &mut Canvas<'b> {
        self.canvas
    }

    /// Set the pen state to defaults [`Pen`].
    pub fn reset(&mut self) -> &mut Self {
        self.state = PenState::default();
        self.bound_self();
        self
    }

    /// Set the pen position without drawing.
    /// In case you wish to draw a line when moving to new position use [`set_position_draw`](struct.Pen.html#method.set_position_draw)
    pub fn set_position(&mut self, x: f32, y: f32) -> &mut Self {
        self.state.position = self.bound_pos(x, y);
        self
    }

    /// Set the pen position and draw a line from old position to the new one.
    /// Warning: it will only draw if the pen is down.
    pub fn set_position_draw(&mut self, x: f32, y: f32) -> &mut Self {
        let (x, y) = self.bound_pos(x, y);

        #[allow(clippy::cast_possible_truncation)]
        if self.state.is_down {
            let x1 = self.state.position.0 as i32;
            let y1 = self.state.position.1 as i32;

            self.stroke(x1, y1, x as i32, y as i32);
        }

        self.state.position = (x, y);

        self
    }

    /// Returns the get position of this [`Pen`].
    #[must_use]
    pub fn get_position(&self) -> (f32, f32) {
        self.state.position
    }

    /// Move the pen forwards. Draws a line on it's way if the pen is down.
    pub fn forward(&mut self, amount: f32) -> &mut Self {
        let (dy, dx) = self.state.direction.sin_cos();
        let new_pos = self.bound_pos(
            self.state.position.0 + dx * amount,
            self.state.position.1 + dy * amount,
        );

        #[allow(clippy::cast_possible_truncation)]
        if self.state.is_down {
            let x1 = self.state.position.0 as i32;
            let y1 = self.state.position.1 as i32;
            let x2 = new_pos.0 as i32;
            let y2 = new_pos.1 as i32;

            self.stroke(x1, y1, x2, y2);
        }

        self.state.position = new_pos;

        self
    }

    /// Move the pen backwards. Draws a line on it's way if the pen is down.
    #[inline]
    pub fn backward(&mut self, amount: f32) -> &mut Self {
        self.forward(-amount)
    }

    /// Initiate a flood fiil at current position.
    #[allow(clippy::cast_possible_truncation)]
    pub fn flood_fill(&mut self) -> &mut Self {
        self.canvas.flood_fill(
            self.state.position.0 as i32,
            self.state.position.1 as i32,
            self.state.color,
        );
        self
    }

    /// Sets the direction of this [`Pen`].
    pub fn set_direction(&mut self, deg: f32) -> &mut Self {
        self.state.direction = deg.to_radians();
        self
    }

    /// Returns the get direction of this [`Pen`].
    #[must_use]
    pub fn get_direction(&self) -> f32 {
        self.state.direction.to_degrees()
    }

    /// Sets the direction in radians of this [`Pen`].
    pub fn set_direction_rad(&mut self, rad: f32) -> &mut Self {
        self.state.direction = rad;
        self
    }

    /// Returns the direction of this [`Pen`] in radians.
    #[must_use]
    pub fn get_direction_rad(&self) -> f32 {
        self.state.direction
    }

    /// Turn the pen left by the given degree.
    pub fn turn_left(&mut self, deg: f32) -> &mut Self {
        self.state.direction -= deg.to_radians();
        self
    }

    /// Turn the pen left by the given degree in radians.
    pub fn turn_left_rad(&mut self, rad: f32) -> &mut Self {
        self.state.direction -= rad;
        self
    }

    /// Turn the pen right by the given degree.
    pub fn turn_right(&mut self, deg: f32) -> &mut Self {
        self.state.direction += deg.to_radians();
        self
    }

    /// Turn the pen right by the given degree in radians.
    pub fn turn_right_rad(&mut self, rad: f32) -> &mut Self {
        self.state.direction += rad;
        self
    }

    /// Sets the color of this [`Pen`].
    pub fn set_color(&mut self, color: impl Into<Color>) -> &mut Self {
        self.state.color = color.into();
        self
    }

    /// Returns the get color of this [`Pen`].
    #[must_use]
    pub fn get_color(&self) -> Color {
        self.state.color
    }

    /// Sets the thickness of this [`Pen`].
    pub fn set_thickness(&mut self, thickness: i32) -> &mut Self {
        self.state.thickness = thickness;
        self
    }

    /// Returns the get thickness of this [`Pen`].
    #[must_use]
    pub fn get_thickness(&self) -> i32 {
        self.state.thickness
    }

    /// Pick the pen up. When pen is up there is no line drawn when moving the it.
    pub fn pen_up(&mut self) -> &mut Self {
        self.state.is_down = false;
        self
    }

    /// Put the pen down. When pen is down a line is drawn when moving the it.
    pub fn pen_down(&mut self) -> &mut Self {
        self.state.is_down = true;
        self
    }

    /// Toggle whether the pen is up or down.
    pub fn pen_toggle(&mut self) -> &mut Self {
        self.state.is_down = !self.state.is_down;
        self
    }

    /// Repeat an action multiple times.
    pub fn repeat(&mut self, times: usize, mut f: impl FnMut(&mut Self)) -> &mut Self {
        for _ in 0..times {
            f(self);
        }

        self
    }

    #[allow(clippy::similar_names)]
    fn bound_pos(&self, x: f32, y: f32) -> (f32, f32) {
        match self.state.bounds {
            Some((xmin, xmax, ymin, ymax)) => (x.clamp(xmin, xmax), y.clamp(ymin, ymax)),
            None => (x, y),
        }
    }

    fn bound_self(&mut self) {
        let (x, y) = self.state.position;
        self.state.position = self.bound_pos(x, y);
    }

    fn stroke(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        // thickness <= 1 checked by canvas.thick_line
        self.canvas
            .thick_line(x1, y1, x2, y2, self.state.thickness, self.state.color);

        if self.state.thickness > 1 {
            let half_thickness = self.state.thickness / 2;

            // TODO: optimize with kind of a dirty flag?
            self.canvas
                .fill_circle(x1, y1, half_thickness, self.state.color);
            self.canvas
                .fill_circle(x2, y2, half_thickness, self.state.color);
        }
    }
}
