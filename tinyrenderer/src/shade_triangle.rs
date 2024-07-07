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
pub fn shade_triangle(points: &mut [Vec3; 3], result: ColorBuffer, color: &Vec4) {
  let [pt0, pt1, pt2] = points;

  if pt0.y > pt1.y {
    std::mem::swap(pt0, pt1);
  }
  if pt0.y > pt2.y {
    std::mem::swap(pt0, pt2);
  }
  if pt1.y > pt2.y {
    std::mem::swap(pt1, pt2);
  }
  let y_span = pt2.y - pt0.y;
  let mut y = pt0.y;
  while y <= pt1.y {

    


  }
}

// pub fn
