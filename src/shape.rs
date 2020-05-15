use super::intersection::Intersections;
use super::material::Material;
use super::matrix::Matrix;
use super::ray::Ray;
use super::tuple::Tuple;

use std::any::Any;
use std::fmt;

pub trait Shape: fmt::Debug {
    fn transformation(&self) -> Matrix;
    fn material(&self) -> &Material;
    fn set_material(&mut self, m: Material);

    fn normal_at(&self, &world_point: &Tuple) -> Tuple {
        let local_point = self.transformation().inverse() * world_point;
        let local_normal = self.local_normal_at(&local_point);
        let world_normal = self.transformation().inverse().transpose() * local_normal;
        Tuple::vector(world_normal.x, world_normal.y, world_normal.z).normalize()
    }

    fn intersect(&self, r: Ray) -> Intersections {
        let r = r.transform(self.transformation().inverse());
        self.local_intersect(&r)
    }

    fn box_clone(&self) -> BoxShape;
    fn box_eq(&self, other: &dyn Any) -> bool;
    fn as_any(&self) -> &dyn Any;

    // Non-public API
    fn local_intersect(&self, r: &Ray) -> Intersections;
    fn local_normal_at(&self, local_point: &Tuple) -> Tuple;
}

pub type BoxShape = Box<dyn Shape>;

impl Clone for BoxShape {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}
impl PartialEq for BoxShape {
    fn eq(&self, other: &Self) -> bool {
        self.box_eq(other.as_any())
    }
}
