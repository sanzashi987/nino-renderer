use fltk::{self, app::set_visual, enums::Mode, prelude::*, window::Window};
use nino_renderer::cpu_renderer::{self};
use nino_renderer::math::{self, Vec4};
use nino_renderer::renderer::{texture_sample, RendererInterface};
use nino_renderer::shader::{self, Attributes, Vertex};
use nino_renderer::texture::TextureStore;
use nino_renderer::{camera, gpu_renderer};

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 720;

const ATTR_COLOR: usize = 0;
const ATTR_TEXCOORD: usize = 1;
const UNIFORM_TEXTURE: u32 = 0;

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

fn create_renderer(w: u32, h: u32, camera: camera::Camera) -> Box<dyn RendererInterface> {
  if cfg!(feature = "cpu") {
    print!("use cpu renderer");
    Box::new(cpu_renderer::Renderer::new(w, h, camera))
  } else {
    print!("use gpu renderer");
    Box::new(gpu_renderer::Renderer::new(w, h, camera))
  }
}

fn draw_image(renderer: &mut Box<dyn RendererInterface>) {
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
  let mut texture_store = TextureStore::default();
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

  // let store_ref = &mut texture_store;
  let texture_id = texture_store
    .load("./resources/plane/pic.jpg", "test")
    .unwrap();

  renderer
    .get_uniforms()
    .texture
    .insert(UNIFORM_TEXTURE, texture_id);

  let shader = renderer.get_shader();

  shader.vertex_shading = Box::new(|v, _, _| *v);
  shader.fragment_shading = Box::new(|a, u, t| {
    let frag_color = a.vec4[ATTR_COLOR];
    let textcoord = a.vec2[ATTR_TEXCOORD];
    let texture = t.get_by_id(u.texture[&UNIFORM_TEXTURE]).unwrap();
    let texture_color = texture_sample(texture, &textcoord);
    texture_color * frag_color
  });

  run_fltk(move |window| {
    renderer.clear(&Vec4::new(0.0, 0.0, 0.0, 1.0));

    // let texture = texture_store.get_by_id(texture_id);

    // // SRT
    let model = model * math::apply_eular_rotate_y(rotation.to_radians());

    renderer.draw_triangle(&model, &vertices, 2, &texture_store);
    rotation += 1.0;

    draw_image(&mut renderer);
  });
}
