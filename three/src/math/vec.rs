use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::Mat4;

macro_rules! define_vec_op {
  ($name:ident,$trait_name:ident,  $func_name:ident, $op:tt, $($p:ident),+) => {
    impl $trait_name for $name {
      type Output = Self;

      fn $func_name(self, rhs: $name) -> Self::Output {
        Self {
          $(
            $p: self.$p $op rhs.$p,
          )+
        }
      }
    }

    impl $trait_name<f32> for $name {
      type Output = Self;
      fn $func_name(self, rhs: f32) -> Self::Output {
        $name {
          $(
            $p:self.$p $op rhs,
          )+
        }
      }
    }
  };
}

macro_rules! define_vec_op_assign {
  ($name:ident,$trait_name:ident, $func:ident, $op:tt, $($var:ident),+) => {
    impl $trait_name for $name {
      fn $func(&mut self, rhs: Self) {
        $(
          self.$var $op rhs.$var;
        )+
      }
    }

    impl $trait_name<f32> for $name {
      fn $func(&mut self, rhs: f32) {
        $(
          self.$var $op rhs;
        )+
      }
    }
  };
}

macro_rules! define_vec {
  ($name:ident, $($p:ident),+) => {
    #[derive(Debug, PartialEq, Copy, Clone, Default)]
    pub struct $name {
      $(
        pub $p:f32,
      )+
    }

    define_vec_op!($name, Add, add, + ,$($p),+);
    define_vec_op!($name, Sub, sub, - ,$($p),+);
    define_vec_op!($name, Mul, mul, * ,$($p),+);
    define_vec_op!($name, Div, div, / ,$($p),+);
    define_vec_op_assign!($name, AddAssign, add_assign, += $(,$p)+ );
    define_vec_op_assign!($name, SubAssign, sub_assign, -= $(,$p)+ );
    define_vec_op_assign!($name, MulAssign, mul_assign, *= $(,$p)+ );
    define_vec_op_assign!($name, DivAssign, div_assign, /= $(,$p)+ );

    impl $name {
      pub const fn new($($p:f32),+) -> Self {
        $name{
          $(
            $p,
          )+

        }
      }

      pub fn min(&mut self, rhs: Self){
        $(
          self.$p = self.$p.min(rhs.$p);
        )+
      }

      pub fn max(&mut self, rhs: Self){
        $(
          self.$p = self.$p.max(rhs.$p);
        )+
      }

      pub fn zero() -> Self {
        $name{
          $(
            $p:0.0,
          )+
        }
      }

      pub fn dot(&self, rhs:&Self) -> f32{
        $(
          self.$p * rhs.$p +
        )+
        0.0
      }

      pub fn length_square(&self) -> f32 {
        $(
          self.$p * self.$p +
        )+
        0.0
      }

      pub fn length(&self) -> f32 {
        self.length_square().sqrt()
      }

      pub fn normalize(&self) -> Self {
        let length = self.length();
        Self {
          $(
            $p: self.$p/length,
          )+
        }
      }

    }
  };
}

define_vec!(Vec2, x, y);
define_vec!(Vec3, x, y, z);
define_vec!(Vec4, x, y, z, w);

impl Vec2 {
  pub fn cross(&self, rhs: &Self) -> f32 {
    self.x * rhs.y - self.y * rhs.x
  }
}
impl Vec3 {
  pub fn cross(&self, rhs: &Self) -> Self {
    Self {
      x: self.y * rhs.z - self.z * rhs.y,
      y: self.z * rhs.x - self.x * rhs.z,
      z: self.x * rhs.y - self.y * rhs.x,
    }
  }

  pub fn truncate_to_vec2(&self) -> Vec2 {
    Vec2 {
      x: self.x,
      y: self.y,
    }
  }

  pub fn x_axis() -> &'static Self {
    const V: Vec3 = Vec3::new(1.0, 0.0, 0.0);
    &V
  }

  pub fn y_axis() -> &'static Self {
    const V: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    &V
  }

  pub fn z_axis() -> &'static Self {
    const V: Vec3 = Vec3::new(0.0, 0.0, 1.0);
    &V
  }
}

impl Vec4 {
  pub fn truncated_to_vec3(&self) -> Vec3 {
    Vec3::new(self.x, self.y, self.z)
  }

  pub fn truncate_to_vec2(&self) -> Vec2 {
    Vec2::new(self.x, self.y)
  }

  pub fn from_vec3(v3: &Vec3, w: f32) -> Self {
    Self::new(v3.x, v3.y, v3.z, w)
  }
}

pub fn lerp<T>(a: T, b: T, t: f32) -> T
where
  T: Sub<Output = T> + Add<Output = T> + Mul<f32, Output = T> + Copy + Clone,
{
  a + (b - a) * t
}
