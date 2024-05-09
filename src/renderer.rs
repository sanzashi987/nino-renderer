use crate::bresenham_line;
use crate::cohen_sutherland;
use crate::math::Vec2;
use image;

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
