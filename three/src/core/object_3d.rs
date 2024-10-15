#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ObjectType {
  Light,
  Mesh,
  Scene,
  Object3D,
  Camera,
  Group,
  Line,
  Point,
}

impl Default for ObjectType {
  fn default() -> Self {
    Self::Object3D
  }
}
pub trait ObjectActions {
  fn parent(&self) -> Option<std::rc::Rc<dyn ObjectActions>>;
  fn set_parent(&self, parent: std::rc::Rc<dyn ObjectActions>);
  fn remove_from_parent(&self);
  fn remove(&self, uuid: &str);
  fn add(&self, val: std::rc::Rc<dyn ObjectActions>);
  fn clear(&self);
  fn attach(&self, child: Box<dyn ObjectActions>);
  fn children(&self) -> std::cell::Ref<'_, Vec<std::rc::Rc<dyn ObjectActions>>>;

  fn look_at(&self, point: crate::math::Vec3);
  fn matrix(&self) -> crate::math::Mat4;
  fn global_matrix(&self) -> crate::math::Mat4;
  fn update_global_matrix(&self);
  fn update_matrix(&self);
  fn compose(&self) -> crate::math::Mat4;
  fn decompose(&self);

  fn apply_matrix(&self, matrix: crate::math::Mat4);
  fn apply_quaternion(&self, matrix: crate::math::Quaternion);

  fn rotate_on_world_axis(&self, axis: crate::math::Vec3, angle: f32);
  fn rotate_on_axis(&self, axis: crate::math::Vec3, angle: f32);
  fn rotate_x(&self, angle: f32);
  fn rotate_y(&self, angle: f32);
  fn rotate_z(&self, angle: f32);

  fn translate_on_axis(&self, axis: crate::math::Vec3, distance: f32);
  fn translate_x(&self, distance: f32);
  fn translate_y(&self, distance: f32);
  fn translate_z(&self, distance: f32);

  fn global_scale(&self) -> crate::math::Vec3;
  fn global_position(&self) -> crate::math::Vec3;
  fn global_rotation(&self) -> crate::math::Rotation;

  fn layers(&self) -> std::cell::Ref<crate::core::layer::Layers>;
  fn test_layers(&self, layers: &crate::core::layer::Layers) -> bool;

  fn visible(&self) -> bool;
  fn get_type(&self) -> ObjectType;

  fn uuid(&self) -> &str;
}

macro_rules! define_support_objects {
  ($enum_name:tt;$($name:tt:$ty:ty),+) => {
    pub enum $enum_name {
      $(
        $name($ty),
      )+
    }
    impl $enum_name {
      #[allow(unused)]
      pub fn convert<T:'static + Sized>(val :T) ->Option<Self>{
        let val_any: Box<dyn std::any::Any> = Box::new(val);
        $(
          let val_any = match val_any.downcast::<$ty>() {
            Ok(matched) =>{
              return Some(Self::$name(*matched));
            },
            Err(instance) =>{
              instance
            }
          };
        )+

        return None;

      }
    }

  };
}

// fn a() {
//   let uid = uuid::Uuid::new_v4().to_string();
// }

macro_rules! with_default_fields {
  ($type:tt;$($val:ident),*) => {{

    let mut this = std::rc::Rc::new(Self {
      $($val,)*
      parent: Default::default(),
      children: Default::default(),
      matrix: Default::default(),
      global_matrix: Default::default(),
      position: Default::default(),
      rotation: Default::default(),
      scale: Default::default(),
      visible: Default::default(),
      layers: Default::default(),
      cast_shadow: Default::default(),
      object_type: crate::core::object_3d::ObjectType::$type,
      receive_shadow: Default::default(),
      user_data: Default::default(),
      _uuid: uuid::Uuid::new_v4().to_string(),
      _self_ref: Default::default(),
    });

    let mut that: std::rc::Rc<dyn crate::core::object_3d::ObjectActions> = this.clone();

    let _ = this._self_ref.set(std::rc::Rc::downgrade(&that));


    this
  }};
}

pub(crate) use define_support_objects;
pub(crate) use with_default_fields;
