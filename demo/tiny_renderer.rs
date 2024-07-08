const RESOURCE_PATH: &str = "./resources";
const FOLDER: &str = "african";
const MODEL: &str = "head.obj";

use tinyrenderer::{
  bresenham_line::line,
  data_array::ColorBuffer,
  math::Vec2,
  obj_loader::{load_obj, ParserMode},
};

fn get_resource_filepath(relative: &str) -> String {
  format!("{}/{}/{}", RESOURCE_PATH, FOLDER, relative)
}

const WINDOW_WIDTH: f32 = 1080.0;
const WINDOW_HEIGHT: f32 = 1080.0;
const HALF_WIDTH: f32 = (WINDOW_WIDTH - 1.0) / 2.0;
const HALF_HEIGHT: f32 = (WINDOW_HEIGHT - 1.0) / 2.0;

fn main() {
  let relative_path = get_resource_filepath(MODEL);

  let mut res = load_obj(&relative_path, ParserMode::Lazy).unwrap();

  let scene = res.get_result().unwrap();

  let mut color_buffer = ColorBuffer::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);

  let vertices = &scene.vertices;

  for model in &scene.models {
    for face in &model.faces {
      for i in 0..3 {
        let i0 = face.vertices[i].vertex_index;
        let i1 = face.vertices[(i + 1) % 3].vertex_index;

        let v0 = vertices[i0 as usize];
        let v1 = vertices[i1 as usize];

        // let pt0 = Vec2::new(, y);
        let pt0 = Vec2::new((v0.x + 1.0) * HALF_WIDTH, (v0.y + 1.0) * HALF_HEIGHT);
        let pt1 = Vec2::new((v1.x + 1.0) * HALF_WIDTH, (v1.y + 1.0) * HALF_HEIGHT);

        line(pt0, pt1, &mut color_buffer);
      }
    }
  }

  let sandbox = sandbox::Sandbox::new(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32, false);
  let draw_image = sandbox.make_draw_image();

  sandbox.run_fltk(move |_| draw_image.as_ref()(color_buffer.data()));

  // println!("{:?}", scene);
}
