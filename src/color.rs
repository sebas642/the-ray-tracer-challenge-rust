// FIXME: Import utils ?!?
// TODO: Document the public API
// FIXME: Awful lot of code duplication with the Token struct
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl Color {
    pub fn color(r: f64, g: f64, b: f64) -> Color {
        Color {r, g, b}
    }    
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        super::utils::approx_eq(self.r, other.r) &&
        super::utils::approx_eq(self.g, other.g) &&
        super::utils::approx_eq(self.b, other.b)
    }
}

impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b
        }
    }
}

impl std::ops::Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b
        }
    }
}


impl std::ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, factor: f64) -> Self {
        Self { 
            r: self.r * factor,
            g: self.g * factor,
            b: self.b * factor,
        }
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Self;

    fn mul(self, other: Color) -> Self {
        // This is called the Hadamard product (or Schur product)
        Self { 
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_rgb_tuples()
    {
        let c = Color::color(-0.1, 0.2, 1.3);

        assert_eq!(c.r, -0.1);
        assert_eq!(c.g, 0.2);
        assert_eq!(c.b, 1.3);
    }

    #[test]
    fn adding_colors()
    {
        let c1 = Color::color(0.9, 0.6, 0.75);
        let c2 = Color::color(0.7, 0.1, 0.25);

        assert_eq!(c1 + c2, Color::color(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors()
    {
        let c1 = Color::color(0.9, 0.6, 0.75);
        let c2 = Color::color(0.7, 0.1, 0.25);

        assert_eq!(c1 - c2, Color::color(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplying_a_color_by_a_scalar()
    {
        let c = Color::color(0.2, 0.3, 0.4);

        assert_eq!(c * 2.0, Color::color(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiplying_colors()
    {
        let c1 = Color::color(1.0, 0.2, 0.4);
        let c2 = Color::color(0.9, 1.0, 0.1);

        assert_eq!(c1 * c2, Color::color(0.9, 0.2, 0.04));
    }
}