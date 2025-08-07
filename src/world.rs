use super::color::{BLACK, Color, WHITE};
use super::comps::Comps;
use super::intersection::{Intersection, Intersections};
use super::light::PointLight;
use super::material::Material;
use super::ray::Ray;
use super::shape::BoxShape;
use super::sphere::Sphere;
use super::transform;
use super::tuple::Tuple;

#[derive(PartialEq)]
pub struct World {
    pub light: Option<PointLight>,
    pub shapes: Vec<BoxShape>,
}

impl World {
    pub fn new(light: Option<PointLight>, shapes: Vec<BoxShape>) -> World {
        World { light, shapes }
    }

    pub fn intersect(&self, &r: &Ray) -> Intersections {
        let mut intersections: Vec<Intersection> = vec![];
        self.shapes.iter().for_each(|shape| {
            shape
                .intersect(r)
                .iter()
                .for_each(|i| intersections.push(i.clone()))
        });
        Intersections::new(intersections)
    }

    pub fn shade_hit(&self, comps: &Comps, remaining: u8) -> Color {
        if self.light.is_some() && remaining > 0 {
            let material = comps.object.material();
            let surface = material.lighting(
                &comps.object,
                &self.light.unwrap(),
                &comps.point,
                &comps.eyev,
                &comps.normalv,
                self.is_shadowed(&comps.over_point),
            );
            let reflected = self.reflected_color(comps, remaining - 1);
            let refracted = self.refracted_color(comps, remaining - 1);

            if material.reflective > 0. && material.transparency > 0. {
                let reflectance = schlick(comps);
                return surface + reflected * reflectance + refracted * (1. - reflectance);
            }
            return surface + reflected + refracted;
        } else {
            BLACK
        }
    }

    pub fn color_at(&self, r: &Ray, remaining: u8) -> Color {
        let i = self.intersect(r);
        match i.hit() {
            None => BLACK,
            Some(h) => {
                let comps = Comps::prepare_computations(h, r, Some(i.clone()));
                self.shade_hit(&comps, remaining)
            }
        }
    }

    pub fn reflected_color(&self, comps: &Comps, remaining: u8) -> Color {
        if comps.object.material().reflective == 0. || remaining == 0 {
            return BLACK;
        }
        let reflect_ray = Ray::new(&comps.over_point, &comps.reflectv);
        let color = self.color_at(&reflect_ray, remaining);
        return color * comps.object.material().reflective;
    }

    pub fn refracted_color(&self, comps: &Comps, remaining: u8) -> Color {
        if comps.object.material().transparency == 0. || remaining == 0 {
            return BLACK;
        }

        // Find the ratio of first index of refraction to the second.
        // This is inverted from the definition of Snell's Law.
        let n_ratio = comps.n1 / comps.n2;

        // cos(theta_i) is the same as the dot product of the two vectors
        let cos_i = Tuple::dot_product(&comps.eyev, &comps.normalv);

        // Find sin(theta_t)^2 via trigonometric identity
        let sin2_t = n_ratio * n_ratio * (1. - (cos_i * cos_i));
        if sin2_t > 1. {
            return BLACK; // Total internal reflection
        }

        // Find cos(theta_t) via trigonometric identity
        let cos_t = (1.0 - sin2_t).sqrt();

        // Compute the direction of the refracted ray
        let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;

        // Create the refracted ray
        let refracted_ray = Ray::new(&comps.under_point, &direction);

        // Find the color at the refracted ray
        // making sure to multiply by the transparency value to account for any opacity
        self.color_at(&refracted_ray, remaining - 1) * comps.object.material().transparency
    }

    pub fn is_shadowed(&self, &point: &Tuple) -> bool {
        if self.light.is_none() {
            false
        } else {
            let v = self.light.unwrap().position - point;
            let distance = v.magnitude();
            let direction = v.normalize();

            let r = Ray::new(&point, &direction);
            let intersections = self.intersect(&r);
            let h = intersections.hit();
            h.is_some() && h.unwrap().t < distance
        }
    }
}

impl Default for World {
    fn default() -> World {
        let light = PointLight::new(&Tuple::point(-10., 10., -10.), &WHITE);

        let diffuse = 0.7;
        let specular = 0.2;
        let material = Material::new(
            Some(Color::new(0.8, 1., 0.6)),
            None,
            None,
            Some(diffuse),
            Some(specular),
            None,
            None,
            None,
            None,
        );
        let s1 = Sphere::new_boxed(None, Some(material));
        let s2 = Sphere::new_boxed(Some(transform::scaling(0.5, 0.5, 0.5)), None);

        World::new(Some(light), vec![s1, s2])
    }
}

fn schlick(comps: &Comps) -> f64 {
    // Find the cosine of the angle between the eye and normal vectors
    let mut cos = Tuple::dot_product(&comps.eyev, &comps.normalv);

    // Total internal reflection can only occur if n1 > n2
    if comps.n1 > comps.n2 {
        let n = comps.n1 / comps.n2;
        let sin2_t = n * n * (1. - cos * cos);
        if sin2_t > 1.0 {
            return 1.0; // Total internal reflection
        }

        // Compute cosine of theta_t using trigonometric identity
        let cos_t = (1.0 - sin2_t).sqrt();

        // When n1 > n2, use cos(theta_t) instead
        cos = cos_t;
    }

    let r0 = ((comps.n1 - comps.n2) / (comps.n1 + comps.n2)).powi(2);
    return r0 + ((1. - r0) * (1. - cos).powi(5));
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use super::*;
    use crate::{
        pattern::{PatternType, TestPattern},
        plane::Plane,
        utils::approx_eq,
    };

    #[test]
    fn creating_a_world() {
        let w = World::new(None, vec![]);

        assert_eq!(None, w.light);
    }

    #[test]
    fn the_default_world() {
        let light = PointLight::new(&Tuple::point(-10., 10., -10.), &WHITE);

        let diffuse = 0.7;
        let specular = 0.2;
        let material = Material::new(
            Some(Color::new(0.8, 1., 0.6)),
            None,
            None,
            Some(diffuse),
            Some(specular),
            None,
            None,
            None,
            None,
        );
        let s1 = Sphere::new_boxed(None, Some(material));
        let s2 = Sphere::new_boxed(Some(transform::scaling(0.5, 0.5, 0.5)), None);

        let w = World::default();
        assert_eq!(light, w.light.unwrap());
        assert!(w.shapes.contains(&s1));
        assert!(w.shapes.contains(&s2));
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = World::default();
        let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));

        let xs = w.intersect(&r);
        assert_eq!(4, xs.len());
        assert_eq!(4., xs[0].t);
        assert_eq!(4.5, xs[1].t);
        assert_eq!(5.5, xs[2].t);
        assert_eq!(6., xs[3].t);
    }

    #[test]
    fn shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));
        let s = w.shapes[0].clone();

        let i = Intersection::new(4., s);

        let comps = Comps::prepare_computations(&i, &r, None);
        let c = w.shade_hit(&comps, 1);
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), c);
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let light = PointLight::new(&Tuple::point(0., 0.25, 0.), &WHITE);
        let w = World {
            light: Some(light),
            ..Default::default()
        };
        let r = Ray::new(&Tuple::point(0., 0., 0.), &Tuple::vector(0., 0., 1.));
        let s = w.shapes[1].clone();

        let i = Intersection::new(0.5, s);

        let comps = Comps::prepare_computations(&i, &r, None);
        let c = w.shade_hit(&comps, 1);
        assert_eq!(Color::new(0.90498, 0.90498, 0.90498), c);
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let light = PointLight::new(&Tuple::point(0., 0., -10.), &WHITE);
        let s1 = Sphere::default_boxed();
        let s2 = Sphere::new_boxed(Some(transform::translation(0., 0., 10.)), None);
        let w = World::new(Some(light), vec![s1, s2.clone()]);

        let r = Ray::new(&Tuple::point(0., 0., 5.), &Tuple::vector(0., 0., 1.));
        let i = Intersection::new(4., s2);

        let comps = Comps::prepare_computations(&i, &r, None);
        let c = w.shade_hit(&comps, 1);
        assert_eq!(Color::new(0.1, 0.1, 0.1), c);
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = World::default();
        let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 1., 0.));

        let c = w.color_at(&r, 1);
        assert_eq!(BLACK, c);
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = World::default();
        let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));

        let c = w.color_at(&r, 1);
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), c);
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let mut w = World::default();
        let material = Material {
            color: w.shapes[0].material().color,
            pattern: w.shapes[0].material().pattern.clone(),
            ambient: 1.,
            diffuse: w.shapes[0].material().diffuse,
            specular: w.shapes[0].material().specular,
            shininess: w.shapes[0].material().shininess,
            reflective: w.shapes[0].material().reflective,
            transparency: 0.,
            refractive_index: 1.,
        };
        w.shapes.first_mut().unwrap().set_material(material.clone());
        w.shapes.last_mut().unwrap().set_material(material);

        let r = Ray::new(&Tuple::point(0., 0., 0.75), &Tuple::vector(0., 0., -1.));
        let c = w.color_at(&r, 1);
        assert_eq!(w.shapes[1].material().color, c);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::default();
        let p = Tuple::point(0., 10., 0.);

        assert_eq!(false, w.is_shadowed(&p));
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_light() {
        let w = World::default();
        let p = Tuple::point(10., -10., 10.);

        assert_eq!(true, w.is_shadowed(&p));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = World::default();
        let p = Tuple::point(-20., 20., -20.);

        assert_eq!(false, w.is_shadowed(&p));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = World::default();
        let p = Tuple::point(-2., 2., -2.);

        assert_eq!(false, w.is_shadowed(&p));
    }

    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let light = PointLight::new(&Tuple::point(-10., 10., -10.), &WHITE);
        let mut material = Material::default();
        material.ambient = 1.;
        let s = Sphere::new_boxed(Some(transform::scaling(0.5, 0.5, 0.5)), Some(material));
        let w = World::new(Some(light), vec![s.clone()]);

        let r = Ray::new(&Tuple::point(0., 0., 0.), &Tuple::vector(0., 0., 1.));
        let i = Intersection::new(1., s);

        let comps = Comps::prepare_computations(&i, &r, None);
        assert_eq!(BLACK, w.reflected_color(&comps, 1));
    }

    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let light = PointLight::new(&Tuple::point(-10., 10., -10.), &WHITE);
        let m1 = Material::new(
            Some(Color::new(0.8, 1., 0.6)),
            None,
            None,
            Some(0.7),
            Some(0.2),
            None,
            None,
            None,
            None,
        );
        let s1 = Sphere::new_boxed(None, Some(m1));
        let s2 = Sphere::new_boxed(Some(transform::scaling(0.5, 0.5, 0.5)), None);

        let mut m3 = Material::default();
        m3.reflective = 0.5;
        let plane = Plane::new_boxed(Some(transform::translation(0., -1., 0.)), Some(m3));

        let w = World::new(Some(light), vec![plane, s1, s2]);

        let r = Ray::new(
            &Tuple::point(0., 0., -3.),
            &Tuple::vector(0., -SQRT_2 / 2., SQRT_2 / 2.),
        );
        let i = Intersection::new(SQRT_2, w.shapes[0].clone());

        let comps = Comps::prepare_computations(&i, &r, None);
        let reflected_color = w.reflected_color(&comps, 1);
        assert_eq!(Color::new(0.19033, 0.23791, 0.14274), reflected_color);
    }

    #[test]
    fn shade_hit_with_a_reflective_material() {
        let light = PointLight::new(&Tuple::point(-10., 10., -10.), &WHITE);
        let m1 = Material::new(
            Some(Color::new(0.8, 1., 0.6)),
            None,
            None,
            Some(0.7),
            Some(0.2),
            None,
            None,
            None,
            None,
        );
        let s1 = Sphere::new_boxed(None, Some(m1));
        let s2 = Sphere::new_boxed(Some(transform::scaling(0.5, 0.5, 0.5)), None);

        let mut m3 = Material::default();
        m3.reflective = 0.5;
        let plane = Plane::new_boxed(Some(transform::translation(0., -1., 0.)), Some(m3));

        let w = World::new(Some(light), vec![plane, s1, s2]);

        let r = Ray::new(
            &Tuple::point(0., 0., -3.),
            &Tuple::vector(0., -SQRT_2 / 2., SQRT_2 / 2.),
        );
        let i = Intersection::new(SQRT_2, w.shapes[0].clone());

        let comps = Comps::prepare_computations(&i, &r, None);
        let color = w.shade_hit(&comps, 2);
        assert_eq!(Color::new(0.87676, 0.92434, 0.82917), color);
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces_terminates() {
        // 2 parallel reflective planes with a light source in between
        // Light will bounce infinitely
        let light = PointLight::new(&Tuple::point(0., 0., 0.), &WHITE);

        let material = Material::new(
            Some(Color::new(0.8, 1., 0.6)),
            None,
            None,
            Some(0.7),
            Some(0.2),
            None,
            Some(1.0),
            None,
            None,
        );
        let lower = Plane::new_boxed(
            Some(transform::translation(0., -1., 0.)),
            Some(material.clone()),
        );
        let upper = Plane::new_boxed(Some(transform::translation(0., 1., 0.)), Some(material));

        let w = World::new(Some(light), vec![lower, upper]);

        let r = Ray::new(&Tuple::point(0., 0., 0.), &Tuple::vector(0., 1., 0.));

        w.color_at(&r, 1);
    }

    #[test]
    fn the_reflected_color_at_the_maximum_recursive_depth() {
        let light = PointLight::new(&Tuple::point(-10., 10., -10.), &WHITE);
        let m1 = Material::new(
            Some(Color::new(0.8, 1., 0.6)),
            None,
            None,
            Some(0.7),
            Some(0.2),
            None,
            None,
            None,
            None,
        );
        let s1 = Sphere::new_boxed(None, Some(m1));
        let s2 = Sphere::new_boxed(Some(transform::scaling(0.5, 0.5, 0.5)), None);

        let mut m3 = Material::default();
        m3.reflective = 0.5;
        let plane = Plane::new_boxed(Some(transform::translation(0., -1., 0.)), Some(m3));

        let w = World::new(Some(light), vec![plane, s1, s2]);

        let r = Ray::new(
            &Tuple::point(0., 0., -3.),
            &Tuple::vector(0., -SQRT_2 / 2., SQRT_2 / 2.),
        );
        let i = Intersection::new(SQRT_2, w.shapes[0].clone());

        let comps = Comps::prepare_computations(&i, &r, None);
        let color = w.reflected_color(&comps, 0);
        assert_eq!(BLACK, color);
    }

    #[test]
    fn the_refracted_color_with_an_opaque_surface() {
        let w = World::default();
        let s = w.shapes[0].clone();
        let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));

        let i0 = Intersection::new(4., s.clone());
        let i1 = Intersection::new(6., s.clone());
        let xs = Intersections::new(vec![i0.clone(), i1.clone()]);

        let comps = Comps::prepare_computations(&i0, &r, Some(xs));
        assert_eq!(BLACK, w.refracted_color(&comps, 5))
    }

    #[test]
    fn the_refracted_color_at_the_maximum_recursive_depth() {
        let w = World::default();
        let mut s = w.shapes[0].clone();
        let mut material = s.material().clone();
        material.transparency = 1.0;
        material.refractive_index = 1.5;
        s.set_material(material);

        let w = World::new(w.light, vec![s.clone(), w.shapes[0].clone()]);

        let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));

        let i0 = Intersection::new(4., s.clone());
        let i1 = Intersection::new(6., s.clone());

        let xs = Intersections::new(vec![i0.clone(), i1.clone()]);
        let comps = Comps::prepare_computations(&i0, &r, Some(xs));
        let color = w.refracted_color(&comps, 0);
        assert_eq!(BLACK, color);
    }

    #[test]
    fn the_refracted_color_under_total_internal_reflection() {
        let w = World::default();
        let mut s = w.shapes[0].clone();
        let mut material = s.material().clone();
        material.transparency = 1.0;
        material.refractive_index = 1.5;
        s.set_material(material);

        let r = Ray::new(
            &Tuple::point(0., 0., SQRT_2 / 2.),
            &Tuple::vector(0., 1., 0.),
        );

        let w = World::new(w.light, vec![s.clone(), w.shapes[0].clone()]);

        let i0 = Intersection::new(-SQRT_2 / 2., s.clone());
        let i1 = Intersection::new(SQRT_2 / 2., s.clone());

        let xs = Intersections::new(vec![i0.clone(), i1.clone()]);
        let comps = Comps::prepare_computations(&i1, &r, Some(xs));
        let color = w.refracted_color(&comps, 5);
        assert_eq!(BLACK, color);
    }

    #[test]
    fn the_refracted_color_with_a_refacted_ray() {
        let w = World::default();

        let mut s1 = w.shapes[0].clone();
        let mut material1 = s1.material().clone();
        material1.ambient = 1.0;
        material1.pattern = Some(PatternType::Test(TestPattern::new(None)));
        s1.set_material(material1);

        let mut s2 = w.shapes[1].clone();
        let mut material2 = s2.material().clone();
        material2.transparency = 1.0;
        material2.refractive_index = 1.5;
        s2.set_material(material2);

        let w = World::new(w.light, vec![s1.clone(), s2.clone()]);

        let r = Ray::new(&Tuple::point(0., 0., 0.1), &Tuple::vector(0., 1., 0.));

        let i0 = Intersection::new(-0.9899, s1.clone());
        let i1 = Intersection::new(-0.4899, s2.clone());
        let i2 = Intersection::new(0.4899, s2.clone());
        let i3 = Intersection::new(0.9899, s1.clone());

        let xs = Intersections::new(vec![i0.clone(), i1.clone(), i2.clone(), i3.clone()]);
        let comps = Comps::prepare_computations(&i2, &r, Some(xs));
        let color = w.refracted_color(&comps, 5);
        assert_eq!(Color::new(0., 0.99888, 0.04722), color);
    }

    #[test]
    fn shade_hit_with_a_transparent_meterial() {
        let w = World::default();
        let floor = Plane::new_boxed(
            Some(transform::translation(0., -1., 0.)),
            Some(Material {
                transparency: 0.5,
                refractive_index: 1.5,
                ..Default::default()
            }),
        );
        let ball = Sphere::new_boxed(
            Some(transform::translation(0., -3.5, -0.5)),
            Some(Material {
                color: Color::new(1., 0., 0.),
                ambient: 0.5,
                ..Default::default()
            }),
        );

        let w = World::new(
            w.light,
            vec![
                w.shapes[0].clone(),
                w.shapes[1].clone(),
                floor.clone(),
                ball,
            ],
        );

        let r = Ray::new(
            &Tuple::point(0., 0., -3.),
            &Tuple::vector(0., -SQRT_2 / 2., SQRT_2 / 2.),
        );

        let i = Intersection::new(SQRT_2, floor);

        let xs = Intersections::new(vec![i.clone()]);
        let comps = Comps::prepare_computations(&i, &r, Some(xs));
        let color = w.shade_hit(&comps, 5);
        assert_eq!(Color::new(0.93642, 0.68642, 0.68642), color);
    }

    #[test]
    fn the_schlick_approximation_under_total_internal_reflection() {
        let s = Sphere::new_boxed(
            None,
            Some(Material {
                transparency: 1.0,
                refractive_index: 1.5,
                ..Default::default()
            }),
        );

        let r = Ray::new(
            &Tuple::point(0., 0., SQRT_2 / 2.),
            &Tuple::vector(0., 1., 0.),
        );

        let i0 = Intersection::new(-SQRT_2 / 2., s.clone());
        let i1 = Intersection::new(SQRT_2 / 2., s.clone());

        let xs = Intersections::new(vec![i0.clone(), i1.clone()]);
        let comps = Comps::prepare_computations(&i1, &r, Some(xs));
        assert_eq!(1.0, schlick(&comps));
    }

    #[test]
    fn the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let s = Sphere::new_boxed(
            None,
            Some(Material {
                transparency: 1.0,
                refractive_index: 1.5,
                ..Default::default()
            }),
        );

        let r = Ray::new(&Tuple::point(0., 0., 0.), &Tuple::vector(0., 1., 0.));

        let i0 = Intersection::new(-1., s.clone());
        let i1 = Intersection::new(1., s.clone());

        let xs = Intersections::new(vec![i0.clone(), i1.clone()]);
        let comps = Comps::prepare_computations(&i1, &r, Some(xs));
        assert!(approx_eq(0.04, schlick(&comps)));
    }

    #[test]
    fn the_schlick_approximation_with_small_angle_and_n2_greather_than_n1() {
        let s = Sphere::new_boxed(
            None,
            Some(Material {
                transparency: 1.0,
                refractive_index: 1.5,
                ..Default::default()
            }),
        );

        let r = Ray::new(&Tuple::point(0., 0.99, -2.), &Tuple::vector(0., 0., 1.));

        let i0 = Intersection::new(1.8589, s.clone());

        let xs = Intersections::new(vec![i0.clone()]);
        let comps = Comps::prepare_computations(&i0, &r, Some(xs));
        assert!(approx_eq(0.48873, schlick(&comps)));
    }

    #[test]
    fn shade_hit_with_a_reflective_transparent_material() {
        let w = World::default();
        let r = Ray::new(
            &Tuple::point(0., 0., -3.),
            &Tuple::vector(0., -SQRT_2 / 2., SQRT_2 / 2.),
        );

        let floor = Plane::new_boxed(
            Some(transform::translation(0., -1., 0.)),
            Some(Material {
                reflective: 0.5,
                transparency: 0.5,
                refractive_index: 1.5,
                ..Default::default()
            }),
        );

        let ball = Sphere::new_boxed(
            Some(transform::translation(0., -3.5, -0.5)),
            Some(Material {
                color: Color::new(1., 0., 0.),
                ambient: 0.5,
                ..Default::default()
            }),
        );

        let w = World::new(
            w.light,
            vec![
                w.shapes[0].clone(),
                w.shapes[1].clone(),
                floor.clone(),
                ball,
            ],
        );

        let i0 = Intersection::new(SQRT_2, floor);
        let xs = Intersections::new(vec![i0.clone()]);
        let comps = Comps::prepare_computations(&i0, &r, Some(xs));

        assert_eq!(
            Color::new(0.93391, 0.69643, 0.69243),
            w.shade_hit(&comps, 5)
        );
    }
}
