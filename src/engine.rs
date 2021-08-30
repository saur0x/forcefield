use crate::{Body, Vector};
use std::io::Write;
use std::ops;


pub struct RenderEngine {
	width: usize,
	height: usize,
}


impl RenderEngine {
	pub fn new(width: usize, height: usize) -> Self {
		Self { width, height }
	}

	fn force_to_color(force: Vector) -> (u8, u8, u8) {
		// let magnitude = force.magnitude().tanh();
		let magnitude = 1.0;
		let force = force.normalize();
	
		let r = range_map(force.0, -1.0, 1.0, 0.0, 255.0) as u8;
		let g = range_map(force.1, -1.0, 1.0, 0.0, 255.0) as u8;
		let b = 128 + range_map(magnitude, 0.0, 1.0, 0.0, 32.0) as u8;
	
		(r, g, b)
	}
}


impl RenderEngine {
	pub fn render(&self, bodies: &Vec<Body>) {
		let mut stdout = std::io::stdout();
		let mut buffer: Vec<u8> = vec![0; 3 * self.width * self.height];
	
		let mut unit = Body::new(
			Vector::default(), Vector::default(), 
			Vector::default(), 1.0, false
		);

		print!("P6\n{} {}\n255\n", self.width, self.height);

		for y in 0..self.height {
			for x in 0..self.width {
				let index = (y * self.width + x) * 3;
				unit.position.0 = x as f64;
				unit.position.1 = y as f64;
	
				let force = bodies
					.iter()
					.map(|body| unit.get_force(&body))
					.reduce(|a, b| a + b)
					.unwrap();
	
				let (r, g, b) = Self::force_to_color(force);
	
				buffer[index] = r;
				buffer[index + 1] = g;
				buffer[index + 2] = b;
			}
		}

		stdout.write_all(&buffer).expect("Your pipe is broken!");
		stdout.flush().expect("Your pipe is broken!");
	}
}


fn range_map<T>(x: T, x0: T, x1: T, y0: T, y1: T) -> T
    where T: ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T> + ops::Div<Output = T> + Copy
{
    y0 + (x - x0) * (y1 - y0) / (x1 - x0)
}