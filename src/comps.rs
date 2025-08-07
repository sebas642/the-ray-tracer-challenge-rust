use crate::intersection::Intersections;

use super::intersection::Intersection;
use super::ray::Ray;
use super::shape::BoxShape;
use super::tuple::Tuple;
use super::utils;

/// Structure used to store helpful computations
pub struct Comps {
    pub t: f64,
    pub object: BoxShape,

    pub point: Tuple,
    pub over_point: Tuple,
    pub under_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,

    pub inside: bool,

    pub reflectv: Tuple,

    pub n1: f64,
    pub n2: f64,
}

impl Comps {
    pub fn prepare_computations(i: &Intersection, r: &Ray, xs: Option<Intersections>) -> Comps {
        let point = r.position(i.t);
        let eyev = -r.direction;
        let mut normalv = i.object.normal_at(&point);
        let xs = xs.unwrap_or_else(|| Intersections::new(vec![i.clone()]));

        let inside = if Tuple::dot_product(&normalv, &eyev) < 0. {
            normalv = -normalv;
            true
        } else {
            false
        };

        let mut n1 = 0.;
        let mut n2 = 0.;

        let mut containers: Vec<&BoxShape> = Vec::new();
        for i_iter in xs.iter() {
            if i == i_iter {
                if containers.is_empty() {
                    n1 = 1.;
                } else {
                    n1 = containers.last().unwrap().material().refractive_index;
                }
            }

            if let Some(index) = containers.iter().position(|x| &i_iter.object == *x) {
                containers.remove(index);
            } else {
                containers.push(&i_iter.object);
            }

            if i == i_iter {
                if containers.is_empty() {
                    n2 = 1.;
                } else {
                    n2 = containers.last().unwrap().material().refractive_index;
                }

                break;
            }
        }

        let over_point = point + (normalv * utils::EPSILON);
        let under_point = point - (normalv * utils::EPSILON);

        Comps {
            t: i.t,
            object: i.object.clone(),
            point,
            over_point,
            under_point,
            eyev,
            normalv,
            inside,
            reflectv: Tuple::reflect(&r.direction, &normalv),
            n1,
            n2,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use super::*;
    use crate::plane::Plane;
    use crate::sphere::Sphere;
    use crate::transform;

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));
        let s = Sphere::default_boxed();

        let i = Intersection::new(4., s);

        let comps = Comps::prepare_computations(&i, &r, None);
        assert_eq!(i.t, comps.t);
        assert_eq!(&i.object, &comps.object);
        assert_eq!(Tuple::point(0., 0., -1.), comps.point);
        assert_eq!(Tuple::vector(0., 0., -1.), comps.eyev);
        assert_eq!(Tuple::vector(0., 0., -1.), comps.normalv);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));
        let s = Sphere::default_boxed();

        let i = Intersection::new(4., s);

        let comps = Comps::prepare_computations(&i, &r, None);
        assert_eq!(false, comps.inside);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(&Tuple::point(0., 0., 0.), &Tuple::vector(0., 0., 1.));
        let s = Sphere::default_boxed();

        let i = Intersection::new(1., s);

        let comps = Comps::prepare_computations(&i, &r, None);
        assert_eq!(true, comps.inside);
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));
        let s = Sphere::new_boxed(Some(transform::translation(0., 0., 1.)), None);

        let i = Intersection::new(5., s);

        let comps = Comps::prepare_computations(&i, &r, None);
        assert!(comps.over_point.z < -utils::EPSILON / 2.);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let r = Ray::new(
            &Tuple::point(0., 1., -1.),
            &Tuple::vector(0., -SQRT_2 / 2., SQRT_2 / 2.),
        );
        let s = Plane::default_boxed();

        let i = Intersection::new(SQRT_2, s);

        let comps = Comps::prepare_computations(&i, &r, None);
        assert_eq!(comps.reflectv, Tuple::vector(0., SQRT_2 / 2., SQRT_2 / 2.));
    }
}
