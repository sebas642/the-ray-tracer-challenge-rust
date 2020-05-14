use super::intersection::Intersections;
use super::material::Material;
use super::matrix::Matrix;
use super::ray::Ray;
use super::tuple::Tuple;

use std::any::Any;
use std::fmt;

pub trait Shape: fmt::Debug {
    fn intersect(&self, r: Ray) -> Intersections;
    fn transformation(&self) -> Matrix;
    fn normal_at(&self, point: &Tuple) -> Tuple;
    fn material(&self) -> &Material;
    fn set_material(&mut self, m: Material);

    fn box_clone(&self) -> BoxShape;
    fn box_eq(&self, other: &dyn Any) -> bool;
    fn as_any(&self) -> &dyn Any;
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
