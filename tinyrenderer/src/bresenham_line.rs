use crate::{
  data_array::ColorBuffer,
  math::{Vec2, Vec4},
};

pub fn line(pt0: Vec2, pt1: Vec2, color_buffer: &mut ColorBuffer) {
  let Vec2 {
    x: mut x0,
    y: mut y0,
  } = pt0;
  let Vec2 {
    x: mut x1,
    y: mut y1,
  } = pt1;

  let steep = if (x1 - x0).abs() > (y1 - y0).abs() {
    false
  } else {
    std::mem::swap(&mut x0, &mut y0);
    std::mem::swap(&mut x1, &mut y1);
    true
  };

  if x0 > x1 {
    // make it left−to−right
    std::mem::swap(&mut x0, &mut x1);
    std::mem::swap(&mut y0, &mut y1);
  }

  let dx = x1 - x0;
  let dy = y1 - y0;

  let derror2 = dy.abs() * 2.0;
  let mut error2 = 0.0;

  let mut y = y0;
  let mut x = x0;
  while x <= x1 {
    let (xx, yy) = if steep { (y, x) } else { (x, y) };
    color_buffer.set(xx as u32, yy as u32, &Vec4::new(1.0, 1.0, 1.0, 1.0));
    error2 += derror2;
    if error2 as f32 > dx {
      y = y + if y1 > y0 { 1.0 } else { -1.0 };
      error2 -= dx * 2.0;
    }
    x += 1.0
  }

  // let Vec2 { 1.0, y } = pt0;
}
