mod tuple;
mod color;
mod canvas;

use canvas::Canvas;
use color::Color;

use tuple::Tuple;

#[derive(Copy, Clone)]
struct Projectile {
	position: Tuple,
	velocity: Tuple
}

#[derive(Copy, Clone)]
struct Environment {
	gravity: Tuple,
	wind: Tuple
}

fn tick(env: Environment, proj: Projectile) -> Projectile {
	let position = proj.position + proj.velocity;
	let velocity = proj.velocity + env.gravity + env.wind;

	Projectile { position, velocity}
}


fn main() {
	let mut p = Projectile { position: Tuple::point(0.0, 1.0, 0.0), 
		velocity: Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25};
	let e = Environment {
		gravity: Tuple::vector(0.0, -0.1, 0.0),
		wind: Tuple::vector(-0.01, 0.0, 0.0)
	};

	let color = Color::new(0.5, 0.88, 1.0);

	let mut canvas = Canvas::new(900, 550);

	let mut pos = p.position.y;

	while pos > 0.0 {
		p = tick(e, p);
		let x = p.position.x as i64;
		let y = canvas.height as i64 - p.position.y as i64;

		if x < 0 || y < 0 {
			pos = p.position.y;
			continue;
		}

		if x > 900 || y > 550 {
			pos = p.position.y;
			continue;
		}
		canvas.write_pixel(x as usize, y as usize, color);
		
		pos = p.position.y;
	}
	println!("{}", canvas.to_ppm());
}
