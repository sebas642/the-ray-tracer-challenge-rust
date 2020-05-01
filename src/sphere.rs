use super::matrix::{Matrix, MATRIX_IDENTITY};
use super::tuple::Tuple;
use super::ray::Ray;
use super::shape::{Shape, BoxShape};
use super::intersection::{Intersection, Intersections};

use std::any::Any;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub origin: Tuple,
    transform: Matrix
}

impl Sphere {
    pub fn new(transform: Option<Matrix>) -> Self {
        Self {
            origin: Tuple::point(0., 0., 0.),
            transform: transform.unwrap_or_default()
        }
    }

    pub fn new_boxed(transform: Option<Matrix>) -> BoxShape {
        Box::new(Sphere::new(transform))
    }

    pub fn default_boxed() -> BoxShape {
        Box::new(Sphere::default())
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            origin: Tuple::point(0., 0., 0.),
            transform: MATRIX_IDENTITY
        }
    }
}

impl PartialEq for Sphere {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform
    }
}

impl Shape for Sphere {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn box_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn box_clone(&self) -> BoxShape {
        Box::new(*self)
    }

    fn intersect(&self, r: Ray) -> Intersections {
        let mut xs: Vec<Intersection> = Vec::new();

        let r = r.transform(self.transform.inverse());

        let sphere_to_ray = r.origin - self.origin;
        let a = Tuple::dot_product(&r.direction, &r.direction);
        let b = 2. * Tuple::dot_product(&r.direction, &sphere_to_ray);
        let c = Tuple::dot_product(&sphere_to_ray, &sphere_to_ray) - 1.0;

        let d = b.powi(2) - (4. * a * c);

        if d >= 0. {
            xs.push(Intersection::new((-1. * b - d.sqrt()) / (2. * a), Box::new(*self)));
            xs.push(Intersection::new((-1. * b + d.sqrt()) / (2. * a), Box::new(*self)));
        }

        Intersections::new(xs)
    }

    fn transformation(&self) -> Matrix {
        self.transform
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transform;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));
        let s = Sphere::default_boxed();

        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0].t);
        assert_eq!(6.0, xs[1].t);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {

        let r = Ray::new(&Tuple::point(0., 1., -5.), &Tuple::vector(0., 0., 1.));
        let s = Sphere::default_boxed();

        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(5.0, xs[0].t);
        assert_eq!(5.0, xs[1].t);
    }


    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(&Tuple::point(0., 2., -5.), &Tuple::vector(0., 0., 1.));
        let s = Sphere::default_boxed();

        let xs = s.intersect(r);
        assert_eq!(0, xs.len());
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(&Tuple::point(0., 0., 0.), &Tuple::vector(0., 0., 1.));
        let s = Sphere::default_boxed();

        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(-1.0, xs[0].t);
        assert_eq!(1.0, xs[1].t);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(&Tuple::point(0., 0., 5.), &Tuple::vector(0., 0., 1.));
        let s = Sphere::default_boxed();

        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(-6.0, xs[0].t);
        assert_eq!(-4.0, xs[1].t);
    }

    #[test]
    fn intersects_sets_the_object_on_the_intersection() {
        let r = Ray::new(&Tuple::point(0., 0., 5.), &Tuple::vector(0., 0., 1.));
        let s = Sphere::default_boxed();

        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(&s, &xs[0].object);
    }

    #[test]
    fn a_sphere_has_a_default_transform() {
        let s = Sphere::default_boxed();

        assert_eq!(s.transformation(), MATRIX_IDENTITY);
    }

    #[test]
    fn changing_a_spheres_transformation() {
        let t = transform::translation(2., 3., 4.);
        let s = Sphere::new_boxed(Some(t));

        assert_eq!(s.transformation(), t);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));
        let s = Sphere::new_boxed(Some(transform::scaling(2., 2., 2.)));

        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(3., xs[0].t);
        assert_eq!(7., xs[1].t);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));
        let s = Sphere::new_boxed(Some(transform::translation(5., 0., 0.)));

        let xs = s.intersect(r);
        assert_eq!(0, xs.len());
    }
}
