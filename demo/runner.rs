use image;
// use nino_renderer::bresenham_line;

use nino_renderer::renderer::draw_line;

fn main() {
  let mut img = image::ImageBuffer::new(100, 100);
  draw_line(10.0, 10.0, 100.0, 100.0, &mut img, [0, 244, 244]);
  draw_line(50.0, 10.0, 100.0, 100.0, &mut img, [250, 0, 244]);
  draw_line(99.0, 50.0, 10.0, 1.0, &mut img, [250, 0, 244]);

  img.save("test.png").unwrap();
}
