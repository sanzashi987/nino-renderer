use image::buffer;
use math::{Vec2, Vec3};
use tinytracer::object::{
  light::{self, Light},
  material::Material,
  ray::{Hittable, Ray},
  sphere::Sphere,
  world::World,
};

fn main() {
  // Image
  let aspect_ratio: f32 = 16.0 / 9.0;
  let image_width = 400;
  let camera_center = Vec3::zero();

  let image_height = (image_width as f32 / aspect_ratio) as i32;
  let image_height = if image_height < 1 { 1 } else { image_height };

  let sandbox = sandbox::Sandbox::new(image_width, image_height, false);
  let draw_image = sandbox.make_draw_image();

  // World
  let mut world = World::new();
  let ivory = Material::new(Vec3::new(0.4, 0.4, 0.3), Vec2::new(0.6, 0.3), 50.);
  let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, ivory);
  let ground = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, ivory);

  world.add(Box::new(sphere));
  world.add(Box::new(ground));

  // Camera
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

    for j in 0..image_height {
      for i in 0..image_width {
        let pixel_center =
          pixel00_loc + (pixel_delta_u * (i as f32)) + (pixel_delta_v * (j as f32));
        let ray_direction = pixel_center - camera_center;

        let r = Ray::new(camera_center, ray_direction);
        let color = world.render(&r) * 255.0;

        let idx = (j as usize * image_width as usize + i as usize) * 3;
        buffer[idx] = (color.x as u8).min(255);
        buffer[idx + 1] = (color.y as u8).min(255);
        buffer[idx + 2] = (color.z as u8).min(255);
      }
    }
    draw_image.as_ref()(&buffer);
  })
}
