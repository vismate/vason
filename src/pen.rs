use crate::{Canvas, Color};

//TODO: line thickness

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
    pub fn new(canvas: &'a mut Canvas<'b>) -> Self {
        Self::with_state(canvas, PenState::default())
    }

    pub fn with_state(canvas: &'a mut Canvas<'b>, state: PenState) -> Self {
        let mut s = Self { canvas, state };
        s.bound_self();
        s
    }

    pub fn set_state(&mut self, state: PenState) -> &mut Self {
        self.state = state;
        self.bound_self();
        self
    }

    #[must_use]
    pub fn get_state(&self) -> PenState {
        self.state
    }

    #[allow(clippy::similar_names)]
    pub fn set_bounds(&mut self, xmin: f32, xmax: f32, ymin: f32, ymax: f32) -> &mut Self {
        self.state.bounds = Some((xmin, xmax, ymin, ymax));
        self.bound_self();
        self
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn set_bounds_to_canvas(&mut self) -> &mut Self {
        self.set_bounds(
            0.0,
            (self.canvas.width() - 1) as f32,
            0.0,
            (self.canvas.height() - 1) as f32,
        )
    }

    #[must_use]
    pub fn get_bounds(&self) -> Option<(f32, f32, f32, f32)> {
        self.state.bounds
    }

    #[must_use]
    pub fn canvas(&self) -> &Canvas<'b> {
        self.canvas
    }

    #[must_use]
    pub fn canvas_mut(&mut self) -> &mut Canvas<'b> {
        self.canvas
    }

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

        // TODO: use line_maybe_axis_aligned once available after a merge
        #[allow(clippy::cast_possible_truncation)]
        if self.state.is_down {
            self.canvas.line(
                self.state.position.0 as i32,
                self.state.position.1 as i32,
                x as i32,
                y as i32,
                self.state.color,
            );
        }

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

        #[allow(clippy::cast_possible_truncation)]
        if self.state.is_down {
            let x1 = self.state.position.0 as i32;
            let y1 = self.state.position.1 as i32;
            let x2 = new_pos.0 as i32;
            let y2 = new_pos.1 as i32;

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

        self.state.position = new_pos;

        self
    }

    pub fn backward(&mut self, amount: f32) -> &mut Self {
        self.forward(-amount)
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn flood_fill(&mut self) -> &mut Self {
        self.canvas.flood_fill(
            self.state.position.0 as i32,
            self.state.position.1 as i32,
            self.state.color,
        );
        self
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

    pub fn set_thickness(&mut self, thickness: i32) -> &mut Self {
        self.state.thickness = thickness;
        self
    }

    #[must_use]
    pub fn get_thickness(&self) -> i32 {
        self.state.thickness
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
}
