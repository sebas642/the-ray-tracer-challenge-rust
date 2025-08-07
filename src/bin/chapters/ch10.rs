extern crate the_ray_tracer_challenge_rust as tracer;
use tracer::camera::Camera;
use tracer::canvas::Canvas;
use tracer::color::{Color, WHITE};
use tracer::light::PointLight;
use tracer::material::Material;
use tracer::pattern::{CheckersPattern, GradientPattern, PatternType, StripePattern};
use tracer::plane::Plane;
use tracer::shape::BoxShape;
use tracer::sphere::Sphere;
use tracer::transform;
use tracer::transformation::view_transform;
use tracer::tuple::Tuple;
use tracer::world::World;

use std::f64;

fn floor() -> BoxShape {
    let pattern_transform = transform::scaling(0.5, 0.5, 0.5);
    let pattern = CheckersPattern {
        first: Color::new(1., 0.9, 0.9),
        second: Color::new(0.9, 0.7, 0.7),
        transform: pattern_transform,
    };
    let material = Material::new(
        Some(Color::new(1., 0.9, 0.9)),
        Some(PatternType::Checkers(pattern)),
        None,
        None,
        Some(0.),
        None,
        None,
        None,
        None,
    );

    Plane::new_boxed(None, Some(material))
}

fn wall() -> BoxShape {
    let pattern_transform = transform::scaling(2., 1., 1.);
    let pattern = GradientPattern {
        first: Color::new(0.1, 0.1, 0.1),
        second: Color::new(0.2, 0.2, 0.2),
        transform: pattern_transform,
    };

    Plane::new_boxed(
        Some(transform::transforms(&[
            transform::rotation_x(f64::consts::FRAC_PI_2),
            transform::translation(15., 15., 15.),
        ])),
        Some(Material::new(
            None,
            Some(PatternType::Gradient(pattern)),
            None,
            None,
            Some(0.),
            None,
            None,
            None,
            None,
        )),
    )
}

pub fn ch10() -> Canvas {
    let s_pattern_tr = transform::transforms(&[
        transform::rotation_x(2.),
        transform::rotation_y(2.),
        transform::rotation_z(2.),
        transform::scaling(0.2, 0.2, 0.2),
    ]);
    let s_pattern = StripePattern {
        first: Color::new(0., 1., 0.),
        second: Color::new(0.3, 0.6, 0.),
        transform: s_pattern_tr,
    };
    let middle_m = Material::new(
        Some(Color::new(0.1, 1., 0.5)),
        Some(PatternType::Stripe(s_pattern)),
        None,
        Some(0.7),
        Some(0.3),
        None,
        None,
        None,
        None,
    );
    let middle_sphere =
        Sphere::new_boxed(Some(transform::translation(-0.5, 1., 0.5)), Some(middle_m));

    let right_tr = transform::transforms(&[
        transform::scaling(0.5, 0.5, 0.5),
        transform::translation(1.5, 0.5, -0.5),
    ]);

    let right_m = Material::new(
        Some(Color::new(0.2, 1., 0.9)),
        None,
        None,
        Some(0.7),
        Some(0.3),
        None,
        None,
        None,
        None,
    );
    let right_sphere = Sphere::new_boxed(Some(right_tr), Some(right_m));

    let left_tr = transform::transforms(&[
        transform::scaling(0.33, 0.33, 0.33),
        transform::translation(-1.5, 0.33, -0.75),
    ]);
    let left_m = Material::new(
        Some(Color::new(1., 0.8, 0.1)),
        None,
        None,
        Some(0.7),
        Some(0.3),
        None,
        None,
        None,
        None,
    );
    let left_sphere = Sphere::new_boxed(Some(left_tr), Some(left_m));

    let light = PointLight::new(&Tuple::point(-10., 10., -10.), &WHITE);
    let world = World::new(
        Some(light),
        vec![floor(), wall(), middle_sphere, right_sphere, left_sphere],
    );

    let c_from = Tuple::point(0., 1.5, -5.);
    let c_to = Tuple::point(0., 1., 0.);
    let c_up = Tuple::vector(0., 1., 0.);
    let c_transform = view_transform(&c_from, &c_to, &c_up);
    let camera = Camera::new(640, 480, f64::consts::FRAC_PI_3, Some(c_transform));

    camera.render(&world)
}
