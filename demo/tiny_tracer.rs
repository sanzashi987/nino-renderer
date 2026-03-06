use math::Vec3;
use tinytracer::object::sphere::Sphere;

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

fn cast_ray(origin: &Vec3, dir: &Vec3, sphere: &Sphere) -> Vec3 {
  if let Some(distance) = sphere.ray_intersect(origin, origin) {
    return Vec3::new(0.4, 0.4, 0.3);
  }
  return Vec3::new(0.2, 0.7, 0.8);
}

fn main() {
  let sandbox = sandbox::Sandbox::new(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32, false);
  let draw_image = sandbox.make_draw_image();

  sandbox.run_fltk(move |_| {
    let mut buffer: Vec<u8> = vec![0; WINDOW_WIDTH as usize * WINDOW_HEIGHT as usize * 3];
    for j in (0..WINDOW_HEIGHT as usize).step_by(1) {
      for i in (0..WINDOW_WIDTH as usize).step_by(1) {
        let idx = (j as usize * WINDOW_WIDTH as usize + i as usize) * 3;
        buffer[idx] = (j as f32 / WINDOW_HEIGHT * 255.0) as u8;
        buffer[idx + 1] = (i as f32 / WINDOW_WIDTH * 255.0) as u8;
        buffer[idx + 2] = 0;
      }
    }
    draw_image.as_ref()(&buffer);
  });
}
