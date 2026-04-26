use core::f32;

use image::buffer;
use math::{Vec2, Vec3};
use tinytracer::object::{
  light::{self, Light},
  material::Material,
  ray::Ray,
  sphere::Sphere,
};

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 768.0;
const HALF_WIDTH: f32 = (WINDOW_WIDTH - 1.0) / 2.0;
const HALF_HEIGHT: f32 = (WINDOW_HEIGHT - 1.0) / 2.0;

#[test]
fn test() {
  // let end = 4.0f32;
  for j in 0..WINDOW_WIDTH as u8 {
    dbg!(j);
  }
}

// fn cast_ray(origin: &Vec3, dir: &Vec3, sphere: &Sphere) -> Vec3 {
//   if let Some(distance) = sphere.ray_intersect(origin, dir) {
//     return Vec3::new(0.4, 0.4, 0.3) * 255.0;
//   }
//   return Vec3::new(0.2, 0.7, 0.8) * 255.0;

// }
fn reflect(i: Vec3, n: Vec3) -> Vec3 {
  let dot = i * n;
  let r = i - n * dot * 2.0;
  r.normalize()
}

fn cast_ray(origin: &Vec3, dir: &Vec3, spheres: &Vec<Sphere>, lights: &Vec<Light>) -> Vec3 {
  let mut dist = f32::INFINITY;
  let mut res: Option<(&Material, Vec3, Vec3)> = None;
  for s in spheres {
    if let Some(distance) = s.ray_intersect(origin, dir) {
      if distance < dist {
        dist = distance;
        let hit = *origin + *dir * distance;
        let norm = (hit - s.center).normalize();
        res = Some((&s.material, hit, norm));
      }
    }
  }
  if let Some((m, hit, norm)) = res {
    let mut diffuse_intensity = 0f32;
    let mut specular_intensity = 0f32;

    for light in lights {
      let light_dir = (light.position - hit).normalize();
      diffuse_intensity += light.intensity * (light_dir * norm).max(0.0);
      specular_intensity +=
        light.intensity * (reflect(light_dir, norm) * *dir).max(0.0).powf(m.shininess);
    }
    return m.diffuse * diffuse_intensity * m.albedo.x
      + Vec3::new(1.0, 1.0, 1.0) * specular_intensity * m.albedo.y;
  }
  // background color
  return Vec3::new(0.2, 0.7, 0.8);
}

fn render_spheres(
  i: usize,
  j: usize,
  buffer: &mut Vec<u8>,
  spheres: &Vec<Sphere>,
  lights: &Vec<Light>,
) {
  let fov = f32::consts::PI / 2.0;
  let x = (2.0 * (i as f32 + 0.5) / WINDOW_WIDTH - 1.0) * (fov / 2.0).tan() * WINDOW_WIDTH
    / WINDOW_HEIGHT;
  let y = -(2.0 * (j as f32 + 0.5) / WINDOW_HEIGHT - 1.0) * (fov / 2.0).tan();
  let dir = Vec3::new(x, y, -1.0).normalize();
  // let sphere = Sphere::new(Vec3::new(-3.0, 0.0, -16.0), 2.0);
  // let sphere = Sphere::new(Vec3::new(-3.0, 0.0, -16.0), 2.0);
  let mut c = cast_ray(&Vec3::zero(), &dir, &spheres, lights);
  let max = c.x.max(c.y).max(c.z);
  if max > 1.0 {
    c /= max;
  }

  let color = c * 255.0;
  let idx = (j as usize * WINDOW_WIDTH as usize + i as usize) * 3;
  buffer[idx] = (color.x as u8).min(255);
  buffer[idx + 1] = (color.y as u8).min(255);
  buffer[idx + 2] = (color.z as u8).min(255);
}

fn test_render(i: usize, j: usize, buffer: &mut Vec<u8>) {
  let idx = (j as usize * WINDOW_WIDTH as usize + i as usize) * 3;
  buffer[idx] = (j as f32 / WINDOW_HEIGHT * 255.0) as u8;
  buffer[idx + 1] = (i as f32 / WINDOW_WIDTH * 255.0) as u8;
  buffer[idx + 2] = 0;
}

fn main_() {
  let sandbox = sandbox::Sandbox::new(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32, false);
  let draw_image = sandbox.make_draw_image();
  let mut periodic_intensity = 0f32;
  sandbox.run_fltk(move |_| {
    let mut buffer: Vec<u8> = vec![0; WINDOW_WIDTH as usize * WINDOW_HEIGHT as usize * 3];
    let mut spheres: Vec<Sphere> = vec![];
    let mut lights = vec![];
    let ivory = Material::new(Vec3::new(0.4, 0.4, 0.3), Vec2::new(0.6, 0.3), 50.);
    let red_rubber = Material::new(Vec3::new(0.3, 0.1, 0.1), Vec2::new(0.9, 0.1), 10.);

    lights.push(Light::new(
      Vec3::new(-20.0, 20.0, 20.0),
      1.5 * periodic_intensity.cos(),
    ));
    lights.push(Light::new(
      Vec3::new(30.0, 50.0, -25.0),
      1.8 * periodic_intensity.cos(),
    ));
    lights.push(Light::new(
      Vec3::new(30.0, 20.0, 30.0),
      1.7 * periodic_intensity.cos(),
    ));

    spheres.push(Sphere::new(Vec3::new(-3.0, 0.0, -16.0), 2.0, ivory));
    spheres.push(Sphere::new(Vec3::new(-1.0, -1.5, -12.0), 2.0, red_rubber));
    spheres.push(Sphere::new(Vec3::new(1.5, -0.5, -18.0), 3.0, red_rubber));
    spheres.push(Sphere::new(Vec3::new(7.0, 5.0, -18.0), 4.0, ivory));

    for j in (0..WINDOW_HEIGHT as usize).step_by(1) {
      for i in (0..WINDOW_WIDTH as usize).step_by(1) {
        // test_render(i, j, &mut buffer);
        render_spheres(i, j, &mut buffer, &spheres, &lights);
      }
    }
    periodic_intensity += f32::consts::PI / 2.0;
    dbg!(periodic_intensity);
    draw_image.as_ref()(&buffer);
  });
}

fn ray_color(r: &Ray, sphere: &Sphere) -> Vec3 {
  if let Some(first_intersect) = sphere.hit(r) {
    let normal = (r.at(first_intersect) - sphere.center).normalize();
    // map normal from [-1,1] to [0,1]
    return Vec3::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) * 0.5;
    // return Vec3::new(1.0, 0.0, 0.0);
  }

  let unit_dir = r.direction.normalize();
  let a = 0.5 * (unit_dir.y + 1.0);

  return Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a;
}

fn main() {
  let aspect_ratio: f32 = 16.0 / 9.0;
  let image_width = 400;
  let camera_center = Vec3::zero();

  let image_height = (image_width as f32 / aspect_ratio) as i32;
  let image_height = if image_height < 1 { 1 } else { image_height };

  let sandbox = sandbox::Sandbox::new(image_width, image_height, false);
  let draw_image = sandbox.make_draw_image();

  let focal_length = 1.0f32;
  let viewport_height = 2.0f32;
  let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

  // see ![viewport](../images/viewport.jpg)
  let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
  let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

  let pixel_delta_u = viewport_u / (image_width as f32);
  let pixel_delta_v = viewport_v / (image_height as f32);

  let viewport_upper_left =
    camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
  let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

  sandbox.run_fltk(move |_| {
    let mut buffer: Vec<u8> = vec![0; image_width as usize * image_height as usize * 3];
    let ivory = Material::new(Vec3::new(0.4, 0.4, 0.3), Vec2::new(0.6, 0.3), 50.);

    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, ivory);

    for j in 0..image_height {
      for i in 0..image_width {
        let pixel_center =
          pixel00_loc + (pixel_delta_u * (i as f32)) + (pixel_delta_v * (j as f32));
        let ray_direction = pixel_center - camera_center;

        let r = Ray::new(camera_center, ray_direction);
        let color = ray_color(&r, &sphere) * 255.0;

        let idx = (j as usize * image_width as usize + i as usize) * 3;
        buffer[idx] = (color.x as u8).min(255);
        buffer[idx + 1] = (color.y as u8).min(255);
        buffer[idx + 2] = (color.z as u8).min(255);
      }
    }
    draw_image.as_ref()(&buffer);
  })
}
