use super::color::{BLACK, Color, WHITE};
use super::light::PointLight;
use super::pattern::PatternType;
use super::shape::BoxShape;
use super::tuple::Tuple;

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub pattern: Option<PatternType>,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pub transparency: f64,
    pub refractive_index: f64,
}

impl Material {
    pub fn new(
        color: Option<Color>,
        pattern: Option<PatternType>,
        ambient: Option<f64>,
        diffuse: Option<f64>,
        specular: Option<f64>,
        shininess: Option<f64>,
        reflective: Option<f64>,
        transparency: Option<f64>,
        refractive_index: Option<f64>,
    ) -> Material {
        Material {
            color: color.unwrap_or(WHITE),
            pattern,
            ambient: ambient.unwrap_or(0.1),
            diffuse: diffuse.unwrap_or(0.9),
            specular: specular.unwrap_or(0.9),
            shininess: shininess.unwrap_or(200.),
            reflective: reflective.unwrap_or(0.),
            transparency: transparency.unwrap_or(0.),
            refractive_index: refractive_index.unwrap_or(1.0),
        }
    }

    pub fn lighting(
        &self,
        object: &BoxShape,
        &light: &PointLight,
        &point: &Tuple,
        &eyev: &Tuple,
        &normalv: &Tuple,
        in_shadow: bool,
    ) -> Color {
        let color = match &self.pattern {
            Some(p) => p.pattern_at_shape(object, &point),
            _ => self.color,
        };
        let effective_color = color * light.intensity;
        let lightv = (light.position - point).normalize();

        let ambient = effective_color * self.ambient;

        // light_dot_normal represents the cosine of the angle between
        // the light vector and the normal vector.
        // A negative number means the light is on the other side of the surface.
        let light_dot_normal = Tuple::dot_product(&lightv, &normalv);

        let (diffuse, specular) = if light_dot_normal < 0. || in_shadow {
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
        Material::new(None, None, None, None, None, None, None, None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        comps::Comps,
        intersection::{Intersection, Intersections},
        matrix::{MATRIX_IDENTITY, Matrix},
        pattern,
        ray::Ray,
        sphere::Sphere,
        utils::EPSILON,
    };

    fn glass_sphere(transform: Option<Matrix>, refractive_index: Option<f64>) -> BoxShape {
        Sphere::new_boxed(
            transform.or(Some(MATRIX_IDENTITY)),
            Some(Material::new(
                Some(Color::new(1., 0.2, 1.)),
                None,
                None,
                None,
                None,
                None,
                Some(0.9),
                Some(1.0),
                refractive_index.or(Some(1.5)),
            )),
        )
    }

    #[test]
    fn the_default_material() {
        let c = WHITE;
        let m = Material::default();

        assert_eq!(c, m.color);
        assert_eq!(None, m.pattern);
        assert_eq!(0.1, m.ambient);
        assert_eq!(0.9, m.diffuse);
        assert_eq!(0.9, m.specular);
        assert_eq!(200., m.shininess);
        assert_eq!(0., m.reflective);
        assert_eq!(0., m.transparency);
        assert_eq!(1., m.refractive_index);
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., 0., -1.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(&Tuple::point(0., 0., -10.), &WHITE);

        let result = Color::new(1.9, 1.9, 1.9);
        assert_eq!(
            result,
            m.lighting(
                &Sphere::default_boxed(),
                &light,
                &position,
                &eyev,
                &normalv,
                false
            )
        );
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface_eye_offset_45_degrees() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., 2f64.sqrt() / 2., -2f64.sqrt() / 2.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(&Tuple::point(0., 0., -10.), &WHITE);

        let result = WHITE;
        assert_eq!(
            result,
            m.lighting(
                &Sphere::default_boxed(),
                &light,
                &position,
                &eyev,
                &normalv,
                false
            )
        );
    }

    #[test]
    fn lighting_with_the_eye_opposite_the_surface_light_offset_45_degrees() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., 0., -1.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(&Tuple::point(0., 10., -10.), &WHITE);

        let result = Color::new(0.7364, 0.7364, 0.7364);
        assert_eq!(
            result,
            m.lighting(
                &Sphere::default_boxed(),
                &light,
                &position,
                &eyev,
                &normalv,
                false
            )
        );
    }

    #[test]
    fn lighting_with_the_eye_in_the_path_of_the_reflection_vector() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., -2f64.sqrt() / 2., -2f64.sqrt() / 2.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(&Tuple::point(0., 10., -10.), &WHITE);

        let result = Color::new(1.6364, 1.6364, 1.6364);
        assert_eq!(
            result,
            m.lighting(
                &Sphere::default_boxed(),
                &light,
                &position,
                &eyev,
                &normalv,
                false
            )
        );
    }

    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., 0., -1.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(&Tuple::point(0., 0., 10.), &WHITE);
        let in_shadow = false;

        let result = Color::new(0.1, 0.1, 0.1);
        assert_eq!(
            result,
            m.lighting(
                &Sphere::default_boxed(),
                &light,
                &position,
                &eyev,
                &normalv,
                in_shadow
            )
        );
    }

    #[test]
    fn lighting_with_the_surface_in_shadow() {
        let m = Material::default();
        let position = Tuple::point(0., 0., 0.);

        let eyev = Tuple::vector(0., 0., -1.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(&Tuple::point(0., 0., -10.), &WHITE);
        let in_shadow = true;

        let result = Color::new(0.1, 0.1, 0.1);
        assert_eq!(
            result,
            m.lighting(
                &Sphere::default_boxed(),
                &light,
                &position,
                &eyev,
                &normalv,
                in_shadow
            )
        );
    }

    #[test]
    fn lighting_with_a_pattern_applied() {
        let mut m = Material::default();
        m.pattern = Some(PatternType::Stripe(pattern::StripePattern {
            first: WHITE,
            second: BLACK,
            transform: Matrix::default(),
        }));
        m.ambient = 1.;
        m.diffuse = 0.;
        m.specular = 0.;

        let eyev = Tuple::vector(0., 0., -1.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(&Tuple::point(0., 0., -10.), &WHITE);
        let in_shadow = true;

        let s = Sphere::default_boxed();
        assert_eq!(
            WHITE,
            m.lighting(
                &s,
                &light,
                &Tuple::point(0.9, 0., 0.),
                &eyev,
                &normalv,
                in_shadow
            )
        );
        assert_eq!(
            BLACK,
            m.lighting(
                &s,
                &light,
                &Tuple::point(1.1, 0., 0.),
                &eyev,
                &normalv,
                in_shadow
            )
        );
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let s1 = glass_sphere(Some(crate::transform::scaling(2., 2., 2.)), Some(1.5));
        let s2 = glass_sphere(
            Some(crate::transform::translation(0., 0., -0.25)),
            Some(2.0),
        );
        let s3 = glass_sphere(Some(crate::transform::translation(0., 0., 0.25)), Some(2.5));

        let r = Ray::new(&Tuple::point(0., 0., -4.), &Tuple::vector(0., 0., 1.));

        let i0 = Intersection::new(2., s1.clone());
        let i1 = Intersection::new(2.75, s2.clone());
        let i2 = Intersection::new(3.25, s3.clone());
        let i3 = Intersection::new(4.75, s2.clone());
        let i4 = Intersection::new(5.25, s3.clone());
        let i5 = Intersection::new(6., s1.clone());

        let xs = Intersections::new(vec![
            i0.clone(),
            i1.clone(),
            i2.clone(),
            i3.clone(),
            i4.clone(),
            i5.clone(),
        ]);

        let comps = Comps::prepare_computations(&i0, &r, Some(xs.clone()));
        assert_eq!(comps.n1, 1.0);
        assert_eq!(comps.n2, 1.5);

        let comps = Comps::prepare_computations(&i1, &r, Some(xs.clone()));
        assert_eq!(comps.n1, 1.5);
        assert_eq!(comps.n2, 2.);

        let comps = Comps::prepare_computations(&i2, &r, Some(xs.clone()));
        assert_eq!(comps.n1, 2.0);
        assert_eq!(comps.n2, 2.5);

        let comps = Comps::prepare_computations(&i3, &r, Some(xs.clone()));
        assert_eq!(comps.n1, 2.5);
        assert_eq!(comps.n2, 2.5);

        let comps = Comps::prepare_computations(&i4, &r, Some(xs.clone()));
        assert_eq!(comps.n1, 2.5);
        assert_eq!(comps.n2, 1.5);

        let comps = Comps::prepare_computations(&i5, &r, Some(xs.clone()));
        assert_eq!(comps.n1, 1.5);
        assert_eq!(comps.n2, 1.0);
    }

    #[test]
    fn the_under_point_is_offset_below_the_surface() {
        let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));
        let s = glass_sphere(Some(crate::transform::translation(0., 0., 1.)), None);

        let i = Intersection::new(5., s);
        let xs = Intersections::new(vec![i.clone()]);
        let comps = Comps::prepare_computations(&i, &r, Some(xs));

        assert!(comps.under_point.z > EPSILON / 2.);
        assert!(comps.point.z < comps.under_point.z);
    }
}
