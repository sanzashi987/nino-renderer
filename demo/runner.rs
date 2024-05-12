use image;
// use nino_renderer::bresenham_line;

use image::imageops::colorops;
use nino_renderer::camera;
use nino_renderer::math::{Mat4, Vec3, Vec4};
use nino_renderer::renderer::Renderer;

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 720;

fn main() {
  let mut img = image::ImageBuffer::new(100, 100);

  let camera = camera::Camera::new(
    1.0,
    WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
    45f32.to_radians(),
  );
  let mut renderer = Renderer::new(100, 100, camera);

  let color = Vec4::new(0.0, 1.0, 10.0, 1.0);

  let vertices = [
    Vec3::new(-1.0, 1.0, -2.0),
    Vec3::new(1.0, 1.0, -2.0),
    Vec3::new(0.0, -1.0, -2.0),
  ];

  renderer.draw_triangle(&Mat4::identity(), &vertices, &color);

  renderer.draw_line(10.0, 10.0, 100.0, 100.0, &mut img, [0, 244, 244]);
  renderer.draw_line(50.0, 10.0, 100.0, 100.0, &mut img, [250, 0, 244]);
  renderer.draw_line(99.0, 50.0, 10.0, 1.0, &mut img, [250, 0, 244]);

  img.save("test.png").unwrap();
}
