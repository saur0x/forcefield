mod body;
mod engine;
mod vector;

use body::Body;
use engine::RenderEngine;
use vector::Vector;


fn main() {
    const WIDTH: usize = 500;
    const HEIGHT: usize = 500;
    const FRAMES: usize = 1000;    

    let engine = RenderEngine::new(WIDTH, HEIGHT);

    let mut bodies = vec![
        Body::new(Vector::new(125.0, 125.0, 0.0), Vector::new(0.0, -5.0, 0.0), Vector::default(), 20000.0, true),
        Body::new(Vector::new(375.0, 375.0, 0.0), Vector::new(0.0, 5.0, 0.0), Vector::default(), 20000.0, true),
    ];

    for _ in 0..FRAMES {
        engine.render(&bodies);
        body::update_bodies(&mut bodies);
    }
}