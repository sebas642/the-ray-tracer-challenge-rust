use super::intersection::{Intersection, Intersections};
use super::material::Material;
use super::matrix::Matrix;
use super::ray::Ray;
use super::shape::{Shape, BoxShape};
use super::tuple::{Tuple, POINT_ORIGIN};
use super::utils::EPSILON;

use std::any::Any;

#[derive(Debug, Copy, Clone)]
pub struct Plane {
    origin: Tuple,
    transform: Matrix,
    material: Material
}

impl Plane {
    pub fn new(transform: Option<Matrix>, material: Option<Material>) -> Self {
        Self {
            origin: POINT_ORIGIN,
            transform: transform.unwrap_or_default(),
            material: material.unwrap_or_default()
        }
    }

    pub fn new_boxed(transform: Option<Matrix>, material: Option<Material>) -> BoxShape {
        Box::new(Plane::new(transform, material))
    }

    pub fn default_boxed() -> BoxShape {
        Box::new(Plane::default())
    }
}

impl Default for Plane {
    fn default() -> Self {
        Plane::new(None, None)
    }
}

impl PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform
    }
}

impl Shape for Plane {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn box_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn box_clone(&self) -> BoxShape {
        Box::new(*self)
    }

    fn transformation(&self) -> Matrix {
        self.transform
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn set_material(&mut self, m: Material) {
        self.material = m;
    }

    fn local_normal_at(&self, _local_point: &Tuple) -> Tuple {
        Tuple::vector(0., 1., 0.)
    }

    fn local_intersect(&self, &r: &Ray) -> Intersections {
        let mut xs = vec![];
        if r.direction.y.abs() >= EPSILON {
            let t = -r.origin.y / r.direction.y;
            xs.push(Intersection::new(t, self.box_clone()));
        }

        Intersections::new(xs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_normal_of_a_place_is_a_constant_everywhere() {
        let p = Plane::default();

        let normal = Tuple::vector(0., 1., 0.);
        assert_eq!(normal, p.local_normal_at(&Tuple::point(0., 0., 0.)));
        assert_eq!(normal, p.local_normal_at(&Tuple::point(10., 0., -10.)));
        assert_eq!(normal, p.local_normal_at(&Tuple::point(-5., 0., 150.)));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_place() {
        let p = Plane::default();
        let r = Ray::new(&Tuple::point(0., 10., 0.), &Tuple::vector(0., 0., 1.));
        let xs = p.local_intersect(&r);
        assert_eq!(0, xs.len());
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = Plane::default();
        let r = Ray::new(&Tuple::point(0., 0., 0.), &Tuple::vector(0., 0., 1.));
        let xs = p.local_intersect(&r);
        assert_eq!(0, xs.len());
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = Plane::default_boxed();
        let r = Ray::new(&Tuple::point(0., 1., 0.), &Tuple::vector(0., -1., 0.));
        let xs = p.local_intersect(&r);
        assert_eq!(1, xs.len());
        assert_eq!(&p, &xs[0].object);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Plane::default_boxed();
        let r = Ray::new(&Tuple::point(0., -1., 0.), &Tuple::vector(0., 1., 0.));
        let xs = p.local_intersect(&r);
        assert_eq!(1, xs.len());
        assert_eq!(&p, &xs[0].object);
    }
}
