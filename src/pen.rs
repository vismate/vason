use crate::{Canvas, Color};

// TODO: Flood fill, pen width

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy)]
pub enum PenBound {
    Unbounded,
    Clamp,
    Wrap,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy)]
pub struct PenState {
    pub position: (f32, f32),
    pub direction: f32,
    pub color: Color,
    pub is_down: bool,
    pub bound: PenBound,
}

impl Default for PenState {
    fn default() -> Self {
        Self {
            position: (0.0, 0.0),
            direction: 0.0,
            color: Color::WHITE,
            is_down: true,
            bound: PenBound::Unbounded,
        }
    }
}

pub struct Pen<'a> {
    canvas: &'a mut Canvas,
    state: PenState,
}

impl<'a> Pen<'a> {
    pub fn new(canvas: &'a mut Canvas) -> Self {
        Self::with_state(canvas, PenState::default())
    }

    pub fn with_state(canvas: &'a mut Canvas, state: PenState) -> Self {
        Self { canvas, state }
    }

    pub fn set_state(&mut self, state: PenState) -> &mut Self {
        self.state = state;
        self
    }

    pub fn set_bound(&mut self, bound: PenBound) -> &mut Self {
        self.state.bound = bound;
        let (x, y) = self.state.position;
        self.state.position = self.bound_pos(x, y);
        self
    }

    #[must_use]
    pub fn get_bound(&self) -> PenBound {
        self.state.bound
    }

    #[must_use]
    pub fn get_state(&self) -> PenState {
        self.state
    }

    #[must_use]
    pub fn canvas(&self) -> &Canvas {
        self.canvas
    }

    #[must_use]
    pub fn canvas_mut(&mut self) -> &mut Canvas {
        self.canvas
    }

    pub fn reset(&mut self) -> &mut Self {
        self.state = PenState::default();
        self
    }

    pub fn set_position(&mut self, x: f32, y: f32) -> &mut Self {
        self.state.position = (x, y);
        self
    }

    #[must_use]
    pub fn get_position(&self) -> (f32, f32) {
        self.state.position
    }

    pub fn forward(&mut self, amount: f32) -> &mut Self {
        let (dy, dx) = self.state.direction.sin_cos();
        let new_pos = self.bound_pos(
            self.state.position.0 + dx * amount,
            self.state.position.1 + dy * amount,
        );

        // We can allow truncation, this is a safe behaviour
        // Edge case: whenever we have a position outside the range of an i32,
        // the value is clamped to said range.
        // This can result in a vastly different line (of a different slope)
        // TODO: mitigate the edge case above
        // TODO: use line_maybe_axis_aligned once available after a merge
        #[allow(clippy::cast_possible_truncation)]
        if self.state.is_down {
            self.canvas.line(
                self.state.position.0 as i32,
                self.state.position.1 as i32,
                new_pos.0 as i32,
                new_pos.1 as i32,
                self.state.color,
            );
        }

        self.state.position = new_pos;

        self
    }

    pub fn backward(&mut self, amount: f32) -> &mut Self {
        self.forward(-amount)
    }

    pub fn set_direction(&mut self, deg: f32) -> &mut Self {
        self.state.direction = deg.to_radians();
        self
    }

    #[must_use]
    pub fn get_direction(&self) -> f32 {
        self.state.direction.to_degrees()
    }

    pub fn set_direction_rad(&mut self, rad: f32) -> &mut Self {
        self.state.direction = rad;
        self
    }

    #[must_use]
    pub fn get_direction_rad(&self) -> f32 {
        self.state.direction
    }

    pub fn turn_left(&mut self, deg: f32) -> &mut Self {
        self.state.direction -= deg.to_radians();
        self
    }

    pub fn turn_left_rad(&mut self, rad: f32) -> &mut Self {
        self.state.direction -= rad;
        self
    }

    pub fn turn_right(&mut self, deg: f32) -> &mut Self {
        self.state.direction += deg.to_radians();
        self
    }

    pub fn turn_right_rad(&mut self, rad: f32) -> &mut Self {
        self.state.direction += rad;
        self
    }

    pub fn set_color(&mut self, color: impl Into<Color>) -> &mut Self {
        self.state.color = color.into();
        self
    }

    #[must_use]
    pub fn get_color(&self) -> Color {
        self.state.color
    }

    pub fn pen_up(&mut self) -> &mut Self {
        self.state.is_down = false;
        self
    }

    pub fn pen_down(&mut self) -> &mut Self {
        self.state.is_down = true;
        self
    }

    pub fn pen_toggle(&mut self) -> &mut Self {
        self.state.is_down = !self.state.is_down;
        self
    }

    pub fn repeat(&mut self, times: usize, mut f: impl FnMut(&mut Self)) -> &mut Self {
        for _ in 0..times {
            f(self);
        }

        self
    }

    #[allow(clippy::cast_precision_loss)]
    fn bound_pos(&self, x: f32, y: f32) -> (f32, f32) {
        match self.state.bound {
            PenBound::Clamp => (
                x.clamp(0.0, self.canvas.width() as f32 - 1.0),
                y.clamp(0.0, self.canvas.height() as f32 - 1.0),
            ),
            PenBound::Wrap => {
                let nx = x % self.canvas.width() as f32;
                let ny = y % self.canvas.height() as f32;

                let nx = if nx.is_sign_negative() {
                    self.canvas.width() as f32 + nx
                } else {
                    nx
                };
                let ny = if ny.is_sign_negative() {
                    self.canvas.height() as f32 + ny
                } else {
                    ny
                };

                (nx, ny)
            }
            PenBound::Unbounded => (x, y),
        }
    }
}
