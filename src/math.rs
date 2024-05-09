use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

macro_rules! define_mat {
  ($name:ident, $dim:expr) => {
    #[derive(Debug, Clone, Copy)]
    pub struct $name {
      data: [f32; $dim * $dim],
    }
  };
}

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

macro_rules! definv_vec_op_assign {
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
    definv_vec_op_assign!($name, AddAssign, add_assign, += $(,$var)+ );
    definv_vec_op_assign!($name, SubAssign, sub_assign, -= $(,$var)+ );
    definv_vec_op_assign!($name, MulAssign, mul_assign, *= $(,$var)+ );
    definv_vec_op_assign!($name, DivAssign, div_assign, /= $(,$var)+ );
  };
}

define_vec!(Vec2, x, y);

define_mat!(Mat2, 2);
define_mat!(Mat3, 3);
define_mat!(Mat4, 4);

// impl Mul<Vec2> for Mat4 {}
