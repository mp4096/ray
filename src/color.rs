#[derive(PartialEq, Eq, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn from_rgb_double(red_double: f64, green_double: f64, blue_double: f64) -> Color {
        Color {
            red: color_double_to_byte(red_double),
            green: color_double_to_byte(green_double),
            blue: color_double_to_byte(blue_double),
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "#{:X?}{:X?}{:X?}", self.red, self.green, self.blue)
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
