use super::Vec2;

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
