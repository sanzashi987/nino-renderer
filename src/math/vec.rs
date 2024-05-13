use std::default::Default;

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

macro_rules! define_vec_op {
  ($name:ident,$trait_name:ident, $func:ident, $op:tt, $($var:ident),+) => {
    impl $trait_name for $name {
      type Output = Self;
      fn $func(self, rhs: Self) -> Self::Output {
        $name {
          $(
            $var:self.$var $op rhs.$var,
          )+
        }
      }
    }

    impl $trait_name<f32> for $name {
      type Output = Self;
      fn $func(self, rhs: f32) -> Self::Output {
        $name {
          $(
            $var:self.$var $op rhs,
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
  ($name:ident,$($var:ident),+) => {

    #[derive(Debug, PartialEq, Copy, Clone, Default)]
    pub struct $name {
      $(
        pub $var:f32,
      )+
    }

    impl $name {
      pub const fn new($($var:f32),+) -> $name{
        $name{
          $(
            $var,
          )+
        }
      }

      pub fn zero() -> $name {
        $name {
          $( $var: 0f32, )+
        }
      }

      pub fn length_square(&self)->f32{
        $(
          self.$var * self.$var +
        )+
        0.0
      }

      pub fn length(&self) ->f32{
        self.length_square().sqrt()
      }

      pub fn normalize(&self) -> $name {
        *self / self.length()
      }
    }

    impl Neg for $name {
      type Output = Self;
      fn neg(self) -> Self::Output{
        Self::new(
          $(
            -self.$var,
          )+
        )
      }
    }

    define_vec_op!($name, Add, add, + $(,$var)+);
    define_vec_op!($name, Sub, sub, - $(,$var)+);
    define_vec_op!($name, Mul, mul, * $(,$var)+);
    define_vec_op!($name, Div, div, / $(,$var)+);
    define_vec_op_assign!($name, AddAssign, add_assign, += $(,$var)+ );
    define_vec_op_assign!($name, SubAssign, sub_assign, -= $(,$var)+ );
    define_vec_op_assign!($name, MulAssign, mul_assign, *= $(,$var)+ );
    define_vec_op_assign!($name, DivAssign, div_assign, /= $(,$var)+ );
  };
}

define_vec!(Vec2, x, y);
define_vec!(Vec3, x, y, z);
define_vec!(Vec4, x, y, z, w);

impl Vec4 {
  pub fn from_vec3(v: &Vec3, w: f32) -> Vec4 {
    Self {
      x: v.x,
      y: v.y,
      z: v.z,
      w,
    }
  }

  pub fn truncated_to_vec3(&self) -> Vec3 {
    Vec3 {
      x: self.x,
      y: self.y,
      z: self.z,
    }
  }

  pub fn truncated_to_vec2(&self) -> Vec2 {
    Vec2 {
      x: self.x,
      y: self.y,
    }
  }
}

pub fn lerp<T>(a: T, b: T, t: f32) -> T
where
  T: Sub<Output = T> + Add<Output = T> + Mul<f32, Output = T> + Copy + Clone,
{
  a + (b - a) * t
}
