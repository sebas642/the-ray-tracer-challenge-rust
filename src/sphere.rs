use super::intersection::{Intersection, Intersections};
use super::material::Material;
use super::matrix::Matrix;
use super::ray::Ray;
use super::shape::{Shape, BoxShape};
use super::tuple::{Tuple, POINT_ORIGIN};


use std::any::Any;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    origin: Tuple,
    transform: Matrix,
    material: Material
}

impl Sphere {
    pub fn new(transform: Option<Matrix>, material: Option<Material>) -> Self {
        Self {
            origin: POINT_ORIGIN,
            transform: transform.unwrap_or_default(),
            material: material.unwrap_or_default()
        }
    }

    pub fn new_boxed(transform: Option<Matrix>, material: Option<Material>) -> BoxShape {
        Box::new(Sphere::new(transform, material))
    }

    pub fn default_boxed() -> BoxShape {
        Box::new(Sphere::default())
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere::new(None, None)
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

    fn material(&self) -> &Material {
        &self.material
    }

    fn normal_at(&self, &world_point: &Tuple) -> Tuple {
        let object_point = self.transformation().inverse() * world_point;
        let object_normal = object_point - self.origin;
        let world_normal = self.transformation().inverse().transpose() * object_normal;
        Tuple::vector(world_normal.x, world_normal.y, world_point.z).normalize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::WHITE;
    use crate::matrix::MATRIX_IDENTITY;
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
        let s = Sphere::new_boxed(Some(t), None);

        assert_eq!(s.transformation(), t);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));
        let s = Sphere::new_boxed(Some(transform::scaling(2., 2., 2.)), None);

        let xs = s.intersect(r);
        assert_eq!(2, xs.len());
        assert_eq!(3., xs[0].t);
        assert_eq!(7., xs[1].t);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));
        let s = Sphere::new_boxed(Some(transform::translation(5., 0., 0.)), None);

        let xs = s.intersect(r);
        assert_eq!(0, xs.len());
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::default_boxed();
        let p = Tuple::point(1., 0., 0.);

        let n = Tuple::vector(1., 0., 0.);
        assert_eq!(n, s.normal_at(&p));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::default_boxed();
        let p = Tuple::point(0., 1., 0.);

        let n = Tuple::vector(0., 1., 0.);
        assert_eq!(n, s.normal_at(&p));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::default_boxed();
        let p = Tuple::point(0., 0., 1.);

        let n = Tuple::vector(0., 0., 1.);
        assert_eq!(n, s.normal_at(&p));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Sphere::default_boxed();
        let p = Tuple::point(3f64.sqrt() / 3., 3f64.sqrt() / 3., 3f64.sqrt() / 3.);

        let n = Tuple::vector(3f64.sqrt() / 3., 3f64.sqrt() / 3., 3f64.sqrt() / 3.);
        assert_eq!(n, s.normal_at(&p));
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = Sphere::default_boxed();
        let p = Tuple::point(3f64.sqrt() / 3., 3f64.sqrt() / 3., 3f64.sqrt() / 3.);

        let n = s.normal_at(&p);
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn the_normal_on_a_translated_sphere() {
        let t = transform::translation(0., 1., 0.);
        let s = Sphere::new_boxed(Some(t), None);
        let p = Tuple::point(0., 1.70711, -0.70711);

        let n = Tuple::vector(0., 0.70711, -0.70711);
        assert_eq!(n, s.normal_at(&p));
    }

    #[test]
    fn the_normal_on_a_transformed_sphere() {
        let s = transform::scaling(1., 0.5, 1.);
        let r = transform::rotation_z(std::f64::consts::PI / 5.);
        let t = transform::transforms(&[r, s]);
        let s = Sphere::new_boxed(Some(t), None);
        let p = Tuple::point(0., 2f64.sqrt() / 2., -1. * 2f64.sqrt() / 2.);

        let n = Tuple::vector(0., 0.97014, -0.24254);
        assert_eq!(n, s.normal_at(&p));
    }

    #[test]
    fn a_sphere_has_a_default_material() {
        let s = Sphere::default();
        let m = Material::default();

        assert_eq!(m, s.material);
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let m = Material { ambient: 1., ..Default::default() };
        let s = Sphere::new(None, Some(m));

        assert_eq!(m, s.material);
        assert_eq!(1., s.material.ambient);
        assert_eq!(WHITE, s.material.color);
    }
}
