use super::tuple::*;
// FIXME: Document public API

const MAX_MATRIX_SIZE: usize = 4*4;
pub const MATRIX_IDENTITY: Matrix = Matrix {rows: 4, columns: 4, data:
    [1., 0., 0., 0.,
    0., 1., 0., 0.,
    0., 0., 1., 0.,
    0., 0., 0., 1.]};

#[derive(Debug, Copy, Clone)]
pub struct Matrix {
    pub rows: usize,
    pub columns: usize,
    pub data: [f64; MAX_MATRIX_SIZE]
}

impl Matrix {
    pub fn new(rows: usize, columns: usize) -> Matrix {
        Matrix{rows, columns, data: [0.; MAX_MATRIX_SIZE]}
    }

    pub fn new_with_values(rows: usize, columns: usize, values: &[f64]) -> Matrix {
        let mut m = Matrix{rows, columns, data: [0.; MAX_MATRIX_SIZE]};

        // Copy matrix values (there must be a cleaner way to do this!?)
        for i in 0..MAX_MATRIX_SIZE {
            let val = values.get(i);
            match val {
             Some(v) => m.data[i] = *v,
             None => break
            }
        }
        m
    }

    pub fn get(&self, row: usize, column: usize) -> f64 {
        // No bounds checking is done
        self.data[(row * self.columns) + column]
    }

    pub fn set(&mut self, row: usize, column: usize, value: f64) {
        // No bounds checking is done
        self.data[(row * self.columns) + column] = value;
    }

    pub fn transpose(&self) -> Matrix {
        let mut transposed = Matrix::new(self.columns, self.rows);
        for row in 0..self.rows {
            for column in 0..self.columns {
                transposed.set(column, row, self.get(row, column));
            }
        }

        transposed
    }

    pub fn determinant(&self) -> f64 {
        if self.rows == 2 && self.columns == 2 {
            return self.get(0,0) * self.get(1,1) - self.get(1,0) * self.get(0,1);
        }

        let mut determinant = 0.;
        for x in 0..self.columns {
            determinant += self.get(0, x) * self.cofactor(0, x);
        }
        determinant
    }

    pub fn submatrix(&self, row: usize, column: usize) -> Matrix {
        let mut submatrix = Matrix::new(self.rows-1, self.columns-1);
        let mut dst_row = 0;

        for src_row in 0..self.rows {
            let mut dst_column = 0;
            if src_row == row { continue; }
            for src_column in 0..self.columns {
                if src_column == column { continue; }
                submatrix.set(dst_row, dst_column, self.get(src_row, src_column));
                dst_column += 1;
            }
            dst_row += 1;
        }

        submatrix
    }

    pub fn minor(&self, row: usize, column: usize) -> f64 {
        self.submatrix(row, column).determinant()
    }

    pub fn cofactor(&self, row: usize, column: usize) -> f64 {
        let minor = self.minor(row, column);
        if (row + column) % 2 == 0 { return minor; }
        minor * -1.
    }

    pub fn is_invertible(&self) -> bool {
        !super::utils::approx_eq(self.determinant(), 0.)
    }

    pub fn inverse(&self) -> Matrix {
        if !self.is_invertible() { panic!("Matrix is invertible!"); }

        let determinant = self.determinant();
        let mut m = Matrix::new(self.rows, self.columns);
        for row in 0..self.rows {
            for column in 0..self.columns {
                let c = self.cofactor(row, column);
                m.set(column, row, c / determinant);
            }
        }
        m
    }
}

impl Default for Matrix {
    fn default() -> Self {
        MATRIX_IDENTITY
    }
}

impl std::ops::Mul<&Matrix> for Matrix {
    type Output = Self;

    fn mul(self, m2: &Matrix) -> Self {
        assert_eq!(self.rows, m2.columns);
        let mut mult = Matrix::new(self.rows, self.columns);
        for x in 0..self.rows {
            for y in 0..self.columns {
                let mut dot_product = 0.;
                for pos in 0..self.rows {
                    dot_product += self.get(x, pos) * m2.get(pos, y);
                }
                mult.set(x, y, dot_product);
            }
        }
        mult
    }
}

impl std::ops::Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, tuple: Tuple) -> Tuple {
        assert_eq!(self.columns, 4);

        Tuple {
            x: self.get(0, 0) * tuple.x + self.get(0, 1) * tuple.y + self.get(0, 2) * tuple.z + self.get(0, 3) * tuple.w,
            y: self.get(1, 0) * tuple.x + self.get(1, 1) * tuple.y + self.get(1, 2) * tuple.z + self.get(1, 3) * tuple.w,
            z: self.get(2, 0) * tuple.x + self.get(2, 1) * tuple.y + self.get(2, 2) * tuple.z + self.get(2, 3) * tuple.w,
            w: self.get(3, 0) * tuple.x + self.get(3, 1) * tuple.y + self.get(3, 2) * tuple.z + self.get(3, 3) * tuple.w,
        }
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.rows != other.rows || self.columns != other.columns {
            return false;
        }

        for i in 0..self.rows * self.columns {
            if !super::utils::approx_eq(self.data[i], other.data[i]) {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let values = [
            1., 2., 3., 4.,
            5.5, 6.5, 7.5, 8.5,
            9., 10., 11., 12.,
            13.5, 14.5, 15.5, 16.5];
        let m = Matrix::new_with_values(4, 4, &values);
        println!("{:?}", m);

        assert_eq!(m.get(0, 0), 1.);
        assert_eq!(m.get(0, 3), 4.);
        assert_eq!(m.get(1, 0), 5.5);
        assert_eq!(m.get(1, 2), 7.5);
        assert_eq!(m.get(2, 2), 11.);
        assert_eq!(m.get(3, 0), 13.5);
        assert_eq!(m.get(3, 2), 15.5);
    }

    #[test]
    fn constructing_and_inspecting_a_3x3_matrix() {
        let values = [
            -3., 5., 0.,
            1., -2., -7.,
            0., 1., 1.];
        let m = Matrix::new_with_values(3, 3, &values);
        println!("{:?}", m);

        assert_eq!(m.get(0, 0), -3.);
        assert_eq!(m.get(1, 1), -2.);
        assert_eq!(m.get(2, 2), 1.);
    }

    #[test]
    fn constructing_and_inspecting_a_2x2_matrix() {
        let values = [
            -3., 5.,
            1., -2.];
        let m = Matrix::new_with_values(2, 2, &values);
        println!("{:?}", m);

        assert_eq!(m.get(0, 0), -3.);
        assert_eq!(m.get(0, 1), 5.);
        assert_eq!(m.get(1, 0), 1.);
        assert_eq!(m.get(1, 1), -2.);
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
        let values = [
            1., 2., 3., 4.,
            5., 6., 7., 8.,
            9., 8., 7., 6.,
            5., 4., 3., 2.];
        let m1 = Matrix::new_with_values(4, 4, &values);
        let m2 = Matrix::new_with_values(4, 4, &values);

        assert_eq!(m1, m2);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let values = [
            1., 2., 3., 4.,
            5., 6., 7., 8.,
            9., 8., 7., 6.,
            5., 4., 3., 2.];
        let m1 = Matrix::new_with_values(4, 4, &values);
        let mut m2 = Matrix::new_with_values(4, 4, &values);
        m2.set(2, 3, 12.);

        assert_ne!(m1, m2);
    }

    #[test]
    fn multiplying_two_matrices() {
        let m1_values = [
            1., 2., 3., 4.,
            5., 6., 7., 8.,
            9., 8., 7., 6.,
            5., 4., 3., 2.];
        let m1 = Matrix::new_with_values(4, 4, &m1_values);

        let m2_values = [
            -2., 1., 2., 3.,
            3., 2., 1., -1.,
            4., 3., 6., 5.,
            1., 2., 7., 8.];
        let m2 = Matrix::new_with_values(4, 4, &m2_values);

        let mult_values = [
            20., 22., 50., 48.,
            44., 54., 114., 108.,
            40., 58., 110., 102.,
            16., 26., 46., 42.];

        assert_eq!(Matrix::new_with_values(4, 4, &mult_values), m1 * &m2);
    }

    #[test]
    fn matrix_multiplied_by_a_tuple() {
        let values = [
            1., 2., 3., 4.,
            2., 4., 4., 2.,
            8., 6., 4., 1.,
            0., 0., 0., 1.];
        let m1 = Matrix::new_with_values(4, 4, &values);

        let tuple = Tuple {x:1., y:2., z:3., w:1.};

        assert_eq!(Tuple {x:18., y:24., z:33., w:1.}, m1 * tuple);
    }

    #[test]
    fn multiplying_a_matrix_by_the_identity_matrix() {
        let values = [
            0., 1., 2., 4.,
            1., 2., 4., 8.,
            2., 4., 8., 16.,
            4., 8., 16., 32.];
        let m1 = Matrix::new_with_values(4, 4, &values);
        let m2 = Matrix::new_with_values(4, 4, &values);

        assert_eq!(m2, m1 * &MATRIX_IDENTITY);
    }

    #[test]
    fn transposing_a_matrix() {
        let source = [
            0., 9., 3., 0.,
            9., 8., 0., 8.,
            1., 8., 5., 3.,
            0., 0., 5., 8.];
        let m1 = Matrix::new_with_values(4, 4, &source);

        let transposed = [
            0., 9., 1., 0.,
            9., 8., 8., 0.,
            3., 0., 5., 5.,
            0., 8., 3., 8.];
        let m2 = Matrix::new_with_values(4, 4, &transposed);

        assert_eq!(m2, m1.transpose());
    }

    #[test]
    fn transposing_the_identity_matrix() {
        let m1 = MATRIX_IDENTITY;
        assert_eq!(MATRIX_IDENTITY, m1.transpose());
    }

    #[test]
    fn calculating_the_determinant_of_a_2x2_matrix() {
        let values = [
            1., 5.,
            -3., 2.];
        let m = Matrix::new_with_values(2, 2, &values);

        assert_eq!(17., m.determinant());
    }

    #[test]
    fn a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
        let values = [
            1., 5., 0.,
            -3., 2., 7.,
            0., 6., -3.];
        let m = Matrix::new_with_values(3, 3, &values);

        let sub_values = [
            -3., 2.,
            0., 6.];
        let sub_m = Matrix::new_with_values(2, 2, &sub_values);

        assert_eq!(sub_m, m.submatrix(0, 2));
    }

    #[test]
    fn a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
        let values = [
            -6., 1., 1., 6.,
            -8., 5., 8., 6.,
            -1., 0., 8., 2.,
            -7., 1., -1., 1.];
        let m = Matrix::new_with_values(4, 4, &values);

        let sub_values = [
            -6., 1., 6.,
            -8., 8., 6.,
            -7., -1., 1.];
        let sub_m = Matrix::new_with_values(3, 3, &sub_values);

        assert_eq!(sub_m, m.submatrix(2, 1));
    }

    #[test]
    fn calculating_a_minor_of_a_3x3_matrix() {
        let values = [
            3., 5., 0.,
            2., -1., -7.,
            6., -1., 5.];
        let m = Matrix::new_with_values(3, 3, &values);

        let sub_matrix = m.submatrix(1, 0);
        assert_eq!(25., sub_matrix.determinant());
        assert_eq!(25., m.minor(1,0));
    }

    #[test]
    fn calculating_a_cofactor_of_a_3x3_matrix() {
        let values = [
            3., 5., 0.,
            2., -1., -7.,
            6., -1., 5.];
        let m = Matrix::new_with_values(3, 3, &values);

        assert_eq!(-12., m.minor(0,0));
        assert_eq!(-12., m.cofactor(0,0));

        assert_eq!(25., m.minor(1,0));
        assert_eq!(-25., m.cofactor(1,0));
    }

    #[test]
    fn calculating_the_determinant_of_a_3x3_matrix() {
        let values = [
            1., 2., 6.,
            -5., 8., -4.,
            2., 6., 4.];
        let m = Matrix::new_with_values(3, 3, &values);

        assert_eq!(56., m.cofactor(0,0));
        assert_eq!(12., m.cofactor(0,1));
        assert_eq!(-46., m.cofactor(0,2));
        assert_eq!(-196., m.determinant());
    }

    #[test]
    fn calculating_the_determinant_of_a_4x4_matrix() {
        let values = [
            -2., -8., 3., 5.,
            -3., 1., 7., 3.,
            1., 2., -9., 6.,
            -6., 7., 7., -9.];
        let m = Matrix::new_with_values(4, 4, &values);

        assert_eq!(690., m.cofactor(0,0));
        assert_eq!(447., m.cofactor(0,1));
        assert_eq!(210., m.cofactor(0,2));
        assert_eq!(51., m.cofactor(0,3));
        assert_eq!(-4071., m.determinant());
    }

    #[test]
    fn testing_an_invertible_matrix_for_invertibility() {
        let values = [
            6., 4., 4., 4.,
            5., 5., 7., 6.,
            4., -9., 3., -7.,
            9., 1., 7., -6.];
        let m = Matrix::new_with_values(4, 4, &values);

        assert_eq!(-2120., m.determinant());
        assert_eq!(true, m.is_invertible());
    }

    #[test]
    fn testing_a_noninvertible_matrix_for_invertibility() {
        let values = [
            -4., 2., -2., 3.,
            9., 6., 2., 6.,
            0., -5., 1., -5.,
            0., 0., 0., 0.];
        let m = Matrix::new_with_values(4, 4, &values);

        assert_eq!(0., m.determinant());
        assert_eq!(false, m.is_invertible());
    }

    #[test]
    fn calculating_the_inverse_of_a_matrix() {
        let values = [
            -5., 2., 6., -8.,
            1., -5., 1., 8.,
            7., 7., -6., -7.,
            1., -3., 7., 4.];
        let m = Matrix::new_with_values(4, 4, &values);

        assert_eq!(true, m.is_invertible());
        let inverse = m.inverse();

        assert_eq!(532., m.determinant());
        assert_eq!(-160., m.cofactor(2, 3));
        assert_eq!(-160./532., inverse.get(3,2));
        assert_eq!(105., m.cofactor(3, 2));
        assert_eq!(105./532., inverse.get(2,3));

        let values_inverse = [
            0.21805, 0.45113, 0.24060, -0.04511,
            -0.80827, -1.45677, -0.44361, 0.52068,
            -0.07895, -0.22368, -0.05263, 0.19737,
            -0.52256, -0.81391, -0.30075, 0.30639];
        let m_inverse= Matrix::new_with_values(4, 4, &values_inverse);
        assert_eq!(m_inverse, inverse);
    }

    #[test]
    fn calculating_the_inverse_of_another_matrix() {
        let values = [
            8., -5., 9., 2.,
            7., 5., 6., 1.,
            -6., 0., 9., 6.,
            -3., 0., -9., -4.];
        let m = Matrix::new_with_values(4, 4, &values);

        let values_inverse = [
            -0.15385, -0.15385, -0.28205, -0.53846,
            -0.07692, 0.12308, 0.02564, 0.03077,
            0.35897, 0.35897, 0.43590, 0.92308,
            -0.69231, -0.69231, -0.76923, -1.92308];
        let m_inverse= Matrix::new_with_values(4, 4, &values_inverse);

        assert_eq!(m_inverse, m.inverse());
    }

    #[test]
    fn calculating_the_inverse_of_a_third_matrix() {
        let values = [
            9., 3., 0., 9.,
            -5., -2., -6., -3.,
            -4., 9., 6., 4.,
            -7., 6., 6., 2.];
        let m = Matrix::new_with_values(4, 4, &values);

        let values_inverse = [
            -0.04074, -0.07778, 0.14444, -0.22222,
            -0.07778, 0.03333, 0.36667, -0.33333,
            -0.02901, -0.14630, -0.10926, 0.12963,
            0.17778, 0.06667, -0.26667, 0.33333];
        let m_inverse= Matrix::new_with_values(4, 4, &values_inverse);

        assert_eq!(m_inverse, m.inverse());
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let values1 = [
            3., -9., 7., 3.,
            3., -8., 2., -9.,
            -4., 4., 4., 1.,
            -6., 5., -1., 1.];
        let m1 = Matrix::new_with_values(4, 4, &values1);

        let values2 = [
            8., 2., 2., 2.,
            3., -1., 7., 0.,
            7., 0., 5., 4.,
            6., -2., 0., 5.];
        let m2 = Matrix::new_with_values(4, 4, &values2);

        let m_product = m1 * &m2;
        assert_eq!(m1, m_product * &m2.inverse());
    }
}
