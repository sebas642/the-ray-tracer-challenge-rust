extern crate the_ray_tracer_challenge_rust as tracer;
use tracer::canvas::Canvas;
use tracer::color::{Color, WHITE};
use tracer::light::PointLight;
use tracer::material::Material;
use tracer::ppm;
use tracer::ray::Ray;
use tracer::sphere::Sphere;
use tracer::tuple::Tuple;

use std::f64;

fn main() {
    const CANVAS_SIZE: usize = 200;

    // The light is at z = -5
    let ray_origin = Tuple::point(0., 0., -5.);
    let wall_z = 10.;
    let wall_size = 7.;
    let pixel_size = wall_size / CANVAS_SIZE as f64;
    let half = wall_size / 2.;

    let mut canvas = Canvas::new(CANVAS_SIZE, CANVAS_SIZE);
    let material = Material::new(Some(Color::new(1., 0.2, 1.)), None, None, None, None, None);
    let shape = Sphere::new_boxed(None, Some(material));

    let light_position = Tuple::point(-10., 10., -10.);
    let light_color = WHITE;
    let light = PointLight::new(&light_position, &light_color);

    for y in 0..CANVAS_SIZE {
        let world_y = half - pixel_size * y as f64;
        for x in 0..CANVAS_SIZE {
            let world_x = -half + pixel_size * x as f64;
            let position = Tuple::point(world_x, world_y, wall_z);
            let direction = position - ray_origin;

            let r = Ray::new(&ray_origin, &direction.normalize());
            let xs = shape.intersect(r);

            if xs.hit().is_some() {
                let hit = xs.hit().unwrap();
                let point = r.position(hit.t);
                let normal = hit.object.normal_at(&point);
                let eye = -r.direction;

                let color = hit.object.material().lighting(
                    &hit.object,
                    &light,
                    &point,
                    &eye,
                    &normal,
                    false,
                );
                canvas.write_pixel(x, y, color);
            }
        }
    }

    println!("{}", ppm::canvas_to_ppm(canvas));
}
