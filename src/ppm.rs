use super::canvas::*;
use super::color::*;

// TODO: Document the public API

const MAX_COLOR_VALUE: i32 = 255;
const MAX_PIXELS_PER_LINE: usize = 5;

fn clamp(value: i32, min: i32, max: i32) -> i32 {
    if value < min {
        return min;
    }

    if value > max {
        return max;
    }

    value
}

pub fn canvas_to_ppm(canvas: Canvas) -> String {
    let mut s = format!("P3\n{} {}\n{}\n", canvas.width, canvas.height, MAX_COLOR_VALUE);
    
    for y in 0..canvas.height {
        for x in 0..canvas.width {
            let pixel = canvas.pixel_at(x, y);
            let red = (pixel.r * MAX_COLOR_VALUE as f64) as i32;
            let green = (pixel.g * MAX_COLOR_VALUE as f64) as i32;
            let blue = (pixel.b * MAX_COLOR_VALUE as f64) as i32;

            let rgb_tuple = format!("{} {} {}", clamp(red, 0, MAX_COLOR_VALUE), clamp(green, 0, MAX_COLOR_VALUE), clamp(blue, 0, MAX_COLOR_VALUE));
            s.push_str(&rgb_tuple);
            if (x > 0 && x % MAX_PIXELS_PER_LINE == 0) || x == (canvas.width -1) {
                s.push('\n');
            }
            else {
                s.push(' ');
            }
        }
    }

    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructing_the_ppm_header() {
        let c = Canvas::new(5,3);
        let ppm = canvas_to_ppm(c);
        let mut lines = ppm.lines();

        assert_eq!(Some("P3"), lines.next());
        assert_eq!(Some("5 3"), lines.next());
        assert_eq!(Some("255"), lines.next());
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = Canvas::new(5,3);
        let c1 = Color::new(1.5, 0., 0.);
        let c2 = Color::new(0., 0.505, 0.);
        let c3 = Color::new(-0.5, 0., 1.);

        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        let ppm = canvas_to_ppm(c);
        let mut lines = ppm.lines();

        // Skip header
        lines.next(); lines.next(); lines.next();

        assert_eq!(Some("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0"), lines.next());
        assert_eq!(Some("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0"), lines.next());
        assert_eq!(Some("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"), lines.next());
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let color = Color::new(1., 0.8, 0.6);
        let c = Canvas::new_filled(10, 2, color);

        let ppm = canvas_to_ppm(c);
        let mut lines = ppm.lines();

        // Skip header
        lines.next(); lines.next(); lines.next();

        assert_eq!(Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153"), lines.next());
        assert_eq!(Some("255 204 153 255 204 153 255 204 153 255 204 153"), lines.next());
        assert_eq!(Some("255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153"), lines.next());
        assert_eq!(Some("255 204 153 255 204 153 255 204 153 255 204 153"), lines.next());
    }

    #[test]
    fn ppm_files_are_terminated_by_newline() {
        let c = Canvas::new(5,3);
        let ppm = canvas_to_ppm(c);

        assert_eq!(Some('\n'), ppm.chars().last());
    }
}