use crate::{
  data_array::ColorBuffer,
  math::{Vec3, Vec4},
};

pub struct Barycentric {
  alpha: f32,
  beta: f32,
  gamma: f32,
}

pub fn barycentric() {}
pub fn shade_triangle(points: &mut [Vec3; 3], result: &mut ColorBuffer, color: &Vec4) {
  let [p0, p1, p2] = points;

  if p0.y > p1.y {
    std::mem::swap(p0, p1);
  }
  if p0.y > p2.y {
    std::mem::swap(p0, p2);
  }
  if p1.y > p2.y {
    std::mem::swap(p1, p2);
  }
  let total_span = p2.y - p0.y;
  let top_span = p1.y - p0.y;
  let bottom_span = p2.y - p1.y;
  // let mut y = 0.0;
  for y in 0..total_span as i32 {
    let alpha = y as f32 / total_span;
    let bottom_half = y as f32 > top_span || p1.y == p0.y;
    let segment_height = if bottom_half { top_span } else { bottom_span };

    let beta = (y as f32 - if bottom_half { top_span } else { 0.0 }) / segment_height;

    let mut left = p0.x + (p2.x - p0.x) * alpha;
    let mut right = if bottom_half {
      p1.x + (p2.x - p1.x) * beta
    } else {
      p0.x + (p1.x - p0.x) * beta
    };

    if left > right {
      std::mem::swap(&mut left, &mut right);
    }

    for x in (left as i32)..(right as i32) {
      result.set(x as u32, y as u32, color);
    }
  }
}

// pub fn

pub struct BoundaryBox {
  x_min: f32,
  x_max: f32,
  y_min: f32,
  y_max: f32,
}

impl BoundaryBox {
  pub fn new(vertices: &[Vec3; 3], width: f32, height: f32) -> Self {
    let x_min = vertices
      .iter()
      .fold(std::f32::MAX, |min, v| if min < v.x { min } else { v.x })
      .max(0.0);
    let x_max = vertices
      .iter()
      .fold(std::f32::MIN, |max, v| if max > v.x { max } else { v.x })
      .min(width - 1.0);
    let y_min = vertices
      .iter()
      .fold(std::f32::MAX, |min, v| if min < v.y { min } else { v.y })
      .max(0.0);
    let y_max = vertices
      .iter()
      .fold(std::f32::MIN, |max, v| if max > v.y { max } else { v.y })
      .min(height - 1.0);

    Self {
      x_min,
      y_min,
      x_max,
      y_max,
    }
  }
}
