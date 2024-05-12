// use fltk::browser::BrowserScrollbar;
use crate::image::ColorAttachment;
use crate::math;

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
