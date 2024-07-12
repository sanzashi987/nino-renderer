use crate::{
  data_array::{ColorBuffer, DepthBuffer},
  math::{lerp, Barycentric, Vec2, Vec3, Vec4},
};

fn shade_triangle_scanline<F: Fn(&Vec2) -> Vec4>(
  points: &mut [Vec3; 3],
  depth: &mut DepthBuffer,
  result: &mut ColorBuffer,
  color: F,
) {
  let mut points = points.map(|v| v.truncate_to_vec2());

  let [p0, p1, p2] = &mut points;

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
    let segment_height = if bottom_half { bottom_span } else { top_span };

    let beta = (y as f32 - if bottom_half { top_span } else { 0.0 }) / segment_height;

    let mut left = lerp(*p0, *p2, alpha);

    let mut right = if bottom_half {
      lerp(*p1, *p2, beta)
    } else {
      lerp(*p0, *p1, beta)
    };

    if left.x > right.x {
      std::mem::swap(&mut left, &mut right);
    }

    // let line_span = right as i32 - left as i32;
    for x in (left.x as i32)..(right.x as i32 + 1) {
      // let progress = (left as f32 + x as f32) / line_span as f32;

      // let z = lerp(left.z, right.z, progress);

      // if depth.get(x as u32, y as u32 + p0.y as u32) > z {
      // depth.set(x as u32, y as u32 + p0.y as u32, z);
      // result.set(x as u32, y as u32 + p0.y as u32, color);
      result.set(
        x as u32,
        y as u32 + p0.y as u32,
        &Vec4::new(1.0, 1.0, 1.0, 1.0),
      );
      // }
    }
  }
}

// pub fn

pub struct BoundaryBox {
  pub x_min: f32,
  pub x_max: f32,
  pub y_min: f32,
  pub y_max: f32,
}

impl BoundaryBox {
  pub fn new(vertices: &[Vec2; 3], width: f32, height: f32) -> Self {
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

fn shade_triangle_barycentric<F: Fn(&Vec2) -> Vec4>(
  points: &mut [Vec3; 3],
  depth: &mut DepthBuffer,
  result: &mut ColorBuffer,
  color: F,
) {
  let points_2d = points.map(|v| v.truncate_to_vec2());
  let (width, height) = (result.width(), result.height());
  let boundary = BoundaryBox::new(&points_2d, width as f32, height as f32);

  for x in (boundary.x_min as u32)..(boundary.x_max as u32 + 1) {
    for y in (boundary.y_min as u32)..(boundary.y_max as u32 + 1) {
      let barycentric = Barycentric::new(&Vec2::new(x as f32, y as f32), &points_2d);
      if !barycentric.is_inside() {
        continue;
      }

      let z = barycentric.alpha() * points[0].z
        + barycentric.beta() * points[1].z
        + barycentric.gamma() * points[2].z;

      if depth.get(x, y) < z {
        depth.set(x, y, z);
        let c = color(&Vec2::default());

        result.set(x, y, &c);
      }
    }
  }
}

pub fn shade_triangle<F: Fn(&Vec2) -> Vec4>(
  points: &mut [Vec3; 3],
  depth: &mut DepthBuffer,
  result: &mut ColorBuffer,
  color: F,
) {
  if cfg!(feature = "scanline") {
    shade_triangle_scanline(points, depth, result, color);
  } else {
    shade_triangle_barycentric(points, depth, result, color);
  }
}
