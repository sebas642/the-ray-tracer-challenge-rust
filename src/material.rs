use super::color::{Color, BLACK, WHITE};
use super::tuple::Tuple;
use super::light::PointLight;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64
}

impl Material {
    pub fn new(color: Option<Color>, ambient: Option<f64>, diffuse: Option<f64>, specular: Option<f64>, shininess: Option<f64>) -> Material {
        Material {
            color: color.unwrap_or(WHITE),
            ambient: ambient.unwrap_or(0.1),
            diffuse: diffuse.unwrap_or(0.9),
            specular: specular.unwrap_or(0.9),
            shininess: shininess.unwrap_or(200.)
        }
    }

    pub fn lighting(&self, &light: &PointLight, &point: &Tuple, &eyev: &Tuple, &normalv: &Tuple) -> Color {
        let effective_color = self.color * light.intensity;
        let lightv = (light.position - point).normalize();

        let ambient = effective_color * self.ambient;

        // light_dot_normal represents the cosine of the angle between
        // the light vector and the normal vector.
        // A negative number means the light is on the other side of the surface.
        let light_dot_normal = Tuple::dot_product(&lightv, &normalv);

        let (diffuse, specular) = if light_dot_normal < 0. {
            (BLACK, BLACK)
        } else {
            let diffuse = effective_color * self.diffuse * light_dot_normal;

            // reflect_dot_eye represents the cosing of the angle between
            // the reflection vector and the eye vector.
            // A negative number means the light reflects away from the eye.
            let reflectv = Tuple::reflect(&(-lightv), &normalv);
            let reflect_dot_eye = Tuple::dot_product(&reflectv, &eyev);

            let specular = if reflect_dot_eye < 0. {
                BLACK
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                light.intensity * self.specular * factor
            };

            (diffuse, specular)
        };

        ambient + diffuse + specular

    }
}

impl Default for Material {
    fn default() -> Self {
        Material::new(None, None, None, None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_material() {
        let c = WHITE;
        let m = Material::default();

        assert_eq!(c, m.color);
        assert_eq!(0.1, m.ambient);
        assert_eq!(0.9, m.diffuse);
        assert_eq!(0.9, m.specular);
        assert_eq!(200., m.shininess);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., 0., -1.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(&Tuple::point(0., 0., -10.), &WHITE);

        let result = Color::new(1.9, 1.9, 1.9);
        assert_eq!(result, m.lighting(&light, &position, &eyev, &normalv));
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface_eye_offset_45_degrees() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., 2f64.sqrt()/2., -2f64.sqrt()/2.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(&Tuple::point(0., 0., -10.), &WHITE);

        let result = WHITE;
        assert_eq!(result, m.lighting(&light, &position, &eyev, &normalv));
    }

    #[test]
    fn lighting_with_the_eye_opposite_the_surface_light_offset_45_degrees() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., 0., -1.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(&Tuple::point(0., 10., -10.), &WHITE);

        let result = Color::new(0.7364, 0.7364, 0.7364);
        assert_eq!(result, m.lighting(&light, &position, &eyev, &normalv));
    }

    #[test]
    fn lighting_with_the_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., -2f64.sqrt()/2., -2f64.sqrt()/2.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(&Tuple::point(0., 10., -10.), &WHITE);

        let result = Color::new(1.6364, 1.6364, 1.6364);
        assert_eq!(result, m.lighting(&light, &position, &eyev, &normalv));
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., 0., -1.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(&Tuple::point(0., 0., 10.), &WHITE);

        let result = Color::new(0.1, 0.1, 0.1);
        assert_eq!(result, m.lighting(&light, &position, &eyev, &normalv));
    }
}
