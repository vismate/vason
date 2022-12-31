/// A tuple struct that represents a color.
/// This struct has a single public field, which stores
/// the color as a u32.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(pub u32);

impl Color {
    /// Creates a [`Color`] from r, g and b values.
    ///
    /// # Examples
    ///
    /// ```
    /// use vason::Color;
    ///
    /// let color = Color::rgb(0, 255, 255);
    /// assert_eq!(color, Color::CYAN);
    /// ```
    #[must_use]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self(u32::from_le_bytes([b, g, r, 0]))
    }

    /// Returns a tuple of (r,g,b) values.
    ///
    /// # Examples
    ///
    /// ```
    /// use vason::Color;
    ///
    /// let color = Color::YELLOW;
    /// assert_eq!(color.to_rgb(), (255, 255, 0));
    /// ```
    #[must_use]
    pub const fn to_rgb(self) -> (u8, u8, u8) {
        let [b, g, r, _] = u32::to_le_bytes(self.0);
        (r, g, b)
    }

    #[must_use]
    pub const fn gray(c: u8) -> Self {
        Self::rgb(c, c, c)
    }

    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const GRAY: Self = Self::rgb(128, 128, 128);
    pub const WHITE: Self = Self::rgb(255, 255, 255);
    pub const LIGHT_GRAY: Self = Self::rgb(192, 192, 192);
    pub const RED: Self = Self::rgb(255, 0, 0);
    pub const DARK_RED: Self = Self::rgb(128, 0, 0);
    pub const GREEN: Self = Self::rgb(0, 255, 0);
    pub const DARK_GREEN: Self = Self::rgb(0, 128, 0);
    pub const BLUE: Self = Self::rgb(0, 0, 255);
    pub const DARK_BLUE: Self = Self::rgb(0, 0, 128);
    pub const CYAN: Self = Self::rgb(0, 255, 255);
    pub const TEAL: Self = Self::rgb(0, 128, 128);
    pub const MAGENTA: Self = Self::rgb(255, 0, 255);
    pub const PURPLE: Self = Self::rgb(128, 0, 128);
    pub const YELLOW: Self = Self::rgb(255, 255, 0);
    pub const OLIVE: Self = Self::rgb(128, 128, 0);
    pub const BROWN: Self = Self::rgb(165, 42, 42);
    pub const GOLD: Self = Self::rgb(255, 215, 0);
    pub const INDIGO: Self = Self::rgb(75, 0, 130);
    pub const SKY_BLUE: Self = Self::rgb(135, 205, 250);
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::rgb(r, g, b)
    }
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        Self::gray(value)
    }
}

impl From<Color> for u32 {
    fn from(value: Color) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversions() {
        assert_eq!(u32::from(Color::rgb(12, 1, 231)), 786_919);
        assert_eq!(Color::from(786_919u32).to_rgb(), (12, 1, 231));
    }
}
