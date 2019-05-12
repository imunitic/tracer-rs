use crate::color::Color;
use nalgebra::{DMatrix};

#[derive(Debug)]
pub struct Canvas {
	pub width: usize,
	pub height: usize,
	data: DMatrix<Color>
}

impl Canvas {
	pub fn new(width: usize, height: usize) -> Canvas {
		let m = DMatrix::from_fn(width, height, |_i, _j| Color::new(0.0, 0.0, 0.0));
		Canvas{width, height, data: m}
	}

	pub fn write_pixel(&mut self, x: usize, y: usize, color: Color ) {
		self.data[(x, y)] = color;
	}

	pub fn pixel_at(&self, x: usize, y: usize) -> Color {
		self.data[(x, y)]
	}

	pub fn to_ppm(&self) -> String {
		let mut v: Vec<String> = Vec::new();
		let header = format!("P3\n{} {}\n255", self.width, self.height);
		let footer = "\n\n";

		v.push(header);
		for x in 0 .. self.height {
			let mut row: Vec<String> = Vec::new();
			let mut rw = String::new();
			for y in 0 .. self.width {
				let c = self.pixel_at(y, x);
				let vc = vec![c.red, c.green, c.blue];
				for i in vc {
					rw.push_str(&to_rgb(i));
					if rw.len() < 65 {
						rw.push_str(" ");
					} else {
						rw.push_str("\n");
						row.push(rw);
						rw = "".to_string();
					}
				}
			}
			row.push(rw.trim().to_string());
			v.push(row.join(" "));
		}
		let mut data = v.join("\n");
		data.push_str(footer);
		data
	}
}

fn to_rgb(c: f64) -> String {
	let clip = |r| {
		if r > 255 { 255 }
		else if r < 0 { 0 }
		else {r}
	};
	format!("{}", clip((c * 255.0) as i64))
}

#[cfg(test)]
mod tests {
	use crate::canvas::Canvas;
	use crate::color::Color;

	#[test]
	fn test_canvas_creation() {
		let c = Canvas::new(10, 20);
		assert_eq!(c.width, 10 );
		assert_eq!(c.height, 20 );
	}

	#[test]
	fn test_canvas_write_pixel() {
		let mut c = Canvas::new(10, 20);
		let red = Color::new(1.0, 0.0, 0.0);
		c.write_pixel(2, 3, red);
		assert_eq!(c.pixel_at(2, 3),  red);
	}

	#[test]
	fn test_canvas_to_ppm_header() {
		let c = Canvas::new(5, 3);
		let ppm = c.to_ppm();
		let l13: Vec<&str> = ppm.split("\n").collect();
		assert_eq!(l13[0..3].join("\n"), "P3\n5 3\n255");
	}

	#[test]
	fn test_canvas_to_ppm_data() {
		let mut c = Canvas::new(5, 3);
		let c1 = Color::new(1.5, 0.0, 0.0);
		let c2 = Color::new(0.0, 0.5, 0.0);
		let c3 = Color::new(-0.5, 0.0, 1.0);

		c.write_pixel(0, 0, c1);
		c.write_pixel(2, 1, c2);
		c.write_pixel(4, 2, c3);
		let ppm = c.to_ppm();
		let l46: Vec<&str> = ppm.split("\n").collect();
		assert_eq!(l46[3..6].join("\n"),"255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 127 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255" );

	}

	#[test]
	fn test_canvas_to_ppm_long_line() {
		let mut c = Canvas::new(10, 2);
		for x in 0 .. c.height {
			for y in 0 .. c.width {
				c.write_pixel(y, x, Color::new(1.0, 0.8, 0.6));
			}
		}

		let ppm = c.to_ppm();
		assert_eq!(ppm, "P3\n10 2\n255\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n 153 255 204 153 255 204 153 255 204 153 255 204 153\n255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n 153 255 204 153 255 204 153 255 204 153 255 204 153\n\n" );
	}
}