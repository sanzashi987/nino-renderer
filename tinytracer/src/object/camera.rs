use std::usize;

use derive_builder::Builder;
use math::Vec3;
use rand::RngExt;

use super::{
  ray::{Hittable, Ray},
  world::World,
};

fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
  let on_unit_sphere = Vec3::random_unit();
  if on_unit_sphere * *normal > 0.0 {
    on_unit_sphere
  } else {
    on_unit_sphere * -1.0
  }
}

pub fn ray_color(world: &World, ray: &Ray, depth: i32) -> Vec3 {
  if depth == 0 {
    return Vec3::zero();
  }

  if let Some(rec) = world.hit(ray, None) {
    // let direction = Vec3::random_unit() + rec.normal;
    // return ray_color(world, &Ray::new(rec.point, direction), depth - 1) * 0.1;
    if let Some((a, scattered)) = rec.material.scatter(ray, &rec) {
      let c = ray_color(world, &scattered, depth - 1);
      return Vec3::new(a.x * c.x, a.y * c.y, a.z * c.z);
    } else {
      return Vec3::zero();
    }
  }

  let unit_dir = ray.direction.normalize();
  let a = 0.5 * (unit_dir.y + 1.0);

  return Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a;
}

fn gamma_correction(linear: f32) -> f32 {
  if linear > 0.0 {
    linear.sqrt()
  } else {
    0.0
  }
}

#[derive(Debug, Default, Builder)]
#[builder(name = "CameraBuilder", build_fn(skip), default)]
pub struct Camera {
  pub aspect_ratio: f32,
  pub center: Vec3,
  pub image_width: i32,
  pub samples_per_pixel: i32,
  pub max_depth: i32,
  // in degree
  pub fov: f32,
  pub look_at: Vec3,
  pub vup: Vec3,
  // in degrees
  pub defocus_angle: f32,
  pub focus_dist: f32,

  // ---- 下面都是由上面的输入计算得到的，不暴露 setter ----
  #[builder(setter(skip))]
  pub look_from: Vec3,
  #[builder(setter(skip))]
  image_height: i32,
  #[builder(setter(skip))]
  pixel_delta_u: Vec3,
  #[builder(setter(skip))]
  pixel_delta_v: Vec3,
  #[builder(setter(skip))]
  pixel00_loc: Vec3,
  #[builder(setter(skip))]
  pixel_samples_scale: f32,
  #[builder(setter(skip))]
  u: Vec3,
  #[builder(setter(skip))]
  v: Vec3,
  #[builder(setter(skip))]
  w: Vec3,
  #[builder(setter(skip))]
  defocus_disk_u: Vec3,
  #[builder(setter(skip))]
  defocus_disk_v: Vec3,
}

impl CameraBuilder {
  pub fn build(&self) -> Camera {
    // 每个输入字段：未设置则 fallback 到默认值（这就是你想要的 `fov = 30` 效果）
    let image_width = self.image_width.unwrap_or(400);
    let aspect_ratio = self.aspect_ratio.unwrap_or(16.0 / 9.0);
    let center = self.center.unwrap_or(Vec3::zero());
    let fov = self.fov.unwrap_or(30.0);
    let look_at = self.look_at.unwrap_or(Vec3::new(0.0, 0.0, -1.0));
    let vup = self.vup.unwrap_or(*Vec3::y_axis());
    let samples_per_pixel = self.samples_per_pixel.unwrap_or(100);
    let max_depth = self.max_depth.unwrap_or(10);
    let focus_dist = self.focus_dist.unwrap_or(10.0);
    let defocus_angle = self.defocus_angle.unwrap_or(0.0);

    let image_height = ((image_width as f32 / aspect_ratio) as i32).max(1);
    let look_from = center;

    let viewport_height = 2.0 * focus_dist * (fov / 2.0).to_radians().tan();
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

    let w = (look_from - look_at).normalize();
    let u = vup.cross(&w).normalize();
    let v = w.cross(&u);

    // see ![viewport](../images/viewport.jpg)
    let viewport_u = u * viewport_width;
    let viewport_v = v * viewport_height * -1.0; // down growth

    let pixel_delta_u = viewport_u / (image_width as f32);
    let pixel_delta_v = viewport_v / (image_height as f32);

    let viewport_upper_left = center - w * focus_dist - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // defocus disk basis vectors
    let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
    let defocus_disk_u = u * defocus_radius;
    let defocus_disk_v = v * defocus_radius;

    Camera {
      image_width,
      image_height,
      aspect_ratio,
      center,
      pixel_delta_u,
      pixel_delta_v,
      pixel00_loc,
      samples_per_pixel,
      pixel_samples_scale: 1.0 / samples_per_pixel as f32,
      max_depth,
      fov,
      look_at,
      look_from,
      vup,
      u,
      v,
      w,
      defocus_disk_u,
      defocus_disk_v,
      focus_dist,
      defocus_angle,
    }
  }
}

fn random_in_unit_disk() -> Vec3 {
  loop {
    let p = Vec3::new(
      rand::rng().random_range(-1.0..1.0),
      rand::rng().random_range(-1.0..1.0),
      0.0,
    );

    if p.length_square() < 1.0 {
      return p;
    }
  }
}

impl Camera {
  pub fn image_height(&self) -> i32 {
    self.image_height
  }

  pub fn builder() -> CameraBuilder {
    CameraBuilder::default()
  }

  fn defocus_disk_sample(&self) -> Vec3 {
    // Returns a random point in the camera defocus disk.
    let Vec3 { x, y, .. } = random_in_unit_disk();
    return self.center + self.defocus_disk_u * x + self.defocus_disk_v * y;
  }

  pub fn get_ray(&self, i: i32, j: i32) -> Ray {
    let Self {
      center,
      pixel_delta_u,
      pixel_delta_v,
      pixel00_loc,
      ..
    } = self;
    let mut offset = Vec3::random() - Vec3::new(0.5, 0.5, 0.5);
    // let mut offset = Vec3::zero();
    offset.z = 0.0;

    let pixel_center = *pixel00_loc
      + (*pixel_delta_u * (i as f32 + offset.x))
      + (*pixel_delta_v * (j as f32 + offset.y));

    let origin = if self.defocus_angle <= 0.0 {
      *center
    } else {
      self.defocus_disk_sample()
    };
    // 方向必须从实际起点指向焦平面上的采样点，光线才会在焦平面汇聚
    let ray_direction = pixel_center - origin;

    Ray::new(origin, ray_direction)
  }

  pub fn render(&self, world: &World, buffer: &mut Vec<u8>) {
    let Self {
      image_width,
      image_height,
      samples_per_pixel,
      max_depth,
      ..
    } = self;
    for j in 0..*image_height {
      for i in 0..*image_width {
        let mut c = Vec3::zero();

        for _ in 0..*samples_per_pixel {
          let r = self.get_ray(i, j);
          let color = ray_color(world, &r, *max_depth);
          c += color;
        }

        let idx = (j as usize * *image_width as usize + i as usize) * 3;
        c *= self.pixel_samples_scale;
        let x = gamma_correction(c.x);
        let y = gamma_correction(c.y);
        let z = gamma_correction(c.z);

        buffer[idx] = (x.clamp(0.0, 1.0) * 255.0) as u8;
        buffer[idx + 1] = (y.clamp(0.0, 1.0) * 255.0) as u8;
        buffer[idx + 2] = (z.clamp(0.0, 1.0) * 255.0) as u8;
      }
    }
  }
}
