use super::color::{BLACK, Color, WHITE};
use super::comps::Comps;
use super::intersection::{Intersection, Intersections};
use super::light::PointLight;
use super::material::Material;
use super::ray::Ray;
use super::shape::BoxShape;
use super::sphere::Sphere;
use super::transform;
use super::tuple::Tuple;

#[derive(PartialEq)]
pub struct World {
    pub light: Option<PointLight>,
    pub shapes: Vec<BoxShape>,
}

impl World {
    pub fn new(light: Option<PointLight>, shapes: Vec<BoxShape>) -> World {
        World { light, shapes }
    }

    pub fn intersect(&self, &r: &Ray) -> Intersections {
        let mut intersections: Vec<Intersection> = vec![];
        self.shapes.iter().for_each(|shape| {
            shape
                .intersect(r)
                .iter()
                .for_each(|i| intersections.push(i.clone()))
        });
        Intersections::new(intersections)
    }

    pub fn shade_hit(&self, comps: &Comps) -> Color {
        if self.light.is_some() {
            comps.object.material().lighting(
                &comps.object,
                &self.light.unwrap(),
                &comps.point,
                &comps.eyev,
                &comps.normalv,
                self.is_shadowed(&comps.over_point),
            )
        } else {
            BLACK
        }
    }

    pub fn color_at(&self, r: &Ray) -> Color {
        let i = self.intersect(r);
        match i.hit() {
            None => BLACK,
            Some(h) => {
                let comps = Comps::prepare_computations(h, r);
                self.shade_hit(&comps)
            }
        }
    }

    pub fn is_shadowed(&self, &point: &Tuple) -> bool {
        if self.light.is_none() {
            false
        } else {
            let v = self.light.unwrap().position - point;
            let distance = v.magnitude();
            let direction = v.normalize();

            let r = Ray::new(&point, &direction);
            let intersections = self.intersect(&r);
            let h = intersections.hit();
            h.is_some() && h.unwrap().t < distance
        }
    }
}

impl Default for World {
    fn default() -> World {
        let light = PointLight::new(&Tuple::point(-10., 10., -10.), &WHITE);

        let diffuse = 0.7;
        let specular = 0.2;
        let material = Material::new(
            Some(Color::new(0.8, 1., 0.6)),
            None,
            None,
            Some(diffuse),
            Some(specular),
            None,
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
            Some(Color::new(0.8, 1., 0.6)),
            None,
            None,
            Some(diffuse),
            Some(specular),
            None,
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
        let w = World {
            light: Some(light),
            ..Default::default()
        };
        let r = Ray::new(&Tuple::point(0., 0., 0.), &Tuple::vector(0., 0., 1.));
        let s = w.shapes[1].clone();

        let i = Intersection::new(0.5, s);

        let comps = Comps::prepare_computations(&i, &r);
        let c = w.shade_hit(&comps);
        assert_eq!(Color::new(0.90498, 0.90498, 0.90498), c);
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let light = PointLight::new(&Tuple::point(0., 0., -10.), &WHITE);
        let s1 = Sphere::default_boxed();
        let s2 = Sphere::new_boxed(Some(transform::translation(0., 0., 10.)), None);
        let w = World::new(Some(light), vec![s1, s2.clone()]);

        let r = Ray::new(&Tuple::point(0., 0., 5.), &Tuple::vector(0., 0., 1.));
        let i = Intersection::new(4., s2);

        let comps = Comps::prepare_computations(&i, &r);
        let c = w.shade_hit(&comps);
        assert_eq!(Color::new(0.1, 0.1, 0.1), c);
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
        let material = Material {
            color: w.shapes[0].material().color,
            pattern: w.shapes[0].material().pattern.clone(),
            ambient: 1.,
            diffuse: w.shapes[0].material().diffuse,
            specular: w.shapes[0].material().specular,
            shininess: w.shapes[0].material().shininess,
        };
        w.shapes.first_mut().unwrap().set_material(material.clone());
        w.shapes.last_mut().unwrap().set_material(material);

        let r = Ray::new(&Tuple::point(0., 0., 0.75), &Tuple::vector(0., 0., -1.));
        let c = w.color_at(&r);
        assert_eq!(w.shapes[1].material().color, c);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::default();
        let p = Tuple::point(0., 10., 0.);

        assert_eq!(false, w.is_shadowed(&p));
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_light() {
        let w = World::default();
        let p = Tuple::point(10., -10., 10.);

        assert_eq!(true, w.is_shadowed(&p));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = World::default();
        let p = Tuple::point(-20., 20., -20.);

        assert_eq!(false, w.is_shadowed(&p));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = World::default();
        let p = Tuple::point(-2., 2., -2.);

        assert_eq!(false, w.is_shadowed(&p));
    }
}
