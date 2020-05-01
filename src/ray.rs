use super::tuple::Tuple;
use super::matrix::Matrix;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple
}

impl Ray {
    pub fn new(origin: &Tuple, direction: &Tuple) -> Ray {
        Ray {origin: *origin, direction: *direction}
    }

    pub fn position(&self, t: f64) -> Tuple {
        self.origin + (self.direction * t)
    }

    pub fn transform(&self, m: Matrix) -> Ray {
        Ray::new(&(m * self.origin), &(m * self.direction))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transform;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = Tuple::point(1., 2., 3.);
        let direction = Tuple::vector(4., 5., 6.);
        let ray = Ray::new(&origin, &direction);

        assert_eq!(origin, ray.origin);
        assert_eq!(direction, ray.direction);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = Ray::new(&Tuple::point(2., 3., 4.), &Tuple::vector(1., 0., 0.));

        assert_eq!(Tuple::point(2., 3., 4.), r.position(0.));
        assert_eq!(Tuple::point(3., 3., 4.), r.position(1.));
        assert_eq!(Tuple::point(1., 3., 4.), r.position(-1.));
        assert_eq!(Tuple::point(4.5, 3., 4.), r.position(2.5));
    }

    #[test]
    fn translating_a_ray() {
        let origin = Tuple::point(1., 2., 3.);
        let direction = Tuple::vector(0., 1., 0.);
        let ray = Ray::new(&origin, &direction);

        let m = transform::translation(3., 4., 5.);
        let ray2 = ray.transform(m);

        assert_eq!(Tuple::point(4., 6., 8.), ray2.origin);
        assert_eq!(Tuple::vector(0., 1., 0.), ray2.direction);
    }

    #[test]
    fn scaling_a_ray() {
        let origin = Tuple::point(1., 2., 3.);
        let direction = Tuple::vector(0., 1., 0.);
        let ray = Ray::new(&origin, &direction);

        let m = transform::scaling(2., 3., 4.);
        let ray2 = ray.transform(m);

        assert_eq!(Tuple::point(2., 6., 12.), ray2.origin);
        assert_eq!(Tuple::vector(0., 3., 0.), ray2.direction);
    }
}
