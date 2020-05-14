extern crate the_ray_tracer_challenge_rust as tracer;
use tracer::camera::Camera;
use tracer::color::{Color, WHITE};
use tracer::light::PointLight;
use tracer::material::Material;
use tracer::ppm;
use tracer::sphere::Sphere;
use tracer::tuple::Tuple;
use tracer::transform;
use tracer::transformation::view_transform;
use tracer::world::World;

use std::f64;

fn main() {
    let floor_tr = transform::scaling(10., 0.01, 10.);
    let floor_m = Material::new(Some(Color::new(1., 0.9, 0.9)), None, None, Some(0.), None);
    let floor = Sphere::new_boxed(Some(floor_tr), Some(floor_m));

    let left_wall_tr = transform::transforms(&[
        floor_tr,
        transform::rotation_x(f64::consts::FRAC_PI_2),
        transform::rotation_y(-f64::consts::FRAC_PI_4),
        transform::translation(0., 0., 5.)
    ]);
    let left_wall = Sphere::new_boxed(Some(left_wall_tr), Some(floor_m));

    let right_wall_tr = transform::transforms(&[
        floor_tr,
        transform::rotation_x(f64::consts::FRAC_PI_2),
        transform::rotation_y(f64::consts::FRAC_PI_4),
        transform::translation(0., 0., 5.)
    ]);
    let right_wall = Sphere::new_boxed(Some(right_wall_tr), Some(floor_m));

    let middle_m = Material::new(Some(Color::new(0.1, 1., 0.5)), None, Some(0.7), Some(0.3), None);
    let middle_sphere = Sphere::new_boxed(Some(transform::translation(-0.5, 1., 0.5)), Some(middle_m));

    let right_tr = transform::transforms(&[
        transform::scaling(0.5, 0.5, 0.5),
        transform::translation(1.5, 0.5, -0.5)
    ]);
    let right_m = Material::new(Some(Color::new(0.5, 1., 0.1)), None, Some(0.7), Some(0.3), None);
    let right_sphere = Sphere::new_boxed(Some(right_tr), Some(right_m));

    let left_tr = transform::transforms(&[
        transform::scaling(0.33, 0.33, 0.33),
        transform::translation(-1.5, 0.33, -0.75)
    ]);
    let left_m = Material::new(Some(Color::new(1., 0.8, 0.1)), None, Some(0.7), Some(0.3), None);
    let left_sphere = Sphere::new_boxed(Some(left_tr), Some(left_m));

    let light = PointLight::new(&Tuple::point(-10., 10., -10.), &WHITE);
    let world = World::new(Some(light), vec![floor, left_wall, right_wall, middle_sphere, right_sphere, left_sphere]);

    let c_from = Tuple::point(0., 1.5, -5.);
    let c_to = Tuple::point(0., 1., 0.);
    let c_up = Tuple::vector(0., 1., 0.);
    let c_transform = view_transform(&c_from, &c_to, &c_up);
    let camera = Camera::new(300, 150, f64::consts::FRAC_PI_3, Some(c_transform));

    let canvas = camera.render(&world);
    println!("{}", ppm::canvas_to_ppm(canvas));
}
