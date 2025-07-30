use super::color::Color;
use super::matrix::Matrix;
use super::shape::BoxShape;
use super::tuple::Tuple;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StripePattern {
    pub first: Color,
    pub second: Color,
    pub transform: Matrix,
}

pub fn stripe_pattern(first: Color, second: Color, transform: Option<Matrix>) -> StripePattern {
    let s = StripePattern {
        first,
        second,
        transform: transform.unwrap_or_default(),
    };
    s
}

pub fn stripe_at(pattern: &StripePattern, point: &Tuple) -> Color {
    if point.x.floor().abs() as u32 % 2 == 0 {
        pattern.first
    } else {
        pattern.second
    }
}

pub fn stripe_at_object(pattern: &StripePattern, object: &BoxShape, &world_point: &Tuple) -> Color {
    let object_point = object.transformation().inverse() * world_point;
    let pattern_point = pattern.transform.inverse() * object_point;

    stripe_at(pattern, &pattern_point)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{BLACK, WHITE};
    use crate::sphere::Sphere;
    use crate::transform;

    #[test]
    fn creating_a_stripe_pattern() {
        let p = stripe_pattern(WHITE, BLACK, None);
        assert_eq!(p.first, WHITE);
        assert_eq!(p.second, BLACK);
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let p = stripe_pattern(WHITE, BLACK, None);
        assert_eq!(WHITE, stripe_at(&p, &Tuple::point(0., 0., 0.)));
        assert_eq!(WHITE, stripe_at(&p, &Tuple::point(0., 1., 0.)));
        assert_eq!(WHITE, stripe_at(&p, &Tuple::point(0., 2., 0.)));
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let p = stripe_pattern(WHITE, BLACK, None);
        assert_eq!(WHITE, stripe_at(&p, &Tuple::point(0., 0., 0.)));
        assert_eq!(WHITE, stripe_at(&p, &Tuple::point(0., 0., 1.)));
        assert_eq!(WHITE, stripe_at(&p, &Tuple::point(0., 0., 2.)));
    }

    #[test]
    fn a_stripe_pattern_is_alternates_in_x() {
        let p = stripe_pattern(WHITE, BLACK, None);
        assert_eq!(WHITE, stripe_at(&p, &Tuple::point(0., 0., 0.)));
        assert_eq!(WHITE, stripe_at(&p, &Tuple::point(0.9, 0., 0.)));
        assert_eq!(BLACK, stripe_at(&p, &Tuple::point(1., 0., 0.)));
        assert_eq!(BLACK, stripe_at(&p, &Tuple::point(-0.1, 0., 0.)));
        assert_eq!(BLACK, stripe_at(&p, &Tuple::point(-1., 0., 0.)));
        assert_eq!(WHITE, stripe_at(&p, &Tuple::point(-1.1, 0., 0.)));
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let p = stripe_pattern(WHITE, BLACK, None);
        let s = Sphere::new_boxed(Some(transform::scaling(2., 2., 2.)), None);
        assert_eq!(WHITE, stripe_at_object(&p, &s, &Tuple::point(1.5, 0., 0.)));
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let p = stripe_pattern(WHITE, BLACK, Some(transform::scaling(2., 2., 2.)));
        let s = Sphere::new_boxed(None, None);
        assert_eq!(WHITE, stripe_at_object(&p, &s, &Tuple::point(1.5, 0., 0.)));
    }

    #[test]
    fn stripes_with_a_both_an_object_and_a_pattern_transformation() {
        let p = stripe_pattern(WHITE, BLACK, Some(transform::translation(0.5, 0., 0.)));
        let s = Sphere::new_boxed(Some(transform::scaling(2., 2., 2.)), None);
        assert_eq!(WHITE, stripe_at_object(&p, &s, &Tuple::point(1.5, 0., 0.)));
    }
}
