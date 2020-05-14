use super::color::{Color, BLACK, WHITE};
use super::comps::Comps;
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

    pub fn shade_hit(&self, comps: &Comps) -> Color {
        if self.light != None {
            comps.object.material().lighting(&self.light.unwrap(), &comps.point, &comps.eyev, &comps.normalv)
        } else {
            BLACK
        }
    }

    pub fn color_at(&self, r: &Ray) -> Color {
        let i = self.intersect(r);
        let hit = i.hit();
        if hit != None {
            let comps = Comps::prepare_computations(hit.unwrap(), r);
            self.shade_hit(&comps)
        } else {
            BLACK
        }
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

    #[test]
    fn shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));
        let s = w.shapes[0].clone();

        let i = Intersection::new(4., s);

        let comps = Comps::prepare_computations(&i, &r);
        let c = w.shade_hit(&comps);
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), c);
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let light = PointLight::new(&Tuple::point(0., 0.25, 0.), &WHITE);
        let w = World {light: Some(light), ..Default::default() };
        let r = Ray::new(&Tuple::point(0., 0., 0.), &Tuple::vector(0., 0., 1.));
        let s = w.shapes[1].clone();

        let i = Intersection::new(0.5, s);

        let comps = Comps::prepare_computations(&i, &r);
        let c = w.shade_hit(&comps);
        assert_eq!(Color::new(0.90498, 0.90498, 0.90498), c);
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = World::default();
        let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 1., 0.));

        let c = w.color_at(&r);
        assert_eq!(BLACK, c);
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = World::default();
        let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));

        let c = w.color_at(&r);
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), c);
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let mut w = World::default();
        let material = Material {ambient: 1., ..*w.shapes[0].material()};
        w.shapes.first_mut().unwrap().set_material(material);
        w.shapes.last_mut().unwrap().set_material(material);

        let r = Ray::new(&Tuple::point(0., 0., 0.75), &Tuple::vector(0., 0., -1.));
        let c = w.color_at(&r);
        assert_eq!(w.shapes[1].material().color, c);
    }
}
