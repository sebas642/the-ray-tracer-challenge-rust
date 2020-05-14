use super::shape::BoxShape;
use super::intersection::Intersection;
use super::ray::Ray;
use super::tuple::Tuple;
use super::utils;

/// Structure used to store helpful computations
pub struct Comps {
    pub t: f64,
    pub object: BoxShape,

    pub point: Tuple,
    pub over_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,

    pub inside: bool
}

impl Comps {
    pub fn prepare_computations(i: &Intersection, r: &Ray) -> Comps {
        let point = r.position(i.t);
        let eyev = -r.direction;
        let mut normalv = i.object.normal_at(&point);

        let inside = if Tuple::dot_product(&normalv, &eyev) < 0. {
            normalv = -normalv;
            true
        } else {
            false
        };

        let over_point = point + (normalv * utils::EPSILON);
        Comps {t: i.t, object: i.object.clone(), point, over_point, eyev, normalv, inside}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transform;
    use crate::sphere::Sphere;

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));
        let s = Sphere::default_boxed();

        let i = Intersection::new(4., s);

        let comps = Comps::prepare_computations(&i, &r);
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

        let comps = Comps::prepare_computations(&i, &r);
        assert_eq!(false, comps.inside);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(&Tuple::point(0., 0., 0.), &Tuple::vector(0., 0., 1.));
        let s = Sphere::default_boxed();

        let i = Intersection::new(1., s);

        let comps = Comps::prepare_computations(&i, &r);
        assert_eq!(true, comps.inside);
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));
        let s = Sphere::new_boxed(Some(transform::translation(0., 0., 1.)), None);

        let i = Intersection::new(5., s);

        let comps = Comps::prepare_computations(&i, &r);
        assert!(comps.over_point.z < -utils::EPSILON / 2.);
        assert!(comps.point.z > comps.over_point.z);
    }
}
