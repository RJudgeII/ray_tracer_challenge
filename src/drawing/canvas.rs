use super::color::Color;
use std::fs::write;

#[derive(Clone)]
pub struct Canvas {
  pub width: usize,
  pub height: usize,
  pixels: Vec<Color>,
}

//  Instantiations
impl Canvas {
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      width,
      height,
      pixels: vec![Color::black(); width * height],
    }
  }
}

//  Operations
impl Canvas {
  pub fn pixel_at(&self, x: usize, y: usize) -> Color {
    if x >= self.width || y >= self.height {
      panic!("Cannot access a pixel outside the Canvas.");
    }
    let pixel_index = self.get_pixel_index(x, y);
    self.pixels[pixel_index].clone()
  }

  fn get_pixel_index(&self, x: usize, y: usize) -> usize {
    y * self.width + x
  }

  pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
    if x >= self.width || y >= self.height {
      panic!("Cannot accessa pixel outside the Canvas.");
    }
    let pixel_index = self.get_pixel_index(x, y);
    self.pixels[pixel_index] = color;
  }

  pub fn write_to_png(&self, filename: &str) {
    println!("Attempting to write {}", filename);
    let color_data = self.clone().clamp_pixels().as_rgba32();
    let png_data = self.write_png_data(color_data);
    write(filename, png_data).expect("Could not write png file to disk.");
    println!("Finished writing file.");
  }

  fn clamp_pixels(mut self) -> Self {
    let mut pixel_data: Vec<Color> = Vec::new();
    for pixel in self.pixels {
      let max_value = pixel.red.max(pixel.green).max(pixel.blue);
      let red = if max_value == 0.0 {
        0.0
      } else {
        (pixel.red / max_value) * 255.0
      };
      let green = if max_value == 0.0 {
        0.0
      } else {
        (pixel.green / max_value) * 255.0
      };
      let blue = if max_value == 0.0 {
        0.0
      } else {
        (pixel.blue / max_value) * 255.0
      };
      pixel_data.push(Color {
        red: red.min(255.0).max(0.0),
        green: green.min(255.0).max(0.0),
        blue: blue.min(255.0).max(0.0),
      });
    }
    self.pixels = pixel_data;
    self
  }

  fn as_rgba32(&self) -> Vec<u8> {
    let mut color_data: Vec<u8> = Vec::new();
    for pixel in self.pixels.clone() {
      color_data.push(pixel.red as u8);
      color_data.push(pixel.green as u8);
      color_data.push(pixel.blue as u8);
      color_data.push(255);
    }
    color_data
  }

  fn write_png_data(&self, color_data: Vec<u8>) -> Vec<u8> {
    let mut data = Vec::new();
    let mut encoder = png::Encoder::new(&mut data, self.width as u32, self.height as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&color_data).unwrap();
    drop(writer);

    data
  }
}

#[cfg(test)]
mod canvas_tests {
  use super::*;
  use crate::macros::fuzzy_eq::FuzzyEq;

  mod initiation_tests {
    use super::*;
    use crate::assert_feq;

    #[test]
    fn creating_a_canvas() {
      let canvas = Canvas::new(10, 20);

      assert_eq!(10, canvas.width);
      assert_eq!(20, canvas.height);

      for x in 0..canvas.width {
        for y in 0..canvas.height {
          assert_feq!(canvas.pixel_at(x, y), Color::black())
        }
      }
    }
  }

  mod operation_tests {
    use super::*;
    use crate::assert_feq;

    #[test]
    fn writing_pixels_to_a_canvas() {
      let mut c = Canvas::new(10, 20);
      let red = Color::red();

      c.write_pixel(2, 3, red.clone());

      assert_feq!(c.pixel_at(2, 3), red);
    }
  }
}
