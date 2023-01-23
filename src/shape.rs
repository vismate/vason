use crate::{Canvas, Color};

pub trait Draw {
    fn draw_to(&self, canvas: &mut Canvas);
}

macro_rules! impl_shape {
    () => {
        #[inline]
        pub fn set_fill_color(&mut self, color: Option<impl Into<Color>>) -> &mut Self {
            self.fill_color = color.map(|c| u32::from(c.into()));
            self
        }

        #[inline]
        pub fn set_outline_color(&mut self, color: Option<impl Into<Color>>) -> &mut Self {
            self.outline_color = color.map(|c| u32::from(c.into()));
            self
        }

        #[inline]
        pub fn set_outline_thickness(&mut self, thickness: i32) -> &mut Self {
            self.outline_thickness = thickness;
            self
        }
    };
}

#[derive(Debug)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub fill_color: Option<u32>,
    pub outline_thickness: i32,
    pub outline_color: Option<u32>,
}

impl Rectangle {
    #[must_use]
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            fill_color: Some(0),
            outline_thickness: 1,
            outline_color: None,
        }
    }

    #[must_use]
    pub fn from_points(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        let (x, width) = if x1 > x2 {
            (x2, x1 - x2)
        } else {
            (x1, x2 - x1)
        };
        let (y, height) = if y1 > y2 {
            (y2, y1 - y2)
        } else {
            (y1, y2 - y1)
        };

        Self::new(x, y, width, height)
    }

    impl_shape!();
}

#[derive(Debug)]
pub struct Circle {
    pub x: i32,
    pub y: i32,
    pub radius: i32,
    pub outline_thickness: i32,
    pub fill_color: Option<u32>,
    pub outline_color: Option<u32>,
}

impl Circle {
    #[must_use]
    pub fn new(x: i32, y: i32, radius: i32) -> Self {
        Self {
            x,
            y,
            radius,
            outline_thickness: 1,
            fill_color: Some(0),
            outline_color: None,
        }
    }

    impl_shape!();
}

#[derive(Debug)]
pub struct Line {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub fill_color: Option<u32>,
    pub thickness: i32,
}

impl Line {
    #[must_use]
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            fill_color: Some(0),
            thickness: 1,
        }
    }

    #[inline]
    pub fn set_fill_color(&mut self, color: Option<impl Into<Color>>) -> &mut Self {
        self.fill_color = color.map(|c| u32::from(c.into()));
        self
    }

    #[inline]
    pub fn set_thickness(&mut self, thickness: i32) -> &mut Self {
        self.thickness = thickness;
        self
    }
}

#[derive(Debug)]
pub struct Ellipse {
    pub x: i32,
    pub y: i32,
    pub a: i32,
    pub b: i32,
    pub fill_color: Option<u32>,
    pub outline_color: Option<u32>,
    pub outline_thickness: i32,
}

impl Ellipse {
    #[must_use]
    pub fn new(x: i32, y: i32, a: i32, b: i32) -> Self {
        Self {
            x,
            y,
            a,
            b,
            fill_color: Some(0),
            outline_color: None,
            outline_thickness: 1,
        }
    }

    impl_shape!();
}

#[derive(Debug)]
pub struct Triangle {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub x3: i32,
    pub y3: i32,
    pub fill_color: Option<u32>,
    pub outline_color: Option<u32>,
    pub outline_thickness: i32,
}

impl Triangle {
    #[must_use]
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            x3,
            y3,
            fill_color: Some(0),
            outline_color: None,
            outline_thickness: 1,
        }
    }

    impl_shape!();
}

impl Draw for Rectangle {
    #[inline]
    fn draw_to(&self, canvas: &mut Canvas) {
        if let Some(fill_color) = self.fill_color {
            canvas.fill_rect(self.x, self.y, self.width, self.height, fill_color);
        }

        if let Some(outline_color) = self.outline_color {
            canvas.thick_outline_rect(
                self.x,
                self.y,
                self.width,
                self.height,
                self.outline_thickness,
                outline_color,
            );
        }
    }
}

impl Draw for Circle {
    #[inline]
    fn draw_to(&self, canvas: &mut Canvas) {
        if let Some(fill_color) = self.fill_color {
            canvas.fill_circle(self.x, self.y, self.radius, fill_color);
        }

        if let Some(outline_color) = self.outline_color {
            canvas.thick_outline_circle(
                self.x,
                self.y,
                self.radius,
                self.outline_thickness,
                outline_color,
            );
        }
    }
}

impl Draw for Line {
    #[inline]
    fn draw_to(&self, canvas: &mut Canvas) {
        if let Some(fill_color) = self.fill_color {
            canvas.thick_line(
                self.x1,
                self.y1,
                self.x2,
                self.y2,
                self.thickness,
                fill_color,
            );
        }
    }
}

impl Draw for Ellipse {
    #[inline]
    fn draw_to(&self, canvas: &mut Canvas) {
        if let Some(fill_color) = self.fill_color {
            canvas.fill_ellipse(self.x, self.y, self.a, self.b, fill_color);
        }

        if let Some(outline_color) = self.outline_color {
            // TODO: once thick_outline_ellipse is implemented modify this
            canvas.outline_ellipse(self.x, self.y, self.a, self.b, outline_color);
        }
    }
}

impl Draw for Triangle {
    #[inline]
    fn draw_to(&self, canvas: &mut Canvas) {
        if let Some(fill_color) = self.fill_color {
            canvas.fill_triangle(
                self.x1, self.y1, self.x2, self.y2, self.x3, self.y3, fill_color,
            );
        }

        if let Some(outline_color) = self.outline_color {
            canvas.thick_outline_triangle(
                self.x1,
                self.y1,
                self.x2,
                self.y2,
                self.x3,
                self.y3,
                self.outline_thickness,
                outline_color,
            );
        }
    }
}
