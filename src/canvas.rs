use super::color::*;

// TODO: Document the public API

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<Color>> // A vector for each row of pixel 
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Canvas::new_filled(width, height, BLACK)
    }

    // FIXME: How to make 3rd argument optionnal for 'new' ?
    pub fn new_filled(width: usize, height: usize, color: Color) -> Canvas {
        Canvas {width, height, pixels: vec![vec![color; width]; height]}
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        // Note: No check for out-of-bounds
        self.pixels[y][x] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        // Note: No check for out-of-bounds
        self.pixels[y][x]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);
        
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);

        // Make sure every pixel is black
        for row in 0..c.height {
            for x in 0..c.width {
                assert_eq!(c.pixel_at(x, row), BLACK);
            }
        }
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 20);
        
        // Write and read red pixel
        c.write_pixel(2, 3, RED);
        assert_eq!(c.pixel_at(2, 3), RED);

        assert_eq!(c.pixel_at(2, 4), BLACK); // Neighbor is still black
    }
}