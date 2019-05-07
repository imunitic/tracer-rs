#[derive(Copy, Clone)]
pub struct Color {
	red: f64,
	green: f64,
	blue: f64
}

impl Color {
	const EPSILON: f64 = 0.00001_f64;
	
	pub fn new(red: f64, green: f64, blue: f64) -> Color {
		Color{red, green, blue}
	}
}

#[cfg(test)]
mod tests {
	use crate::color::Color;
	use float_cmp::ApproxEq;
    

	#[test]
	fn test_color() {
		let c = Color::new(-0.5, 0.4, 1.7);
		assert!(c.red.approx_eq(&-0.5, Color::EPSILON, 0));
		assert!(c.green.approx_eq(&0.4, Color::EPSILON, 0));
		assert!(c.blue.approx_eq(&1.7, Color::EPSILON, 0));
	}
}