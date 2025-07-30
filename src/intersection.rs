use super::shape::BoxShape;

use std::cmp::Ordering;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: BoxShape,
}

impl Intersection {
    pub fn new(t: f64, object: BoxShape) -> Intersection {
        Intersection { t, object }
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && &self.object == &other.object
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

/// A sorted list of intersections
#[derive(Debug, Clone)]
pub struct Intersections {
    xs: Vec<Intersection>,
}

impl Intersections {
    pub fn new(intersections: Vec<Intersection>) -> Intersections {
        let mut xs = Intersections { xs: intersections };
        xs.sort();
        xs
    }

    /// Locates the closest visible intersection.
    ///
    /// # Examples
    /// ```
    /// # extern crate the_ray_tracer_challenge_rust as tracer;
    /// # use tracer::intersection::{Intersection, Intersections};
    /// # use tracer::sphere::Sphere;
    /// let s = Sphere::default_boxed();
    /// let i1 = Intersection::new(-2., s.clone());
    /// let i2 = Intersection::new(-1., s);
    /// let xs = Intersections::new(vec!(i1, i2));
    /// let closest = xs.hit();
    /// ```
    pub fn hit(&self) -> Option<&Intersection> {
        match self.xs.iter().find(|i| i.t >= 0.) {
            Some(i) => Some(i),
            _ => None,
        }
    }

    fn sort(&mut self) {
        self.xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }
}

impl Deref for Intersections {
    type Target = Vec<Intersection>;

    fn deref(&self) -> &Vec<Intersection> {
        &self.xs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere::Sphere;

    #[test]
    fn an_itersection_encapsulates_t_and_object() {
        let s = Sphere::default_boxed();
        let t = 3.5;

        let i = Intersection::new(t, s.clone());

        assert_eq!(t, i.t);
        assert_eq!(&s, &i.object);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::default_boxed();

        let t1 = 1.;
        let t2 = 2.;

        let i1 = Intersection::new(t1, s.clone());
        let i2 = Intersection::new(t2, s);

        let xs = vec![i1, i2];
        assert_eq!(2, xs.len());
        assert_eq!(t1, xs[0].t);
        assert_eq!(t2, xs[1].t);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::default_boxed();
        let i1 = Intersection::new(1., s.clone());
        let i2 = Intersection::new(2., s.clone());

        let xs = Intersections::new(vec![i1, i2]);

        assert_eq!(Intersection::new(1., s), *xs.hit().unwrap());
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::default_boxed();
        let i1 = Intersection::new(-1., s.clone());
        let i2 = Intersection::new(1., s.clone());

        let xs = Intersections::new(vec![i1, i2]);

        assert_eq!(Intersection::new(1., s), *xs.hit().unwrap());
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::default_boxed();
        let i1 = Intersection::new(-2., s.clone());
        let i2 = Intersection::new(-1., s);

        let xs = Intersections::new(vec![i1, i2]);

        assert_eq!(None, xs.hit());
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::default_boxed();
        let i1 = Intersection::new(5., s.clone());
        let i2 = Intersection::new(7., s.clone());
        let i3 = Intersection::new(-3., s.clone());
        let i4 = Intersection::new(2., s.clone());

        let xs = Intersections::new(vec![i1, i2, i3, i4]);

        assert_eq!(Intersection::new(2., s), *xs.hit().unwrap());
    }
}
