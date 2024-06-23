use std::result;

use crate::cohen_sutherland;
// use fltk::browser::BrowserScrollbar;
use crate::image::ColorAttachment;
use crate::math::{self, Vec2};

//Bresenham
pub fn draw_line(
  x0: i32,
  y0: i32,
  x1: i32,
  y1: i32,
  color: &math::Vec4,
  color_pool: &mut ColorAttachment,
) {
  let mut dy = (y1 - y0).abs();
  let mut dx = (x1 - x0).abs();
  let mut x = x0;
  let mut y = y0;

  let mut step_x = if x1 > x0 { 1 } else { -1 };
  let mut step_y = if y1 > y0 { 1 } else { -1 };

  let y_grows_faster = dx < dy;

  let final_x = if y_grows_faster { y1 } else { x1 };
  if y_grows_faster {
    std::mem::swap(&mut dx, &mut dy);
    std::mem::swap(&mut x, &mut y);
    std::mem::swap(&mut step_x, &mut step_y);
  }

  let mut e = -dx;

  let step = 2 * dy;
  let desc = -2 * dx;
  while x != final_x {
    if y_grows_faster {
      color_pool.set(y as u32, x as u32, color);
    } else {
      color_pool.set(x as u32, y as u32, color);
    }

    x += step_x;
    e += step;
    y += if e >= 0 {
      e += desc;
      step_y
    } else {
      0
    };
  }

  // if y1 - y0 > x1 - x0 {}
}

pub struct Bresenham {
  final_x: i32,
  x: i32,
  y: i32,
  steep: i32,
  step: i32,
  e: i32,
  step_y: i32,
  step_x: i32,
  desc: i32,
}

impl Bresenham {
  pub fn new(pt1: Vec2, pt2: Vec2, min: Vec2, max: Vec2) -> Option<Self> {
    let clip_result = cohen_sutherland::clip(&pt1, &pt2, &min, &max);
    if let Some((v1, v2)) = clip_result {
      let x0 = v1.x as i32;
      let y0 = v1.y as i32;
      let x1 = v2.x as i32;
      let y1 = v2.y as i32;
      let mut dy = (y1 - y0).abs();
      let mut dx = (x1 - x0).abs();
      let mut x = x0;
      let mut y = y0;

      let mut step_x = if x1 > x0 { 1 } else { -1 };
      let mut step_y = if y1 > y0 { 1 } else { -1 };

      let steep = if dx < dy { 1 } else { -1 };
      let final_x = if dx < dy { y1 } else { x1 };

      if dx < dy {
        std::mem::swap(&mut dx, &mut dy);
        std::mem::swap(&mut x, &mut y);
        std::mem::swap(&mut step_x, &mut step_y);
      }

      let mut e = -dx;
      let step = 2 * dy;
      let desc = -2 * dx;

      Some(Bresenham {
        final_x,
        x,
        y,
        steep,
        e,
        step_x,
        step_y,
        desc,
        step,
      })
    } else {
      None
    }
  }
}

impl Iterator for Bresenham {
  type Item = (i32, i32);

  fn next(&mut self) -> Option<Self::Item> {
    if self.x == self.final_x {
      return None;
    }

    let result = if self.steep > 0 {
      (self.y, self.x)
    } else {
      (self.x, self.y)
    };

    self.e += self.step;
    if self.e >= 0 {
      self.y += self.step_y;
      self.e += self.desc;
    }

    self.x = self.step_x;

    Some(result)
  }
}
