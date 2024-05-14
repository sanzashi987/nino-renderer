use crate::{
  bresenham_line,
  camera::Camera,
  cohen_sutherland,
  image::ColorAttachment,
  math::{Mat4, Vec2, Vec3, Vec4},
  renderer::{RendererInterface, Viewport},
  scanline,
  vertex::{self, Vertex},
};

pub struct Renderer {
  color_attachment: ColorAttachment,
  viewport: Viewport,
  camera: Camera,
}

impl RendererInterface for Renderer {
  fn clear(&mut self, color: &Vec4) {
    self.color_attachment.clear(color)
  }

  fn get_canvas_width(&self) -> u32 {
    self.color_attachment.width()
  }

  fn get_canvas_height(&self) -> u32 {
    self.color_attachment.height()
  }

  fn get_frame_image(&self) -> &[u8] {
    self.color_attachment.data()
  }

  fn draw_triangle(
    &mut self,
    model: &Mat4,
    vertices: &[Vertex],
    count: u32,
    texture: Option<&image::DynamicImage>,
  ) {
    for i in 0..count {
      let index = (i * 3) as usize;

      let mut vertices = [vertices[index], vertices[index + 1], vertices[index + 2]];

      for v in &vertices {
        v.position = *model * v.position;
      }

      for v in &vertices {
        v.position = *self.camera.get_frustum().get_mat() * v.position;
      }

      for v in &vertices {}

      for v in &vertices {
        v.position.x =
          (v.position.x + 1.0) * 0.5 * (self.viewport.w as f32 - 1.0) + self.viewport.x as f32;
        v.position.y =
          (v.position.y + 1.0) * 0.5 * (self.viewport.w as f32 - 1.0) + self.viewport.x as f32;
      }
      let [trap1, trap2] = scanline::Trapezoid::from_triangle(&vertices);
      // rasterization
      if let Some(trap) = trap1 {
        self.draw_trapezoid(trap, texture);
      }
      if let Some(trap) = trap2 {
        self.draw_trapezoid(trap, texture);
      }
    }

    // let length = vertices.len();
    // for i in 0..length {
    //   let p1 = &vertices[i];
    //   let p2 = &vertices[(i + 1) % length];

    //   self.draw_line(&p1, &p2, texture);
    // }
  }
}

impl Renderer {
  pub fn new(w: u32, h: u32, camera: Camera) -> Self {
    Self {
      camera,
      viewport: Viewport { x: 0, y: 0, w, h },
      color_attachment: ColorAttachment::new(w, h),
    }
  }

  fn draw_line(&mut self, p0: &Vec2, p1: &Vec2, color: &Vec4) {
    // let rect_min = Vec2 { x: 50.0, y: 50.0 };
    let rect_min = Vec2::zero();
    let rect_max = Vec2::new(
      self.color_attachment.width() as f32 - 1.0,
      self.color_attachment.height() as f32 - 1.0,
    );

    let res = cohen_sutherland::clip(&p0, &p1, &rect_min, &rect_max);

    // if let Some((next_p0, next_p1)) = res {}

    match res {
      Some((next_p0, next_p1)) => bresenham_line::draw_line(
        next_p0.x as i32,
        next_p0.y as i32,
        next_p1.x as i32,
        next_p1.y as i32,
        color,
        &mut self.color_attachment,
      ),
      None => {}
    }
  }

  pub fn draw_trapezoid(
    &mut self,
    trap: scanline::Trapezoid,
    texture: Option<&image::DynamicImage>,
  ) {
    let top = trap.top.ceil().max(0.0) as i32;
    let bottom = trap
      .bottom
      .ceil()
      .min(self.color_attachment.height() as f32 - 1.0) as i32
      - 1;
    let mut y = top as f32;

    while y <= bottom as f32 {
      let mut scanline = scanline::Scanline::from_trapezoid(&trap, y);
      self.draw_scanline(&scanline, texture);
      y += 1.0;
    }
  }

  pub fn draw_scanline(
    &mut self,
    scanline: &scanline::Scanline,
    texture: Option<&image::DynamicImage>,
  ) {
    let mut vertex = scanline.vertex;
    let y = scanline.y as u32;
    let mut width = scanline.width;
    let border = self.color_attachment.width() as f32;
    while width > 0.0 {
      let x = &vertex.x;
      if *x >= 0.0 && *x < border {
        self.color_attachment.set(*x as u32, y, color)
      }

      width -= 1.0;
      vertex += scanline.step;
    }
  }
}
