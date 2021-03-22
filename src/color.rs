use std::ops::{Add, Sub, Mul};
use float_cmp::{ApproxEq, F64Margin, approx_eq};
use std::cmp::PartialEq;

#[derive(Copy, Clone, Debug)]
pub struct Color {
	pub red: f64,
	pub green: f64,
	pub blue: f64
}

impl Color {
	const EPSILON: f64 = 0.00001_f64;

	pub fn new(red: f64, green: f64, blue: f64) -> Color {
		Color{red, green, blue}
	}

	pub fn eq(self, rhs: Color) -> bool {
        approx_eq!(Self, self, rhs, epsilon=Color::EPSILON)
	}

	pub fn default() -> Color {
		Color::new(0.0, 0.0, 0.0)
	}
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.red.eq(&other.red) &&
        self.green.eq(&other.green) &&
        self.blue.eq(&other.blue)
    }
}

impl ApproxEq for Color {
    type Margin = F64Margin;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        (self.red == other.red &&
        self.green == other.green &&
        self.blue == other.blue) ||
        (self.red.approx_eq(other.red, margin) &&
        self.green.approx_eq(other.green, margin) &&
        self.blue.approx_eq(other.blue, margin))
    }
}

impl Add for Color {
	 type Output = Color;
	 fn add(self, rhs: Color) -> Color {
	 	Color::new(self.red + rhs.red,
	 		  	self.green + rhs.green,
	 		  	self.blue + rhs.blue)
	 }
}

impl Sub for Color {
	type Output = Color;

	fn sub(self, rhs: Color) -> Color {
		Color::new(self.red - rhs.red, 
			self.green - rhs.green, 
			self.blue - rhs.blue)
	}
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self: Color, rhs: f64) -> Color {
        Color::new(self.red * rhs,
            self.green * rhs,
            self.blue * rhs)
    }
}

impl Mul<Color> for Color {
	type Output = Color;

	fn mul(self, rhs: Color) -> Color {
		Color::new(self.red * rhs.red,
			self.green * rhs.green,
			self.blue * rhs.blue)
	}
}

#[cfg(test)]
mod tests {
	use crate::color::Color;
    use float_cmp::approx_eq;
    

	#[test]
	fn test_color() {
		let c = Color::new(-0.5, 0.4, 1.7);
		assert!(approx_eq!(f64, c.red, -0.5, epsilon=Color::EPSILON));
		assert!(approx_eq!(f64, c.green, 0.4, epsilon=Color::EPSILON));
		assert!(approx_eq!(f64, c.blue, 1.7, epsilon=Color::EPSILON));
	}

	#[test]
	fn test_color_addition() {
		let c1 = Color::new(0.9, 0.6, 0.75);
		let c2 = Color::new(0.7, 0.1, 0.25);
		let result = c1 + c2;
		assert!(result.eq(Color::new(1.6, 0.7, 1.0)));
	}

	#[test]
	fn test_color_subtraction() {
		let c1 = Color::new(0.9, 0.6, 0.75);
		let c2 = Color::new(0.7, 0.1, 0.25);
		let result = c1 - c2;
		assert!(result.eq(Color::new(0.2, 0.5, 0.5)));
	}

	#[test]
	fn test_color_multiply_by_scalar() {
		let c1 = Color::new(0.2, 0.3, 0.4);
		let result = c1 * 2.0;

		assert!(result.eq(Color::new(0.4, 0.6, 0.8)));
	}

	#[test]
	fn test_color_blend() {
		let c1 = Color::new(1.0, 0.2, 0.4);
		let c2 = Color::new(0.9, 1.0, 0.1);
		let result = c1 * c2;

		assert!(result.eq(Color::new(0.9, 0.2, 0.04)));
	}
}
