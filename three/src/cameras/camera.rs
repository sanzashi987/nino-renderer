use crate::{core::object_3d::IObject3D, math::Mat4};

pub trait ICamera: IObject3D {
  fn view_matrix(&self) -> Mat4;

  fn update_projection_matrix(&self);

  fn projection_matrix(&self) -> Mat4;

  fn project_matrix_inverse(&self) -> Mat4 {
    self.projection_matrix().inverse().unwrap()
  }
}

pub struct View {
  pub enabled: bool,
  pub full_width: f32,
  pub full_height: f32,
  pub offset_x: f32,
  pub offset_y: f32,
  pub width: f32,
  pub height: f32,
}

macro_rules! derive_view_matrix {
  ($instance:ident) => {
    $instance.update_projection_matrix();
    let this = $instance.clone();
    let mut event_emitter = this.event_emitter.borrow_mut();
    let that = $instance.clone();
    event_emitter.on(
      "update:global_matrix",
      Box::new(move |x| {
        if let Ok(global_matrix) = x.downcast::<Mat4>() {
          let mut mutator = that.view_matrix.borrow_mut();
          *mutator = global_matrix.inverse().unwrap();
        }
      }),
    );
  };
}

pub(crate) use derive_view_matrix;
