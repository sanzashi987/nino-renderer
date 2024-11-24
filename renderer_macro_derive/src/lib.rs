extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, parse_macro_input, AttributeArgs, DeriveInput, Field, Ident, Lit};

#[proc_macro_attribute]
pub fn object_3d(args: TokenStream, input: TokenStream) -> TokenStream {
  let attr_ast = parse_macro_input!(args as AttributeArgs);

  let trait_name = match &attr_ast[0] {
    syn::NestedMeta::Meta(syn::Meta::Path(p)) => {
      quote! {#p}
    }

    _ => quote! {},
  };

  let ast: DeriveInput = syn::parse(input).unwrap();
  let struct_name = ast.ident;
  let mut attributes = vec![];
  let expr = quote! {#struct_name};
  let struct_name_str = expr.to_string();

  if let syn::Data::Struct(data_struct) = ast.data {
    for field in data_struct.fields.iter() {
      let Field { ident, ty, vis, .. } = field;
      let attr = quote! {
        #vis #ident:#ty,
      };
      attributes.push(attr)
    }
  }

  quote! {
    pub struct #struct_name{
      #(#attributes)*
      name:std::cell::RefCell<String>,
      event_emitter: std::cell::RefCell<crate::core::event_emitter::EventEmitter>,
      parent: std::cell::RefCell<Option<std::rc::Rc<dyn #trait_name>>>,
      children: std::cell::RefCell<Vec<std::rc::Rc<dyn #trait_name>>>,
      matrix: std::cell::RefCell<crate::math::Mat4>,
      global_matrix: std::cell::RefCell<crate::math::Mat4>,
      position: std::cell::RefCell<crate::math::Vec3>,
      rotation: std::cell::RefCell<crate::math::Rotation>,
      scale: std::cell::RefCell<crate::math::Vec3>,
      layers: std::cell::RefCell<crate::core::layer::Layers>,
      cast_shadow: bool,
      receive_shadow: bool,
      visible: std::cell::RefCell<bool>,
      user_data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
      object_type: crate::core::object_3d::ObjectType,
      _self_ref: std::cell::OnceCell<std::rc::Weak<dyn #trait_name>>,
      _uuid: String,
    }

    impl #trait_name for #struct_name {
      fn name(&self) -> String {
        self.name.borrow().to_string()
      }

      fn set_name(&self, name: &str) {
        let mut mutator = self.name.borrow_mut();
        *mutator = name.to_string();
      }


      fn parent(&self) -> Option<std::rc::Rc<dyn #trait_name>> {
        if let Some(p) = self.parent.borrow().as_ref() {
          Some(p.clone())
        } else {
          None
        }
      }

      fn set_parent(&self, parent: std::rc::Rc<dyn #trait_name>) {
        let mut p = self.parent.borrow_mut();
        *p = Some(parent);
      }

      fn remove_from_parent(&self) {
        let mut p = self.parent.borrow_mut();

        if let Some(parent) = p.as_ref() {
          parent.remove(&self._uuid);
        }

        *p = None;
      }

      fn remove(&self, uuid: &str) {
        let mut children = self.children.borrow_mut();

        if let Some(index) = children.iter().position(|x| (*x).uuid() == uuid) {
          children.remove(index);
        }
      }

      fn add(&self, child: std::rc::Rc<dyn #trait_name>) {
        let mut children = self.children.borrow_mut();

        if let Some(self_pointer) = self._self_ref.get() {
          if let Some(me) = self_pointer.upgrade() {
            child.remove_from_parent();
            child.set_parent(me.clone());
            children.push(child.clone());
          }
        }
      }

      fn clear(&self) {
        {
          let children = self.children.borrow();
          for child in children.iter() {
            child.remove_from_parent();
          }
        }
        let mut children = self.children.borrow_mut();

        *children = vec![];
      }

      fn attach(&self, child: Box<dyn #trait_name>) {
        self.update_global_matrix();

        let mut res = self
          .global_matrix
          .borrow()
          .inverse()
          .expect("expected a invertable global matrix");

        if let Some(parent) = child.parent() {
          parent.update_global_matrix();
          res = res * parent.global_matrix();
        }

        child.apply_matrix(res);
      }

      fn children(&self) -> std::cell::Ref<'_, Vec<std::rc::Rc<dyn #trait_name>>> {
        self.children.borrow()
      }

      fn look_at(&self, target: crate::math::Vec3) {
        // let back = (self.position - target).normalize();
        self.update_global_matrix();

        let position = crate::math::extract_position(*self.global_matrix.borrow());

        let is_revert_z = self.object_type == crate::core::object_3d::ObjectType::Camera
          || self.object_type == crate::core::object_3d::ObjectType::Light;

        let (eye, target) = if is_revert_z {
          // camera default looking back along -z;
          (position, target)
        } else {
          (target, position)
        };

        let orthogonal_basis =
          crate::math::Mat3::get_orthogonal_basis(eye, target, *crate::math::Vec3::y_axis());

        let mut rotate_mat = crate::math::Mat4::identity();

        for i in 0..2 {
          let col = crate::math::Vec4::from_vec3(&orthogonal_basis.get_col(i), 0.0);
          rotate_mat.set_col(i, col);
        }

        let mut q: crate::math::Quaternion = rotate_mat.into();

        if let Some(parent) = self.parent.borrow().as_ref() {
          let (_, r, _) = crate::math::decompose(parent.global_matrix());

          let q_parent: crate::math::Quaternion = r.into();

          q = q_parent.inverse() * q;
        }

        {
          let mut rotate_ref = self.rotation.borrow_mut();
          rotate_ref.set_quaternion(q);
        }
      }

      fn matrix(&self) -> crate::math::Mat4 {
        *self.matrix.borrow()
      }

      fn global_matrix(&self) -> crate::math::Mat4 {
        *self.global_matrix.borrow()
      }

      fn update_global_matrix(&self) {
        self.update_matrix();

        if let Some(parent) = self.parent.borrow().as_ref() {
          parent.update_global_matrix();
          let parent_global = parent.global_matrix();
          let mut global_matrix = self.global_matrix.borrow_mut();
          let local_matrix = self.matrix();
          *global_matrix = parent_global * local_matrix;
        }

        for child in std::ops::Deref::deref(&self.children.borrow()) {
          child.update_global_matrix();
        }

        let mat = self.global_matrix();

        let global_matrix:Box<dyn std::any::Any> = Box::new(mat);
        {
          let mut event_emitter = self.event_emitter.borrow_mut();
          event_emitter.emit("update:global_matrix",global_matrix);
        }

      }

      fn update_matrix(&self) {
        let next_matrix = self.compose();
        let mut matrix = self.matrix.borrow_mut();
        *matrix = next_matrix;
      }

      fn compose(&self) -> crate::math::Mat4 {
        let translate_matrix = crate::math::apply_translate(&self.position.borrow());
        let rotate_matrix = (*self.rotation.borrow()).quaternion.make_rotate_matrix();
        let scale_matrix = crate::math::apply_scale(&self.scale.borrow());

        translate_matrix * rotate_matrix * scale_matrix
      }
      /// refer to http://facweb.cs.depaul.edu/andre/gam374/extractingTRS.pdf
      fn decompose(&self) {
        let mat = *self.matrix.borrow();
        let (position, rotate_matrix, scale) = crate::math::decompose(mat);

        {
          self
            .rotation
            .borrow_mut()
            .update_quaternion_from_matrix(rotate_matrix);
        }

        {
          let mut scale_ref = self.scale.borrow_mut();
          *scale_ref = scale;
        }
        {
          let mut position_ref = self.position.borrow_mut();
          *position_ref = position;
        }
      }

      fn apply_matrix(&self, matrix: crate::math::Mat4) {
        self.update_matrix();
        let mut next_matrix = crate::math::Mat4::zeros();
        {
          next_matrix = matrix * *self.matrix.borrow();
        }
        {
          let mut matrix_ref = self.matrix.borrow_mut();
          *matrix_ref = next_matrix;
        }
        self.decompose();
      }

      fn apply_quaternion(&self, q: crate::math::Quaternion) {
        let next_q = (*self.rotation.borrow()).quaternion * q;
        let mut rotation_ref = self.rotation.borrow_mut();
        rotation_ref.set_quaternion(next_q);
      }

      fn rotate_on_world_axis(&self, axis: crate::math::Vec3, angle: f32) {
        let mut rotate = self.rotation.borrow_mut();
        // apply the rotation first(before any rotation occurs)
        // means the rotate axis is defined in the world coordinate
        rotate.quaternion_rotate(axis, angle, true)
      }

      fn rotate_on_axis(&self, axis: crate::math::Vec3, angle: f32) {
        let mut rotate = self.rotation.borrow_mut();
        rotate.quaternion_rotate(axis, angle, false)
      }

      fn rotate_x(&self, angle: f32) {
        self.rotate_on_axis(*crate::math::Vec3::x_axis(), angle);
      }

      fn rotate_y(&self, angle: f32) {
        self.rotate_on_axis(*crate::math::Vec3::y_axis(), angle);
      }

      fn rotate_z(&self, angle: f32) {
        self.rotate_on_axis(*crate::math::Vec3::z_axis(), angle);
      }

      fn update_position(&self, position:crate::math::Vec3) {
        let mut p = self.position.borrow_mut();
        *p = position;
      }

      fn update_from_global_position(&self, position:crate::math::Vec3) {
        let delta_position = self.global_position() - *self.position.borrow();
        let next_position = position - delta_position;

        let mut p = self.position.borrow_mut();
        *p = next_position;
      }


      fn translate_on_axis(&self, axis: crate::math::Vec3, distance: f32) {
        let q = self.rotation.borrow().quaternion;
        let crate::math::Vec3 {
          x: vx,
          y: vy,
          z: vz,
        } = axis;
      }

      fn translate_x(&self, distance: f32) {}
      fn translate_y(&self, distance: f32) {}
      fn translate_z(&self, distance: f32) {}

      fn global_scale(&self) -> crate::math::Vec3 {
        self.update_global_matrix();
        let mat = self.global_matrix();
        let (_, _, scale) = crate::math::decompose(mat);
        scale
      }

      fn global_position(&self) -> crate::math::Vec3 {
        self.update_global_matrix();
        let mat = self.global_matrix();
        let (position, _, _) = crate::math::decompose(mat);
        position
      }

      fn global_rotation(&self) -> crate::math::Rotation {
        self.update_global_matrix();
        let mat = self.global_matrix();
        let (_, rotation, _) = crate::math::decompose(mat);

        rotation.into()
      }

      fn cast_shadow(&self) -> bool {
        self.cast_shadow
      }


      fn test_layers(&self, layers: &crate::core::layer::Layers) -> bool {
        self.layers.borrow().test(layers)
      }

      fn layers(&self) -> std::cell::Ref<crate::core::layer::Layers> {
        self.layers.borrow()
      }

      fn visible(&self) -> bool {
        *self.visible.borrow()
      }

      fn get_type(&self) -> crate::core::object_3d::ObjectType {
        self.object_type
      }

      fn uuid(&self) -> &str {
        &self._uuid
      }

    }


    impl std::fmt::Debug for #struct_name {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(#struct_name_str)
          .field("matrix", &self.matrix)
          .field("global_matrix", &self.global_matrix)
          .field("position", &self.position)
          .field("rotation", &self.rotation)
          .field("scale", &self.scale)
          .field("layers", &self.layers)
          .field("cast_shadow", &self.cast_shadow)
          .field("receive_shadow", &self.receive_shadow)
          .field("visible", &self.visible)
          .field("user_data", &self.user_data)
          .field("object_type", &self.object_type)
          .field("_self_ref", &self._self_ref)
          .field("_uuid", &self._uuid)
          .finish()
      }
    }

  }
  .into()
}

#[proc_macro_attribute]
pub fn light_shadow(args: TokenStream, input: TokenStream) -> TokenStream {
  let attr_ast = parse_macro_input!(args as AttributeArgs);

  let trait_name = match &attr_ast[0] {
    syn::NestedMeta::Meta(syn::Meta::Path(p)) => {
      quote! {#p}
    }

    _ => quote! {},
  };

  let ast: DeriveInput = syn::parse(input).unwrap();
  let struct_name = ast.ident;
  let mut attributes = vec![];
  let expr = quote! {#struct_name};
  let struct_name_str = expr.to_string();

  if let syn::Data::Struct(data_struct) = ast.data {
    for field in data_struct.fields.iter() {
      let Field { ident, ty, vis, .. } = field;
      let attr = quote! {
        #vis #ident:#ty,
      };
      attributes.push(attr)
    }
  }

  quote! {
    pub struct #struct_name{
      #(#attributes)*
      camera: Rc<dyn ICamera>,
      intensity: i32,
      bias: i32,
      normal_bias: i32,
      radius: i32,
      // shadow texture width & height
      map_size: Vec2,
      mat: Mat4,
      // vec4 -> offsetx, offsety, width, height
      viewports: Vec<Vec4>,
      map: Option<RenderTarget>,

      matrix: Mat4,
    }
    impl ILightShadow for LightShadow {
      fn matrix(&self) -> Mat4 {
        self.matrix
      }

      fn camera(&self) -> Rc<dyn ICamera> {
        self.camera.clone()
      }

      fn map_size(&self) -> Vec2 {
        self.map_size
      }

      fn viewports(&self) -> &Vec<Vec4> {
        &self.viewports
      }

      fn update_matrices(&self, light: Rc<dyn super::light::ILight>, viewport: Vec4) {
        todo!()
      }

      fn map(&self) -> &RenderTarget {
        todo!()
      }
    }

  }
  .into()
}
