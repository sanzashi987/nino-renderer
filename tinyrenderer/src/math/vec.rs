macro_rules! define_vec {
  ($name:ident, $($p:ident),+) => {
    #[derive(Debug, PartialEq, Copy, Clone, Default)]
    pub struct $name {
      $(
        pub $p:f32,
      )+
    }

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
