use super::color::Color;
use super::tuple::Tuple;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(&position: &Tuple, &intensity: &Color) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = Color::new(1., 1., 1.);
        let position = Tuple::point(0., 0., 0.);
        let pl = PointLight::new(&position, &intensity);

        assert_eq!(intensity, pl.intensity);
        assert_eq!(position, pl.position);
    }
}
