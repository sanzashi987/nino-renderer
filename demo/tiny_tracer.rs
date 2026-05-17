use std::f32::consts::PI;

use math::Vec3;
use tinytracer::object::{
  camera::Camera,
  light::{self, Light},
  material::{Dielectric, Lambertian, Material, Metal},
  sphere::Sphere,
  world::World,
};
const IMAGE_WIDTH: i32 = 450;

fn chapter_11() -> (Camera, World) {
  let camera = Camera::new(IMAGE_WIDTH, 16.0 / 9.0, Vec3::zero(), 90.0);
  // World
  let mut world = World::new();
  // let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
  // let ground = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
  // world.add(Box::new(sphere));
  // world.add(Box::new(ground));

  let material_ground = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
  let material_center = Lambertian::new(Vec3::new(0.1, 0.2, 0.5));
  // let material_left = Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3);
  let material_left = Dielectric::new(1.5);
  // let material_left = Dielectric::new(1.0/1.33);
  let material_bubble = Dielectric::new(1.0 / 1.5);
  let material_right = Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0);

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
    Vec3::new(-1.0, 0.0, -1.0),
    0.4,
    material_bubble,
  )));
  world.add(Box::new(Sphere::new(
    Vec3::new(1.0, 0.0, -1.0),
    0.5,
    material_right,
  )));

  return (camera, world);
}

fn chapter_12() -> (Camera, World) {
  let camera = Camera::new(IMAGE_WIDTH, 16.0 / 9.0, Vec3::zero(), 90.0);

  let mut world = World::new();

  let R = (PI / 4.0).cos();
  let material_left = Lambertian::new(Vec3::new(0.0,0.0, 1.0));
  let material_right = Lambertian::new(Vec3::new(1.0, 0.0, 0.0));

  world.add(Box::new(Sphere::new(Vec3::new(R*-1.0, 0.0, -1.0), R, material_left)));
  world.add(Box::new(Sphere::new(Vec3::new( R, 0.0, -1.0), R, material_right)));
  return (camera, world);
}

fn main() {
  // let (camera, world) = chapter_11();
  let (camera, world) = chapter_12();
  let sandbox = sandbox::Sandbox::new(IMAGE_WIDTH, camera.image_height(), false);
  let draw_image = sandbox.make_draw_image();

  sandbox.run_fltk(move |_| {
    let mut buffer: Vec<u8> = vec![0; IMAGE_WIDTH as usize * camera.image_height() as usize * 3];
    camera.render(&world, &mut buffer);
    draw_image.as_ref()(&buffer);
  });
}
