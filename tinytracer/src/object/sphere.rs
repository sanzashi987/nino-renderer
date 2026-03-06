use math::Vec3;

pub struct Sphere {
  pub center: Vec3,
  pub radius: f32,
}

impl Sphere {
  pub fn new(center: Vec3, radius: f32) -> Self {
    Self { center, radius }
  }

  pub fn ray_intersect(
    &self,
    ray_origin: &Vec3,
    ray_dir: &Vec3,
    first_intersect: &mut f32,
  ) -> bool {
    let to_center = self.center - *ray_origin;
    let to_center_projected_at_dir = to_center * *ray_dir; // dot product
    let ray_to_center =
      to_center * to_center - to_center_projected_at_dir * to_center_projected_at_dir;
    let raduis_exq = self.radius.exp2();
    if ray_to_center > raduis_exq {
      return false;
    }

    let half_chord = (raduis_exq - ray_to_center).sqrt();

    *first_intersect = to_center_projected_at_dir - half_chord;
    let far_intersect = to_center_projected_at_dir + half_chord;

    if *first_intersect < 0.0 {
      // emit inside the sphere
      *first_intersect = far_intersect;
    }

    if *first_intersect < 0.0 {
      return false;
    }

    return true;
  }
}
