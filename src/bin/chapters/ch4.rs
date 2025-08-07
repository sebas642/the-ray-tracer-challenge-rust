extern crate the_ray_tracer_challenge_rust as tracer;
use tracer::canvas::Canvas;
use tracer::color;
use tracer::transform;
use tracer::tuple::Tuple;

use std::f64;

pub fn ch4() -> Canvas {
    const CANVAS_SIZE: usize = 200;

    let mut canvas = Canvas::new(CANVAS_SIZE, CANVAS_SIZE);
    let p0 = Tuple::point(0., CANVAS_SIZE as f64 * 0.4, 0.);

    let center = transform::translation(CANVAS_SIZE as f64 / 2., CANVAS_SIZE as f64 / 2., 0.);

    // Create markers for every minutes
    for m in 0..60 {
        let r = transform::rotation_z((2. * f64::consts::PI / 60.) * m as f64);
        let t = transform::transforms(&[r, center]) * p0;
        canvas.write_pixel(t.x as usize, t.y as usize, color::RED);
    }

    // Create markers for every hours
    for h in 0..12 {
        let r = transform::rotation_z((2. * f64::consts::PI / 12.) * h as f64);
        let t = transform::transforms(&[r, center]);
        for l in 0..10 {
            let p1 = Tuple::point(p0.x, p0.y - (l as f64), p0.z);
            let p1t = t * p1;
            canvas.write_pixel(p1t.x as usize, p1t.y as usize, color::RED);
        }
    }

    return canvas;
}
