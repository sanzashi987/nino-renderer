pub struct Layers {
  pub mask: u32,
}

impl Default for Layers {
  fn default() -> Self {
    Self { mask: 0b1 | 0b0 }
  }
}

impl Layers {
  pub fn set(&mut self, channel: u32) {
    self.mask = (1 << channel) as u32;
  }

  pub fn enable(&mut self, channel: u32) {
    self.mask |= 1 << channel;
  }

  pub fn enable_all(&mut self) {
    self.mask |= 0xffffffff;
  }

  pub fn toggle(&mut self, channel: u32) {
    self.mask ^= 1 << channel;
  }

  pub fn disable(&mut self, channel: u32) {
    self.mask &= !(1 << channel);
  }

  pub fn disableAll(&mut self) {
    self.mask = 0;
  }

  pub fn test(&self, layers: Self) -> bool {
    self.mask & layers.mask != 0
  }

  pub fn is_enable(&self, channel: u32) -> bool {
    self.mask & (1 << channel) != 0
  }
}
