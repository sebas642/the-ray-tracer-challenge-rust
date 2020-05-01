use super::intersection::Intersections;
use super::matrix::Matrix;
use super::ray::Ray;

use std::any::Any;
use std::fmt;

pub trait Shape: fmt::Debug {
    fn intersect(&self, r: Ray) -> Intersections;
    fn transformation(&self) -> Matrix;

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
