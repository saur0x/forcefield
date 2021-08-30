use crate::Vector;


#[allow(dead_code)]
pub struct Body {
    pub position: Vector,
    velocity: Vector,
    acceleration: Vector,

    mass: f64,
    charge: i32,

    moving: bool,
}


impl Body {
    pub fn new(position: Vector, velocity: Vector, acceleration: Vector, mass: f64, moving: bool) -> Self {
        Self {
            position, velocity, acceleration, mass, charge: 0, moving
        }
    }

    pub fn exert_force(&mut self, force: Vector) {
        self.acceleration += force / self.mass;
    }

    pub fn update_position(&mut self) {
        if !self.moving {
            return;
        }

        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration = Vector::default();
    }

    pub fn get_force(&self, other: &Self) -> Vector {
        const G: f64 = 1.5;
        const E: f64 = 1E-8;
        
        let displacement = other.position - self.position;
        let distance = displacement.magnitude();
        let mut force = G * self.mass * other.mass / (distance.powi(2) + E);
        force /= distance;
        force * displacement
    }
}


pub fn update_bodies(bodies: &mut Vec<Body>) {
    for i in 0..bodies.len() {
        for j in 0..bodies.len() {
            if i == j {
                continue;
            }
            let (alpha, beta) = if i > j {
                let (x, y) = bodies.split_at_mut(i);
                (&mut y[0], &mut x[j])
            } else {
                let (x, y) = bodies.split_at_mut(j);
                (&mut x[i], &mut y[0])
            };

            let force = beta.get_force(alpha);
            beta.exert_force(force);
        }
    }

    for body in bodies.iter_mut() {
        body.update_position();
    }
}