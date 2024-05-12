// use fltk::macros::window;
// use image;
// use nino_renderer::bresenham_line;
use fltk::{self, app::set_visual, enums::Mode, prelude::*, window::Window};
use nino_renderer::math::{self, Mat4, Vec3, Vec4};
use nino_renderer::renderer::Renderer;
use nino_renderer::{camera, renderer};

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 720;

fn run_fltk<F: FnMut(&mut Window) + 'static>(cb: F) {
  let app = fltk::app::App::default();

  let mut window = Window::new(
    100,
    100,
    WINDOW_WIDTH as i32,
    WINDOW_HEIGHT as i32,
    "sandbox",
  );

  window.draw(cb);

  window.handle(move |_, event| false);
  window.end();
  set_visual(Mode::Rgb).unwrap();
  window.show();

  fltk::app::add_idle3(move |_| {
    window.redraw();
  });

  app.run().unwrap();
}

// fn create_window() -> Window {}

fn draw_image(renderer: &renderer::Renderer) {
  let pixels_buffer = renderer.get_pixiel();

  fltk::draw::draw_image(
    pixels_buffer,
    0,
    0,
    renderer.get_canvas_width() as i32,
    renderer.get_canvas_height() as i32,
    fltk::enums::ColorDepth::Rgb8,
  )
  .unwrap();
}

fn main() {
  // let mut img = image::ImageBuffer::new(100, 100);

  let mut rotation = 0.0f32;

  let camera = camera::Camera::new(
    1.0,
    WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
    50f32.to_radians(),
  );
  let mut renderer = Renderer::new(WINDOW_WIDTH, WINDOW_HEIGHT, camera);
  let color = Vec4::new(0.0, 1.0, 10.0, 1.0);

  let vertices = [
    Vec3::new(-1.0, 1.0, 0.0),
    Vec3::new(1.0, 1.0, 0.0),
    Vec3::new(0.0, -1.0, 0.0),
  ];
  run_fltk(move |window| {
    renderer.clear(&Vec4::new(0.0, 0.0, 0.0, 1.0));
    // let model = Mat4::identity();
    // // SRT
    let model = math::apply_translate(&math::Vec3::new(0.0, 0.0, -4.0))
      * math::apply_eular_rotate_y(rotation.to_radians());

    renderer.draw_triangle(&model, &vertices, &color);
    rotation += 1.0;

    draw_image(&renderer)
  });
}
