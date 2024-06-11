use fltk::{self, app::set_visual, enums::Mode, prelude::*, text, window::Window};

use nino_renderer::{
  camera::{self, Camera},
  cpu_renderer, gpu_renderer,
  math::{self, Vec4},
  model::{self, Mesh},
  renderer::{texture_sample, RendererInterface},
  shader::{Attributes, Vertex},
  texture::{self, TextureStore},
};

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 768;

// attribute
const ATTR_TEXCOORD: usize = 0; // vec2
const ATTR_NORMAL: usize = 0; // vec3

// uniform
const UNIFORM_COLOR: u32 = 1;
const UNIFORM_TEXTURE: u32 = 1;

fn create_renderer(w: u32, h: u32, camera: camera::Camera) -> Box<dyn RendererInterface> {
  if cfg!(feature = "cpu") {
    print!("using scanline rendering");
    Box::new(cpu_renderer::Renderer::new(w, h, camera))
  } else {
    print!("using barycentric coordinate rendering");
    Box::new(gpu_renderer::Renderer::new(w, h, camera))
  }
}

const RESOURCE_PATH: &str = "./resources";
const FOLDER: &str = "plane";

fn get_resource_filepath(relative: &str) -> String {
  format!("{}/{}/{}", RESOURCE_PATH, FOLDER, relative)
}

struct StructedModelData {
  vertices: Vec<Vertex>,
  mtllib: Option<u32>,
  material: Option<String>,
}

fn construct_model_data(meshes: &[Mesh]) -> Vec<StructedModelData> {
  let mut data = Vec::<StructedModelData>::new();

  for mesh in meshes {
    let mut vertices = Vec::<Vertex>::new();
    // convert model vertex to shader vertex
    for vertex in &mesh.vertices {
      let mut attr = Attributes::default();

      attr.set_vec2(ATTR_TEXCOORD, vertex.textcoord);
      attr.set_vec3(ATTR_NORMAL, vertex.normal);

      let shader_vertex = Vertex::new(&vertex.position, attr);
      vertices.push(shader_vertex);
    }

    data.push(StructedModelData {
      vertices,
      material: mesh.material.clone(),
      mtllib: mesh.mtllib,
    })
  }

  data
}

fn run_fltk<F: FnMut(&mut Window) + 'static>(cb: F) {
  let app = fltk::app::App::default();

  let mut window = Window::new(
    100,
    100,
    WINDOW_WIDTH as i32,
    WINDOW_HEIGHT as i32,
    "runner",
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
  let mut texture_store = TextureStore::default();
  let mut rotation = 0.0f32;

  let camera = Camera::new(
    1.0,
    1000.0,
    WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
    30f32.to_radians(),
  );

  let mut renderer = create_renderer(WINDOW_WIDTH, WINDOW_HEIGHT, camera);

  let (meshes, mtllibs) = model::load_from_file(
    &get_resource_filepath("plane.obj"),
    model::PreOperation::None,
  )
  .unwrap();

  let model_structures = construct_model_data(&meshes);

  for mtllib in &mtllibs {
    for (_, val) in mtllib.materials.iter() {
      if let Some(path) = &val.texture_maps.diffuse {
        texture_store
          .load(&get_resource_filepath(path), path)
          .unwrap();
      }
    }
  }

  for structure in &model_structures {
    let uniforms = renderer.get_uniforms();
    if structure.mtllib.is_some() && structure.material.is_some() {
      let mtllib = &mtllibs[structure.mtllib.unwrap() as usize];
      if let Some(material) = mtllib.materials.get(&structure.material.clone().unwrap()) {
        if let Some(ambient) = material.ambient {
          uniforms
            .vec4
            .insert(UNIFORM_COLOR, Vec4::from_vec3(&ambient, 1.0));
        }

        if let Some(diffuse) = &material.texture_maps.diffuse {
          uniforms
            .texture
            .insert(UNIFORM_TEXTURE, *texture_store.get_id(diffuse).unwrap());
        }
      }
    }
  }

  let shader = renderer.get_shader();

  shader.vertex_shading = Box::new(|v, _, _| *v);
  shader.fragment_shading = Box::new(|a, u, t| {
    let mut frag_color = match u.vec4.get(&UNIFORM_COLOR) {
      Some(v4) => *v4,
      None => Vec4::new(1.0, 1.0, 1.0, 1.0),
    };
    let textcoord = a.vec2[ATTR_TEXCOORD];

    if let Some(texture_id) = u.texture.get(&UNIFORM_TEXTURE) {
      if let Some(texture) = t.get_by_id(*texture_id) {
        frag_color = texture_sample(texture, &textcoord) * frag_color;
      }
    }

    frag_color
  });

  run_fltk(move |_| {
    renderer.clear(&Vec4::new(0.0, 0.0, 0.0, 1.0));

    // let texture = texture_store.get_by_id(texture_id);

    // // SRT
    let model = math::apply_translate(&math::Vec3::new(0.0, 0.0, -4.0))
      * math::apply_eular_rotate_y(rotation.to_radians());

    for data in &model_structures {
      renderer.draw_triangle(&model, &data.vertices, &texture_store);
    }

    rotation += 1.0;

    draw_image(&mut renderer);
  });
}
