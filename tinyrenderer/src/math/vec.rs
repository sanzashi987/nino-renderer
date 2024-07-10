use std::ops::{Add, Div, Mul, Sub};

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

macro_rules! define_vec {
  ($name:ident, $($p:ident),+) => {
    #[derive(Debug, PartialEq, Copy, Clone, Default)]
    pub struct $name {
      $(
        pub $p:f32,
      )+
    }

    define_vec_op!($name, Add, add, + ,$($p),+);
    define_vec_op!($name, Sub, sub, + ,$($p),+);
    define_vec_op!($name, Mul, mul, + ,$($p),+);
    define_vec_op!($name, Div, div, + ,$($p),+);

    impl $name {
      pub fn new($($p:f32),+) -> Self {
        $name{
          $(
            $p,
          )+

        }
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
}

pub fn lerp<T>(a: T, b: T, t: f32) -> T
where
  T: Sub<Output = T> + Add<Output = T> + Mul<f32, Output = T> + Copy + Clone,
{
  a + (b - a) * t
}
