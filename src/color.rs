pub type Color = crate::Vec3;

impl Color {
    pub fn new_white() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }
    pub fn new_black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
    pub fn new_red() -> Color {
        Color::new(1.0, 0.0, 0.0)
    }
    pub fn new_green() -> Color {
        Color::new(0.0, 1.0, 0.0)
    }
    pub fn new_blue() -> Color {
        Color::new(0.0, 0.0, 1.0)
    }

    pub fn red(&self) -> f64 {
        self.x
    }

    pub fn green(&self) -> f64 {
        self.y
    }

    pub fn blue(&self) -> f64 {
        self.z
    }

    pub fn as_bytes(&self) -> [u8; 3] {
        [
            color_double_to_byte(self.x),
            color_double_to_byte(self.y),
            color_double_to_byte(self.z),
        ]
    }
}

#[inline]
fn color_double_to_byte(color_as_double: f64) -> u8 {
    (255.999 * color_as_double) as u8
}

#[cfg(test)]
mod tests {
    use super::color_double_to_byte;

    #[test]
    fn min_correct() {
        assert_eq!(color_double_to_byte(0.0_f64), 0_u8);
    }

    #[test]
    fn middle_correct() {
        assert_eq!(color_double_to_byte(0.5_f64), 127_u8);
    }

    #[test]
    fn max_correct() {
        assert_eq!(color_double_to_byte(1.0_f64), 255_u8);
    }
}
