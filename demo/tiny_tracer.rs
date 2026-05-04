use image::buffer;
use math::{Vec2, Vec3};
use tinytracer::object::{
  camera::Camera,
  light::{self, Light},
  material::Material,
  sphere::Sphere,
  world::World,
};

fn main() {
  // Image
  let aspect_ratio: f32 = 16.0 / 9.0;
  let image_width = 400;
  let center = Vec3::zero();

  let camera = Camera::new(image_width, aspect_ratio, center);

  let sandbox = sandbox::Sandbox::new(image_width, camera.image_height(), false);
  let draw_image = sandbox.make_draw_image();

  // World
  let mut world = World::new();
  let ivory = Material::new(Vec3::new(0.4, 0.4, 0.3), Vec2::new(0.6, 0.3), 50.);
  let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, ivory);
  let ground = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, ivory);

  world.add(Box::new(sphere));
  world.add(Box::new(ground));

  // Camera

  sandbox.run_fltk(move |_| {
    let mut buffer: Vec<u8> = vec![0; image_width as usize * camera.image_height() as usize * 3];
    camera.render(&world, &mut buffer);
    draw_image.as_ref()(&buffer);
  })
}
