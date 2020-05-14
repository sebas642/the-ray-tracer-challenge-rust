use super::canvas::Canvas;
use super::matrix::Matrix;
use super::ray::Ray;
use super::tuple::Tuple;
use super::world::World;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub fov: f64, // Field of view
    pub pixel_size: f64,
    pub half_width: f64,
    pub half_height: f64,
    pub transform: Matrix
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, fov: f64, transform: Option<Matrix>) -> Camera {
        let half_view = (fov / 2.).tan();
        let aspect = hsize as f64 / vsize as f64;

        let (half_width, half_height) = if aspect >= 1. {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = half_width * 2. / hsize as f64;
        Camera {hsize, vsize, fov, pixel_size, half_width, half_height, transform: transform.unwrap_or_default()}
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        // the offset from the edge of the canvas to the pixel's center
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        // the untransformed coordinates of the pixel in world space
        let worldx = self.half_width - xoffset;
        let worldy = self.half_height - yoffset;

        // using a camera matrix, transform the canvas point and the origin
        // then compute the ray's direction vector
        // (remember the canvas is at z = -1)
        let t_inverse = self.transform.inverse();
        let pixel = t_inverse * Tuple::point(worldx, worldy, -1.);
        let origin = t_inverse * Tuple::point(0., 0., 0.);
        let direction = (pixel - origin).normalize();

        Ray::new(&origin, &direction)
    }

    pub fn render(&self, w: &World) -> Canvas {
        let mut canvas = Canvas::new(self.hsize, self.vsize);
        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let r = self.ray_for_pixel(x, y);
                let c = w.color_at(&r);
                canvas.write_pixel(x, y, c)
            }
        }
        canvas
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::matrix::MATRIX_IDENTITY;
    use crate::transform;
    use crate::transformation::view_transform;
    use crate::utils::approx_eq;
    use std::f64;

    #[test]
    fn constructing_a_camera() {
        let c = Camera::new(160, 120, f64::consts::FRAC_PI_2, None);

        assert_eq!(160, c.hsize);
        assert_eq!(120, c.vsize);
        assert_eq!(f64::consts::FRAC_PI_2, c.fov);
        assert_eq!(MATRIX_IDENTITY, c.transform);
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, f64::consts::FRAC_PI_2, None);
        assert!(approx_eq(0.01, c.pixel_size));
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, f64::consts::FRAC_PI_2, None);
        assert!(approx_eq(0.01, c.pixel_size));
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, f64::consts::FRAC_PI_2, None);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(Tuple::point(0., 0., 0.), r.origin);
        assert_eq!(Tuple::vector(0., 0., -1.), r.direction);
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, f64::consts::FRAC_PI_2, None);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(Tuple::point(0., 0., 0.), r.origin);
        assert_eq!(Tuple::vector(0.66519, 0.33259, -0.66851), r.direction);
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let t = transform::translation(0., -2., 5.);
        let r = transform::rotation_y(f64::consts::FRAC_PI_4);
        let c = Camera::new(201, 101, f64::consts::FRAC_PI_2, Some(transform::transforms(&[t, r])));
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(Tuple::point(0., 2., -5.), r.origin);
        assert_eq!(Tuple::vector(2f64.sqrt()/2., 0., -2f64.sqrt()/2.), r.direction);
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = World::default();

        let from = Tuple::point(0., 0., -5.);
        let to = Tuple::point(0., 0., 0.);
        let up = Tuple::vector(0., 1., 0.);
        let t = view_transform(&from, &to, &up);
        let c = Camera::new(11, 11, f64::consts::FRAC_PI_2, Some(t));

        let image = c.render(&w);

        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), image.pixel_at(5, 5));
    }
}
