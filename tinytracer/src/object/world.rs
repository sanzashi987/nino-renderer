use super::ray::{HitConfig, HitRecord, Hittable, Ray};

pub struct World {
  objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for World {
  fn hit(&self, ray: &Ray, config: Option<HitConfig>) -> Option<HitRecord> {
    let HitConfig { t_max, .. } = config.unwrap_or(HitConfig::default());
    let mut hit_anything = None;
    let mut closet_so_far = t_max;

    for obj in &self.objects {
      if let Some(t) = obj.hit(ray, config) {
        if t.t < closet_so_far {
          closet_so_far = t.t;
          hit_anything = Some(t);
        }
      }
    }
    hit_anything
  }
}

impl World {
  pub fn new() -> Self {
    World {
      objects: Vec::new(),
    }
  }

  pub fn add(&mut self, object: Box<dyn Hittable>) {
    self.objects.push(object);
  }
  pub fn clear(&mut self) {
    self.objects.clear();
  }
}
