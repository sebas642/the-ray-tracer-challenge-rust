use super::color::{Color, WHITE};
use super::intersection::{Intersection, Intersections};
use super::light::PointLight;
use super::material::Material;
use super::ray::Ray;
use super::shape::{BoxShape};
use super::sphere::Sphere;
use super::transform;
use super::tuple::Tuple;

#[derive(PartialEq)]
pub struct World {
    pub light: Option<PointLight>,
    pub shapes: Vec<BoxShape>
}

impl World {
    pub fn new(light: Option<PointLight>, shapes: Vec<BoxShape>) -> World {
        World { light, shapes }
    }

    pub fn intersect(&self, &r: &Ray) -> Intersections {
        let mut intersections : Vec<Intersection> = vec![];
        self.shapes.iter().for_each(|shape| shape.intersect(r).iter().for_each(|i| intersections.push(i.clone())));
        Intersections::new(intersections)
    }
}

impl Default for World {
    fn default() -> World {
        let light = PointLight::new(&Tuple::point(-10., 10., -10.), &WHITE);

        let diffuse = 0.7;
        let specular = 0.2;
        let material = Material::new(
            Some(Color::new(0.8, 1., 0.6)), None, Some(diffuse), Some(specular), None
        );
        let s1 = Sphere::new_boxed(None, Some(material));
        let s2 = Sphere::new_boxed(Some(transform::scaling(0.5, 0.5, 0.5)), None);

        World::new(Some(light), vec![s1, s2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_world() {
        let w = World::new(None, vec![]);

        assert_eq!(None, w.light);
    }

    #[test]
    fn the_default_world() {
        let light = PointLight::new(&Tuple::point(-10., 10., -10.), &WHITE);

        let diffuse = 0.7;
        let specular = 0.2;
        let material = Material::new(
            Some(Color::new(0.8, 1., 0.6)), None, Some(diffuse), Some(specular), None
        );
        let s1 = Sphere::new_boxed(None, Some(material));
        let s2 = Sphere::new_boxed(Some(transform::scaling(0.5, 0.5, 0.5)), None);

        let w = World::default();
        assert_eq!(light, w.light.unwrap());
        assert!(w.shapes.contains(&s1));
        assert!(w.shapes.contains(&s2));
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = World::default();
        let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));

        let xs = w.intersect(&r);
        assert_eq!(4, xs.len());
        assert_eq!(4., xs[0].t);
        assert_eq!(4.5, xs[1].t);
        assert_eq!(5.5, xs[2].t);
        assert_eq!(6., xs[3].t);
    }
}
