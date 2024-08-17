const RESOURCE_PATH: &str = "./resources";
const FOLDER: &str = "african_head";
const MODEL: &str = "african_head.obj";

const MODEL_PATH: &str = "./resources/african_head/african_head.obj";
// const FOLDER: &str = "Red";
// const MODEL: &str = "Red.obj";
// const FOLDER: &str = "plane";
// const MODEL: &str = "plane.obj";

// use rand;

use fltk::draw;
use tinyrenderer::{
  bresenham_line::line,
  data_array::{ColorBuffer, DepthBuffer},
  math::{self, Mat4, Vec2, Vec3, Vec4},
  model::{self, from_obj_path, Model, Scene, Vertex},
  obj_loader::material::{self, Material, Texture},
  renderer::{
    renderer::Renderer,
    shader::{gouraud::make_gouraud_shader, phong::make_phong_shader, shadow::make_shadow_shader},
  },
  shade_triangle::shade_triangle_barycentric,
};

macro_rules! file {
  ($name:tt) => {
    &format!("{}/{}/{}", RESOURCE_PATH, FOLDER, $name)
  };
}

// fn get_resource_filepath(relative: &str) -> String {
//   format!("{}/{}/{}", RESOURCE_PATH, FOLDER, relative)
// }

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 800.0;
const HALF_WIDTH: f32 = (WINDOW_WIDTH - 1.0) / 2.0;
const HALF_HEIGHT: f32 = (WINDOW_HEIGHT - 1.0) / 2.0;

/**
 * lesson 1
 */
fn static_wireframe(
  scene: &Scene,
  model: &Model,
  points: &mut [Vertex; 3],
  color_buffer: &mut ColorBuffer,
  depth_buffer: &mut DepthBuffer,
) {
  let vertices = &scene.vertices;

  for i in 0..3 {
    let v0 = points[i].position;
    let v1 = points[(i + 1) % 3].position;

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
  color_buffer: &mut ColorBuffer,
  depth_buffer: &mut DepthBuffer,
) {
  let path_str = file!("african_head_diffuse.tga");
  let path = std::path::Path::new(&path_str);
  let mut texture = Texture::load("233", path, 1).unwrap();

  for model in &scene.models {
    let vertext_number = model.vertices.len();
    for i in 0..vertext_number / 3 {
      let v0 = model.vertices[i * 3 + 0];
      let v1 = model.vertices[i * 3 + 1];
      let v2 = model.vertices[i * 3 + 2];

      let (v01, v02) = (v1.position - v0.position, v2.position - v0.position);
      let face_normal = v02
        .truncated_to_vec3()
        .cross(&(v01.truncated_to_vec3()))
        .normalize();

      let direct_light = Vec3::new(0.0, 0.0, 1.0);

      let light_intense = direct_light.dot(&face_normal);

      let pos0 = Vec4::new(
        (v0.position.x + 1.0) * HALF_WIDTH,
        (v0.position.y + 1.0) * HALF_HEIGHT,
        v0.position.z,
        1.0,
      );
      let pos1 = Vec4::new(
        (v1.position.x + 1.0) * HALF_WIDTH,
        (v1.position.y + 1.0) * HALF_HEIGHT,
        v1.position.z,
        1.0,
      );
      let pos2 = Vec4::new(
        (v2.position.x + 1.0) * HALF_WIDTH,
        (v2.position.y + 1.0) * HALF_HEIGHT,
        v2.position.z,
        1.0,
      );

      let pt0 = Vertex::new(pos0, v0.normal, v0.texture);
      let pt1 = Vertex::new(pos1, v1.normal, v1.texture);
      let pt2 = Vertex::new(pos2, v2.normal, v2.texture);

      let mut points = [pt0, pt1, pt2];

      if light_intense > 0.0 {
        shade_triangle_barycentric(
          &mut points,
          depth_buffer,
          color_buffer,
          &mut texture,
          &Vec4::new(
            light_intense, //rand::random::<f32>(),
            light_intense, //rand::random::<f32>(),
            light_intense, //rand::random::<f32>(),
            1.0,
          ),
        )
      }
    }
  }
}

/// lesson 4,5,
// fn render_pipeline(texture: &Texture) -> ColorBuffer {}

fn main() {
  // let relative_path = get_resource_filepath(MODEL);

  // let mut res = load_obj(&relative_path).unwrap();
  // let scene = res.parse().unwrap();

  // let scene = from_obj_path(&relative_path).unwrap();

  let sandbox = sandbox::Sandbox::new(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32, false);
  // let sandbox = sandbox::Sandbox::new(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32, true);
  let draw_image = sandbox.make_draw_image();

  // let _ = scene.textures.load(tag_path, "african_head_diffuse");

  // for model in &scene.models {
  //   for face in &model.faces {
  //     // static_wireframe(&mut scene, face, &mut color_buffer);
  //     // println!("{:?}", face);
  //     direct_light_shading(scene, face, &mut color_buffer, &mut depth_buffer);
  //   }
  // }

  // direct_light_shading(&scene, &mut color_buffer, &mut depth_buffer);
  // let color_buffer = render_pipeline();

  // lesson 4, 5
  // let mut rotation = -80.0f32;
  let mut rotation = 0.0f32;

  let mut renderer = Renderer::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32);

  renderer.load_texture(file!("african_head_diffuse.tga"), "african_head_diffuse");
  renderer.load_texture(file!("african_head_nm.tga"), "african_head_nm");
  renderer.load_texture(
    file!("african_head_nm_tangent.tga"),
    "african_head_nm_tangent",
  );
  renderer.load_texture(file!("african_head_spec.tga"), "african_head_spec");
  renderer.camera.move_to(Vec3::new(3.0, 3.0, 3.0));
  // renderer.camera.move_to(Vec3::new(5.0, 5.0, 5.0));

  let mut material = Material::default();

  // material.shader = make_gouraud_shader(Vec3::new(1.0, 1.0, 1.0));
  // material.shader = make_phong_shader(Vec3::new(1.0, 1.0, 1.0));
  material.shader = make_shadow_shader();
  renderer.camera.lookat(Vec3::new(0.0, 0.0, 0.0));
  // renderer.camera.set_rotation(Vec3::new(0.0, 0.0, 0.0));

  let scene = from_obj_path(MODEL_PATH).unwrap();

  // sandbox.run_fltk(move |_| draw_image.as_ref()(color_buffer.data()));
  sandbox.run_fltk(move |_| {
    let model = math::apply_translate(&math::Vec3::new(0.0, 0.0, 0.0))
      * math::apply_eular_rotate_y(rotation.to_radians());

    // println!("{:?}", model);

    renderer.render(&scene, model, &material);
    let color = renderer.take_color();
    draw_image.as_ref()(color.data());
    // rotation -= 10.0;
  });

  // println!("{:?}", scene);
}
