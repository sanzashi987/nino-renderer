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

fn chapters() -> (Camera, World) {
  let camera = Camera::new(
    IMAGE_WIDTH,
    16.0 / 9.0,
    Vec3::new(-2.0, 2.0, 1.0),
    20.0,
    Vec3::new(0.0, 0.0, -1.0),
  );
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

fn chapter_12_test() -> (Camera, World) {
  let camera = Camera::new(
    IMAGE_WIDTH,
    16.0 / 9.0,
    Vec3::zero(),
    90.0,
    Vec3::new(0.0, 0.0, -1.0),
  );

  let mut world = World::new();

  let rr = (PI / 4.0).cos();
  let material_left = Lambertian::new(Vec3::new(0.0, 0.0, 1.0));
  let material_right = Lambertian::new(Vec3::new(1.0, 0.0, 0.0));

  world.add(Box::new(Sphere::new(
    Vec3::new(rr * -1.0, 0.0, -1.0),
    rr,
    material_left,
  )));
  world.add(Box::new(Sphere::new(
    Vec3::new(rr, 0.0, -1.0),
    rr,
    material_right,
  )));
  return (camera, world);
}

fn random_f32() -> f32 {
  rand::random::<f32>()
}

fn cover() -> (Camera, World) {
  let camera = Camera::new(
    IMAGE_WIDTH,
    16.0 / 9.0,
    Vec3::new(13.0, 2.0, 3.0),
    20.0,
    Vec3::new(0.0, 0.0, 0.0),
  );
  let mut world = World::new();
  let ground_material = Lambertian::new(Vec3::new(0.5, 0.5, 0.5));
  world.add(Box::new(Sphere::new(
    Vec3::new(0.0, -1000.0, 0.0),
    1000.0,
    ground_material,
  )));

  let x = Vec3::new(4.0, 0.2, 0.0);
  for a in -11..11 {
    for b in -11..11 {
      let choose_mat: f32 = random_f32();
      let center = Vec3::new(
        a as f32 + 0.9 * random_f32(),
        0.2,
        b as f32 + 0.9 * random_f32(),
      );

      if (center - x).length() > 0.9 {
        if choose_mat < 0.8 {
          // diffuse
          let albedo = Vec3::new(
            random_f32() * random_f32(),
            random_f32() * random_f32(),
            random_f32() * random_f32(),
          );
          let sphere_material = Lambertian::new(albedo);
          world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
        } else if choose_mat < 0.95 {
          // metal
          let albedo = Vec3::random_with_range(0.5, 1.0);
          let fuzz = random_f32();
          let sphere_material = Metal::new(albedo, fuzz);
          world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
        } else {
          // glass
          let sphere_material = Dielectric::new(1.5);
          world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
        }
      }
    }
  }

  let material1 = Dielectric::new(1.5);
  world.add(Box::new(Sphere::new(
    Vec3::new(0.0, 1.0, 0.0),
    1.0,
    material1,
  )));

  let material2 = Lambertian::new(Vec3::new(0.4, 0.2, 0.1));
  world.add(Box::new(Sphere::new(
    Vec3::new(-4.0, 1.0, 0.0),
    1.0,
    material2,
  )));

  let material3 = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0);
  world.add(Box::new(Sphere::new(
    Vec3::new(4.0, 1.0, 0.0),
    1.0,
    material3,
  )));

  (camera, world)
}

fn main() {
  // let (camera, world) = chapters();
  let (camera, world) = cover();
  // let (camera, world) = chapter_12_test();
  let sandbox = sandbox::Sandbox::new(IMAGE_WIDTH, camera.image_height(), false);
  let draw_image = sandbox.make_draw_image();

  sandbox.run_fltk(move |_| {
    let mut buffer: Vec<u8> = vec![0; IMAGE_WIDTH as usize * camera.image_height() as usize * 3];
    camera.render(&world, &mut buffer);
    draw_image.as_ref()(&buffer);
  });
}
