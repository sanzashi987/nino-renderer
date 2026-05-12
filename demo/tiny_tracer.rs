use math::Vec3;
use tinytracer::object::{
  camera::Camera,
  light::{self, Light},
  material::{Lambertian, Material, Metal},
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
  // let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
  // let ground = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
  // world.add(Box::new(sphere));
  // world.add(Box::new(ground));

  let material_ground = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
  let material_center = Lambertian::new(Vec3::new(0.1, 0.2, 0.5));
  let material_left = Metal::new(Vec3::new(0.8, 0.8, 0.8));
  let material_right = Metal::new(Vec3::new(0.8, 0.6, 0.2));

  world.add(Box::new(Sphere::new(
    Vec3::new(0.0, -100.5, -1.0),
    100.0,
    material_ground,
  )));
  world.add(Box::new(Sphere::new(
    Vec3::new(0.0, 0.0, -1.2),
    0.5,
    material_center,
  )));
  world.add(Box::new(Sphere::new(
    Vec3::new(-1.0, 0.0, -1.0),
    0.5,
    material_left,
  )));
  world.add(Box::new(Sphere::new(
    Vec3::new(1.0, 0.0, -1.0),
    0.5,
    material_right,
  )));

  // Camera

  sandbox.run_fltk(move |_| {
    let mut buffer: Vec<u8> = vec![0; image_width as usize * camera.image_height() as usize * 3];
    camera.render(&world, &mut buffer);
    draw_image.as_ref()(&buffer);
  });

  let a = 1e-8f32;
}
