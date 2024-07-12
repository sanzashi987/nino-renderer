const RESOURCE_PATH: &str = "./resources";
const FOLDER: &str = "african";
const MODEL: &str = "head.obj";
// const FOLDER: &str = "Red";
// const MODEL: &str = "Red.obj";
// const FOLDER: &str = "plane";
// const MODEL: &str = "plane.obj";

// use rand;

use tinyrenderer::{
  bresenham_line::line,
  data_array::{ColorBuffer, DepthBuffer},
  math::{Vec2, Vec3, Vec4},
  obj_loader::{load_obj, Face, ParserMode, Scene},
  shade_triangle::shade_triangle,
};

fn get_resource_filepath(relative: &str) -> String {
  format!("{}/{}/{}", RESOURCE_PATH, FOLDER, relative)
}

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 800.0;
const HALF_WIDTH: f32 = (WINDOW_WIDTH - 1.0) / 2.0;
const HALF_HEIGHT: f32 = (WINDOW_HEIGHT - 1.0) / 2.0;

/**
 * lesson 1
 */
fn static_wireframe(scene: &Scene, face: &Face, color_buffer: &mut ColorBuffer) {
  let vertices = &scene.vertices;

  for i in 0..3 {
    let i0 = face.vertices[i].vertex_index;
    let i1 = face.vertices[(i + 1) % 3].vertex_index;

    let v0 = vertices[i0 as usize];
    let v1 = vertices[i1 as usize];

    let pt0 = Vec2::new((v0.x + 1.0) * HALF_WIDTH, (v0.y + 1.0) * HALF_HEIGHT);
    let pt1 = Vec2::new((v1.x + 1.0) * HALF_WIDTH, (v1.y + 1.0) * HALF_HEIGHT);

    line(pt0, pt1, color_buffer);
  }
}

/**
 * lesson 2 & 3
 */
fn direct_light_shading(
  scene: &Scene,
  face: &Face,
  color_buffer: &mut ColorBuffer,
  depth_buffer: &mut DepthBuffer,
) {
  let vertices = &scene.vertices;
  let textures = &scene.texture_coordinates;

  let v0 = vertices[face.vertices[0].vertex_index as usize];
  let v1 = vertices[face.vertices[1].vertex_index as usize];
  let v2 = vertices[face.vertices[2].vertex_index as usize];

  let t0 = textures[face.vertices[0].texture_index.unwrap() as usize];
  let t1 = textures[face.vertices[1].texture_index.unwrap() as usize];
  let t2 = textures[face.vertices[2].texture_index.unwrap() as usize];

  let (v01, v02) = (v1 - v0, v2 - v0);
  let face_normal = v02.cross(&v01).normalize();

  let direct_light = Vec3::new(0.0, 0.0, 1.0);

  // println!("{:?}", face_normal);
  let light_intense = direct_light.dot(&face_normal);

  let pt0 = Vec3::new((v0.x + 1.0) * HALF_WIDTH, (v0.y + 1.0) * HALF_HEIGHT, v0.z);
  let pt1 = Vec3::new((v1.x + 1.0) * HALF_WIDTH, (v1.y + 1.0) * HALF_HEIGHT, v1.z);
  let pt2 = Vec3::new((v2.x + 1.0) * HALF_WIDTH, (v2.y + 1.0) * HALF_HEIGHT, v2.z);

  let mut points = [pt0, pt1, pt2];

  // let light_intense = (v0.z + v1.z + v2.z) / 3.0;

  // let a = rand::random::<f32>();

  // println!("{}", light_intense);
  if light_intense > 0.0 {
    shade_triangle(
      &mut points,
      depth_buffer,
      color_buffer,
      &Vec4::new(
        light_intense, //rand::random::<f32>(),
        light_intense, //rand::random::<f32>(),
        light_intense, //rand::random::<f32>(),
        1.0,
      ),
    )
  }
}

fn main() {
  let relative_path = get_resource_filepath(MODEL);

  let mut res = load_obj(&relative_path, ParserMode::Lazy).unwrap();

  let scene = res.get_result().unwrap();

  let mut color_buffer = ColorBuffer::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);
  let mut depth_buffer = DepthBuffer::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);

  depth_buffer.clear(std::f32::MIN);

  let sandbox = sandbox::Sandbox::new(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32, false);
  let draw_image = sandbox.make_draw_image();

  let tga = &get_resource_filepath("african_head_diffuse.tga");
  let tag_path = std::path::Path::new(tga);
  let _ = scene.textures.load(tag_path, "african_head_diffuse");

  for model in &scene.models {
    for face in &model.faces {
      // static_wireframe(&mut scene, face, &mut color_buffer);
      // println!("{:?}", face);
      direct_light_shading(scene, face, &mut color_buffer, &mut depth_buffer);
    }
  }

  sandbox.run_fltk(move |_| draw_image.as_ref()(color_buffer.data()));

  // println!("{:?}", scene);
}
