use super::matrix::{MATRIX_IDENTITY, Matrix};
use std::f64;

pub fn transforms(list: &[Matrix]) -> Matrix {
    let mut m = MATRIX_IDENTITY;
    for x in list.iter().rev() {
        m = m * x;
    }
    m
}

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    let mut m = MATRIX_IDENTITY;
    m.set(0, 3, x);
    m.set(1, 3, y);
    m.set(2, 3, z);
    m
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    let mut m = MATRIX_IDENTITY;
    m.set(0, 0, x);
    m.set(1, 1, y);
    m.set(2, 2, z);
    m
}

pub fn rotation_x(r: f64) -> Matrix {
    let mut m = MATRIX_IDENTITY;
    m.set(1, 1, r.cos());
    m.set(1, 2, -r.sin());
    m.set(2, 1, r.sin());
    m.set(2, 2, r.cos());
    m
}

pub fn rotation_y(r: f64) -> Matrix {
    let mut m = MATRIX_IDENTITY;
    m.set(0, 0, r.cos());
    m.set(0, 2, r.sin());
    m.set(2, 0, -r.sin());
    m.set(2, 2, r.cos());
    m
}

pub fn rotation_z(r: f64) -> Matrix {
    let mut m = MATRIX_IDENTITY;
    m.set(0, 0, r.cos());
    m.set(0, 1, -r.sin());
    m.set(1, 0, r.sin());
    m.set(1, 1, r.cos());
    m
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    let mut m = MATRIX_IDENTITY;
    m.set(0, 1, xy);
    m.set(0, 2, xz);
    m.set(1, 0, yx);
    m.set(1, 2, yz);
    m.set(2, 0, zx);
    m.set(2, 1, zy);
    m
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::Tuple;

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let t = translation(5., -3., 2.);
        let p = Tuple::point(-3., 4., 5.);

        let result = Tuple::point(2., 1., 7.);
        assert_eq!(result, t * p);
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let t = translation(5., -3., 2.);
        let inverse = t.inverse();
        let p = Tuple::point(-3., 4., 5.);

        let result = Tuple::point(-8., 7., 3.);
        assert_eq!(result, inverse * p);
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let t = translation(5., -3., 2.);
        let v = Tuple::vector(-3., 4., 5.);

        assert_eq!(v, t * v);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_point() {
        let s = scaling(2., 3., 4.);
        let p = Tuple::point(-4., 6., 8.);

        let result = Tuple::point(-8., 18., 32.);
        assert_eq!(result, s * p);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_vector() {
        let s = scaling(2., 3., 4.);
        let v = Tuple::vector(-4., 6., 8.);

        let result = Tuple::vector(-8., 18., 32.);
        assert_eq!(result, s * v);
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let s = scaling(2., 3., 4.);
        let inv = s.inverse();
        let v = Tuple::vector(-4., 6., 8.);

        let result = Tuple::vector(-2., 2., 2.);
        assert_eq!(result, inv * v);
    }

    #[test]
    fn reflexion_is_scaling_by_a_negative_value() {
        let s = scaling(-1., 1., 1.);
        let p = Tuple::point(2., 3., 4.);

        let result = Tuple::point(-2., 3., 4.);
        assert_eq!(result, s * p);
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let p = Tuple::point(0., 1., 0.);
        let r_half = rotation_x(f64::consts::FRAC_PI_4);
        let r_full = rotation_x(f64::consts::FRAC_PI_2);

        let half_result = Tuple::point(0., 2f64.sqrt() / 2., 2f64.sqrt() / 2.);
        let full_result = Tuple::point(0., 0., 1.);

        assert_eq!(half_result, r_half * p);
        assert_eq!(full_result, r_full * p);
    }

    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = Tuple::point(0., 1., 0.);
        let r_half = rotation_x(f64::consts::FRAC_PI_4);
        let inv = r_half.inverse();

        let result = Tuple::point(0., 2f64.sqrt() / 2., -2f64.sqrt() / 2.);

        assert_eq!(result, inv * p);
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = Tuple::point(0., 0., 1.);
        let r_half = rotation_y(f64::consts::FRAC_PI_4);
        let r_full = rotation_y(f64::consts::FRAC_PI_2);

        let half_result = Tuple::point(2f64.sqrt() / 2., 0., 2f64.sqrt() / 2.);
        let full_result = Tuple::point(1., 0., 0.);

        assert_eq!(half_result, r_half * p);
        assert_eq!(full_result, r_full * p);
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = Tuple::point(0., 1., 0.);
        let r_half = rotation_z(f64::consts::FRAC_PI_4);
        let r_full = rotation_z(f64::consts::FRAC_PI_2);

        let half_result = Tuple::point(-2f64.sqrt() / 2., 2f64.sqrt() / 2., 0.);
        let full_result = Tuple::point(-1., 0., 0.);

        assert_eq!(half_result, r_half * p);
        assert_eq!(full_result, r_full * p);
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_y() {
        let s = shearing(1., 0., 0., 0., 0., 0.);
        let p = Tuple::point(2., 3., 4.);

        let result = Tuple::point(5., 3., 4.);
        assert_eq!(result, s * p);
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_z() {
        let s = shearing(0., 1., 0., 0., 0., 0.);
        let p = Tuple::point(2., 3., 4.);

        let result = Tuple::point(6., 3., 4.);
        assert_eq!(result, s * p);
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_x() {
        let s = shearing(0., 0., 1., 0., 0., 0.);
        let p = Tuple::point(2., 3., 4.);

        let result = Tuple::point(2., 5., 4.);
        assert_eq!(result, s * p);
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_z() {
        let s = shearing(0., 0., 0., 1., 0., 0.);
        let p = Tuple::point(2., 3., 4.);

        let result = Tuple::point(2., 7., 4.);
        assert_eq!(result, s * p);
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_x() {
        let s = shearing(0., 0., 0., 0., 1., 0.);
        let p = Tuple::point(2., 3., 4.);

        let result = Tuple::point(2., 3., 6.);
        assert_eq!(result, s * p);
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_y() {
        let s = shearing(0., 0., 0., 0., 0., 1.);
        let p = Tuple::point(2., 3., 4.);

        let result = Tuple::point(2., 3., 7.);
        assert_eq!(result, s * p);
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = Tuple::point(1., 0., 1.);
        let r = rotation_x(f64::consts::FRAC_PI_2);
        let s = scaling(5., 5., 5.);
        let t = translation(10., 5., 7.);

        let p2 = r * p;
        assert_eq!(Tuple::point(1., -1., 0.), p2);

        let p3 = s * p2;
        assert_eq!(Tuple::point(5., -5., 0.), p3);

        let p4 = t * p3;
        assert_eq!(Tuple::point(15., 0., 7.), p4);
    }

    #[test]
    fn chained_transformations_are_applied_in_reverse_order() {
        let p = Tuple::point(1., 0., 1.);
        let r = rotation_x(f64::consts::FRAC_PI_2);
        let s = scaling(5., 5., 5.);
        let t = translation(10., 5., 7.);

        let transform = t * &s * &r;
        let result = Tuple::point(15., 0., 7.);
        assert_eq!(result, transform * p);

        // Using a transform list, the operations will be automatically reversed
        assert_eq!(result, transforms(&[r, s, t]) * p)
    }
}
