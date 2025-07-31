use super::color::Color;
use super::matrix::Matrix;
use super::shape::BoxShape;
use super::tuple::Tuple;

#[derive(Debug, Clone, PartialEq)]
pub enum PatternType {
    Checkers(CheckersPattern),
    Gradient(GradientPattern),
    Ring(RingPattern),
    Stripe(StripePattern),
}

impl PatternType {
    pub fn pattern_at(&self, point: &Tuple) -> Color {
        match self {
            PatternType::Checkers(p) => p.pattern_at(point),
            PatternType::Gradient(p) => p.pattern_at(point),
            PatternType::Ring(p) => p.pattern_at(point),
            PatternType::Stripe(p) => p.pattern_at(point),
        }
    }
    pub fn pattern_at_shape(&self, shape: &BoxShape, world_point: &Tuple) -> Color {
        let object_point = shape.transformation().inverse() * *world_point;

        match self {
            PatternType::Checkers(p) => {
                let pattern_point = p.transform.inverse() * object_point;
                p.pattern_at(&pattern_point)
            }
            PatternType::Gradient(p) => {
                let pattern_point = p.transform.inverse() * object_point;
                p.pattern_at(&pattern_point)
            }
            PatternType::Ring(p) => {
                let pattern_point = p.transform.inverse() * object_point;
                p.pattern_at(&pattern_point)
            }
            PatternType::Stripe(p) => {
                let pattern_point = p.transform.inverse() * object_point;
                p.pattern_at(&pattern_point)
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CheckersPattern {
    pub first: Color,
    pub second: Color,
    pub transform: Matrix,
}

impl CheckersPattern {
    pub fn new(first: Color, second: Color, transform: Option<Matrix>) -> Self {
        Self {
            first,
            second,
            transform: transform.unwrap_or_default(),
        }
    }

    pub fn pattern_at(&self, point: &Tuple) -> Color {
        if (point.x.floor().abs() + point.y.floor().abs() + point.z.floor().abs()) as u32 % 2 == 0 {
            self.first
        } else {
            self.second
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GradientPattern {
    pub first: Color,
    pub second: Color,
    pub transform: Matrix,
}

impl GradientPattern {
    pub fn new(first: Color, second: Color, transform: Option<Matrix>) -> Self {
        Self {
            first,
            second,
            transform: transform.unwrap_or_default(),
        }
    }

    pub fn pattern_at(&self, point: &Tuple) -> Color {
        let distance_r = self.second.r - self.first.r;
        let distance_g = self.second.g - self.first.g;
        let distance_b = self.second.b - self.first.b;

        let fraction = point.x - point.x.floor();
        Color {
            r: self.first.r + (distance_r * fraction),
            g: self.first.g + (distance_g * fraction),
            b: self.first.b + (distance_b * fraction),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RingPattern {
    pub first: Color,
    pub second: Color,
    pub transform: Matrix,
}

impl RingPattern {
    pub fn new(first: Color, second: Color, transform: Option<Matrix>) -> Self {
        Self {
            first,
            second,
            transform: transform.unwrap_or_default(),
        }
    }

    pub fn pattern_at(&self, point: &Tuple) -> Color {
        if (point.x.powi(2) + point.z.powi(2)).sqrt().floor().abs() as u32 % 2 == 0 {
            self.first
        } else {
            self.second
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StripePattern {
    pub first: Color,
    pub second: Color,
    pub transform: Matrix,
}

impl StripePattern {
    pub fn new(first: Color, second: Color, transform: Option<Matrix>) -> Self {
        Self {
            first,
            second,
            transform: transform.unwrap_or_default(),
        }
    }

    pub fn pattern_at(&self, point: &Tuple) -> Color {
        if point.x.floor().abs() as u32 % 2 == 0 {
            self.first
        } else {
            self.second
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::{BLACK, WHITE};
    use crate::pattern::PatternType;
    use crate::sphere::Sphere;
    use crate::transform;

    #[test]
    fn create_a_stripe_pattern() {
        let p = StripePattern::new(WHITE, BLACK, None);
        assert_eq!(p.first, WHITE);
        assert_eq!(p.second, BLACK);
        assert_eq!(p.transform, Matrix::default());
    }

    #[test]
    fn create_a_stripe_pattern_with_a_transformation() {
        let p = StripePattern::new(WHITE, BLACK, Some(transform::translation(1., 2., 3.)));
        assert_eq!(p.first, WHITE);
        assert_eq!(p.second, BLACK);
        assert_eq!(p.transform, transform::translation(1., 2., 3.));
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let p = StripePattern::new(WHITE, BLACK, None);
        assert_eq!(WHITE, p.pattern_at(&Tuple::point(0., 0., 0.)));
        assert_eq!(WHITE, p.pattern_at(&Tuple::point(0., 1., 0.)));
        assert_eq!(WHITE, p.pattern_at(&Tuple::point(0., 2., 0.)));
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let p = StripePattern::new(WHITE, BLACK, None);
        assert_eq!(WHITE, p.pattern_at(&Tuple::point(0., 0., 0.)));
        assert_eq!(WHITE, p.pattern_at(&Tuple::point(0., 0., 1.)));
        assert_eq!(WHITE, p.pattern_at(&Tuple::point(0., 0., 2.)));
    }

    #[test]
    fn a_stripe_pattern_is_alternates_in_x() {
        let p = StripePattern::new(WHITE, BLACK, None);
        assert_eq!(WHITE, p.pattern_at(&Tuple::point(0., 0., 0.)));
        assert_eq!(WHITE, p.pattern_at(&Tuple::point(0.9, 0., 0.)));
        assert_eq!(BLACK, p.pattern_at(&Tuple::point(1., 0., 0.)));
        assert_eq!(BLACK, p.pattern_at(&Tuple::point(-0.1, 0., 0.)));
        assert_eq!(BLACK, p.pattern_at(&Tuple::point(-1., 0., 0.)));
        assert_eq!(WHITE, p.pattern_at(&Tuple::point(-1.1, 0., 0.)));
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let p = StripePattern::new(WHITE, BLACK, None);
        let s = Sphere::new_boxed(Some(transform::scaling(2., 2., 2.)), None);
        assert_eq!(
            WHITE,
            PatternType::Stripe(p).pattern_at_shape(&s, &Tuple::point(1.5, 0., 0.))
        );
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let p = StripePattern::new(WHITE, BLACK, Some(transform::scaling(2., 2., 2.)));
        let s = Sphere::new_boxed(None, None);
        assert_eq!(
            WHITE,
            PatternType::Stripe(p).pattern_at_shape(&s, &Tuple::point(1.5, 0., 0.))
        );
    }

    #[test]
    fn stripes_with_a_both_an_object_and_a_pattern_transformation() {
        let p = StripePattern::new(WHITE, BLACK, Some(transform::translation(0.5, 0., 0.)));
        let s = Sphere::new_boxed(Some(transform::scaling(2., 2., 2.)), None);
        assert_eq!(
            WHITE,
            PatternType::Stripe(p).pattern_at_shape(&s, &Tuple::point(1.5, 0., 0.))
        );
    }

    #[test]
    fn gradient_linearly_interpolates_between_colors() {
        let p = GradientPattern::new(WHITE, BLACK, None);
        assert_eq!(WHITE, p.pattern_at(&Tuple::point(0., 0., 0.)));
        assert_eq!(
            Color::new(0.75, 0.75, 0.75),
            p.pattern_at(&Tuple::point(0.25, 0., 0.))
        );
        assert_eq!(
            Color::new(0.5, 0.5, 0.5),
            p.pattern_at(&Tuple::point(0.5, 0., 0.))
        );
        assert_eq!(
            Color::new(0.25, 0.25, 0.25),
            p.pattern_at(&Tuple::point(0.75, 0., 0.))
        );
    }

    #[test]
    fn ring_extend_in_x_and_z() {
        let p = RingPattern::new(WHITE, BLACK, None);
        assert_eq!(WHITE, p.pattern_at(&Tuple::point(0., 0., 0.)));
        assert_eq!(BLACK, p.pattern_at(&Tuple::point(1., 0., 0.)));
        assert_eq!(BLACK, p.pattern_at(&Tuple::point(0., 0., 1.)));
        // 0.708 = just slightly more than √2/2
        assert_eq!(BLACK, p.pattern_at(&Tuple::point(0.708, 0., 0.708)));
    }

    #[test]
    fn checkers_should_repeat_in_x() {
        let p = CheckersPattern::new(WHITE, BLACK, None);
        assert_eq!(WHITE, p.pattern_at(&Tuple::point(0., 0., 0.)));
        assert_eq!(WHITE, p.pattern_at(&Tuple::point(0.99, 0., 0.)));
        assert_eq!(BLACK, p.pattern_at(&Tuple::point(1.01, 0., 0.)));
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let p = CheckersPattern::new(WHITE, BLACK, None);
        assert_eq!(WHITE, p.pattern_at(&Tuple::point(0., 0., 0.)));
        assert_eq!(WHITE, p.pattern_at(&Tuple::point(0., 0.99, 0.)));
        assert_eq!(BLACK, p.pattern_at(&Tuple::point(0., 1.01, 0.)));
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let p = CheckersPattern::new(WHITE, BLACK, None);
        assert_eq!(WHITE, p.pattern_at(&Tuple::point(0., 0., 0.)));
        assert_eq!(WHITE, p.pattern_at(&Tuple::point(0., 0., 0.99)));
        assert_eq!(BLACK, p.pattern_at(&Tuple::point(0., 0., 1.01)));
    }
}
