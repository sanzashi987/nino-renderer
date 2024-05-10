use crate::bresenham_line;
use crate::camera::Camera;
use crate::cohen_sutherland;
use crate::math::{Mat4, Vec2, Vec3, Vec4};
use image;

struct Viewport {
  x: i32,
  y: i32,
  w: u32,
  h: u32,
}

pub struct Renderer {
  camera: Camera,
  viewport: Viewport,
}

impl Renderer {
  pub fn new(w: u32, h: u32, camera: Camera) -> Self {
    Self {
      camera,
      viewport: Viewport { x: 0, y: 0, w, h },
    }
  }

  pub fn draw_triangle(&mut self, model: &Mat4, vertices: &[Vec3; 3], color: &Vec4) {
    let vertices = vertices.map(|v| {
      //homogeneous
      let mut h_coord = Vec4::from_vec3(&v, 1.0);
      // mvp & normalize
      h_coord = *self.camera.get_frustum().get_mat() * *model * h_coord;
      h_coord /= h_coord.w;

      // project the normalized vertex back
      Vec2::new(
        (h_coord.x + 1.0) * 0.5 * (self.viewport.w as f32 - 1.0) + self.viewport.x as f32,
        self.viewport.h as f32 - (h_coord.y + 1.0) * 0.5 * (self.viewport.h as f32 - 1.0)
          + self.viewport.y as f32,
      )
    });
  }

  pub fn draw_line(x0: f32, y0: f32, x1: f32, y1: f32, img: &mut image::RgbImage, color: [u8; 3]) {
    let p0 = Vec2 { x: x0, y: y0 };
    let p1 = Vec2 { x: x1, y: y1 };

    let rect_min = Vec2 { x: 50.0, y: 50.0 };
    let rect_max = Vec2 { x: 100.0, y: 100.0 };

    let res = cohen_sutherland::clip(&p0, &p1, &rect_min, &rect_max);

    // if let Some((next_p0, next_p1)) = res {}

    match res {
      Some((next_p0, next_p1)) => bresenham_line::draw_line(
        next_p0.x as i32,
        next_p0.y as i32,
        next_p1.x as i32,
        next_p1.y as i32,
        img,
        color,
      ),
      None => {
        return;
      }
    }
  }
}
