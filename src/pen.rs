use crate::{Canvas, Color};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy)]
pub struct PenState {
    pub position: (f32, f32),
    pub direction: f32,
    pub color: Color,
    pub is_down: bool,
}

impl Default for PenState {
    fn default() -> Self {
        Self {
            position: (0.0, 0.0),
            direction: 0.0,
            color: Color::WHITE,
            is_down: true,
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

    pub fn set_state(&mut self, state: PenState) {
        self.state = state;
    }

    #[must_use]
    pub fn get_state(&self) -> PenState {
        self.state
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
        let new_pos = (
            self.state.position.0 + dx * amount,
            self.state.position.1 + dy * amount,
        );

        // We can allow truncation, this is a safe behaviour
        // Edge case: whenever we have a position outside the range of an i32,
        // the value is clamped to said range.
        // This can result in a vastly different line (of a different slope)
        // TODO: mitigate the edge case above
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
}
