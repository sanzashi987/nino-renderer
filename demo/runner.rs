// use fltk::macros::window;
// use image;
// use nino_renderer::bresenham_line;
use fltk::{self, app::set_visual, enums::Mode, prelude::*, window::Window};
use nino_renderer::cpu_renderer::{self, Renderer};
use nino_renderer::math::{self, Mat4, Vec3, Vec4};
use nino_renderer::renderer::{ATTR_COLOR, ATTR_TEXCOORD};
use nino_renderer::texture::{self, TextureStore};
use nino_renderer::shader::{Attributes, Vertex};
use nino_renderer::{camera, gpu_renderer, renderer};

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

fn create_renderer(w: u32, h: u32, camera: camera::Camera) -> Box<dyn renderer::RendererInterface> {
  if cfg!(feature = "cpu") {
    print!("use cpu renderer");
    Box::new(cpu_renderer::Renderer::new(w, h, camera))
  } else {
    print!("use gpu renderer");
    Box::new(gpu_renderer::Renderer::new(w, h, camera))
  }
}

fn draw_image(renderer: &mut Box<dyn renderer::RendererInterface>) {
  let pixels_buffer = renderer.get_frame_image();

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
    100.0,
    WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
    50f32.to_radians(),
  );
  let mut renderer = create_renderer(WINDOW_WIDTH, WINDOW_HEIGHT, camera);

  let mut attr1 = Attributes::default();
  let mut attr2 = Attributes::default();
  let mut attr3 = Attributes::default();
  let mut attr4 = Attributes::default();
  attr1.set_vec4(ATTR_COLOR, math::Vec4::new(1.0, 1.0, 1.0, 1.0));
  attr2.set_vec4(ATTR_COLOR, math::Vec4::new(1.0, 1.0, 1.0, 1.0));
  attr3.set_vec4(ATTR_COLOR, math::Vec4::new(1.0, 1.0, 1.0, 1.0));
  attr4.set_vec4(ATTR_COLOR, math::Vec4::new(1.0, 1.0, 1.0, 1.0));
  attr1.set_vec2(ATTR_TEXCOORD, math::Vec2::new(0.0, 1.0));
  attr2.set_vec2(ATTR_TEXCOORD, math::Vec2::new(1.0, 1.0));
  attr3.set_vec2(ATTR_TEXCOORD, math::Vec2::new(0.0, 0.0));
  attr4.set_vec2(ATTR_TEXCOORD, math::Vec2::new(1.0, 0.0));

  let vertices = [
    Vertex::new(&math::Vec3::new(-1.0, 1.0, 0.0), attr1),
    Vertex::new(&math::Vec3::new(1.0, 1.0, 0.0), attr2),
    Vertex::new(&math::Vec3::new(-1.0, -1.0, 0.0), attr3),
    Vertex::new(&math::Vec3::new(1.0, 1.0, 0.0), attr2),
    Vertex::new(&math::Vec3::new(-1.0, -1.0, 0.0), attr3),
    Vertex::new(&math::Vec3::new(1.0, -1.0, 0.0), attr4),
  ];
  let model = math::apply_translate(&math::Vec3::new(0.0, 0.0, -4.0));

  run_fltk(move |window| {
    renderer.clear(&Vec4::new(0.0, 0.0, 0.0, 1.0));
    let mut texture_store = TextureStore::default();
    let store_ref = &mut texture_store;
    let texture_id = store_ref.load("./resources/plane/pic.jpg", "test").unwrap();

    let texture = store_ref.get_by_id(texture_id);

    // // SRT
    let model = model * math::apply_eular_rotate_y(rotation.to_radians());

    renderer.draw_triangle(&model, &vertices, 2, texture);
    rotation += 1.0;

    draw_image(&mut renderer);
  });
}
