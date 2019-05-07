mod tuple;
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
		velocity: Tuple::vector(1.0, 1.0, 0.0).normalize()};
	let e = Environment {
		gravity: Tuple::vector(0.0, -0.1, 0.0),
		wind: Tuple::vector(-0.01, 0.0, 0.0)
	};

	let mut pos = p.position.y;
	let mut counter = 1;

	while pos > 0.0 {
		p = tick(e, p);
		
		println!("{:?}", pos);
		
		pos = p.position.y;
		counter = counter +  1;
	}
	println!("Run tick {:?} times", counter);
}
