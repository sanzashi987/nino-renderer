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
    return Vec3::new(0.4, 0.4, 0.3) * 255.0;
  }
  return Vec3::new(0.2, 0.7, 0.8) * 255.0;
}

fn render_sphere(i: usize, j: usize, buffer: &mut Vec<u8>) {
  let fov = 1.7f32;
  let x = (2.0 * (i as f32 + 0.5) / WINDOW_WIDTH - 1.0) * (fov / 2.0).tan() * WINDOW_WIDTH
    / WINDOW_HEIGHT;
  let y = -(2.0 * (j as f32 + 0.5) / WINDOW_HEIGHT - 1.0) * (fov / 2.0).tan();
  let dir = Vec3::new(x, y, -1.0).normalize();
  let sphere = Sphere::new(Vec3::new(-3.0, 0.0, -16.0), 2.0);
  let color = cast_ray(&Vec3::zero(), &dir, &sphere);
  let idx = (j as usize * WINDOW_WIDTH as usize + i as usize) * 3;
  buffer[idx] = color.x as u8;
  buffer[idx + 1] = color.y as u8;
  buffer[idx + 2] = color.z as u8;
}

fn test_render(i: usize, j: usize, buffer: &mut Vec<u8>) {
  let idx = (j as usize * WINDOW_WIDTH as usize + i as usize) * 3;
  buffer[idx] = (j as f32 / WINDOW_HEIGHT * 255.0) as u8;
  buffer[idx + 1] = (i as f32 / WINDOW_WIDTH * 255.0) as u8;
  buffer[idx + 2] = 0;
}

fn main() {
  let sandbox = sandbox::Sandbox::new(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32, false);
  let draw_image = sandbox.make_draw_image();

  sandbox.run_fltk(move |_| {
    let mut buffer: Vec<u8> = vec![0; WINDOW_WIDTH as usize * WINDOW_HEIGHT as usize * 3];
    for j in (0..WINDOW_HEIGHT as usize).step_by(1) {
      for i in (0..WINDOW_WIDTH as usize).step_by(1) {
        // test_render(i, j, &mut buffer);
        render_sphere(i, j, &mut buffer);
      }
    }
    draw_image.as_ref()(&buffer);
  });
}
