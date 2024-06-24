extern crate proc_macro;

use proc_macro::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse::Parse, parse_macro_input, AttributeArgs, DeriveInput, Field, Lit};

#[proc_macro_derive(RendererCommon)]
pub fn renderer_common(input: TokenStream) -> TokenStream {
  let ast: DeriveInput = syn::parse(input).unwrap();
  let id = ast.ident;

  quote! {
    impl #id {
      pub fn new(w: u32, h: u32, camera: Camera) -> Self {
        Self {
          camera,
          viewport: Viewport { x: 0, y: 0, w, h },
          color: ColorAttachment::new(w, h),
          depth:  DepthAttachment::new(w, h),
          shader: Default::default(),
          uniforms: Default::default(),
        }
      }
    }


    impl RendererDerive for # id {
      fn clear(&mut self, color: &crate::math::Vec4) {
        self.color.clear(color);
      }

      fn clear_depth(&mut self){
        self.depth.clear(f32::MIN);
      }

      fn get_canvas_width(&self) -> u32 {
        self.color.width()
      }

      fn get_canvas_height(&self) -> u32 {
        self.color.height()
      }

      fn get_frame_image(&self) -> &[u8] {
        self.color.data()
      }

      fn get_shader(&mut self) -> &mut Shader {
        &mut self.shader
      }

      fn get_uniforms(&mut self) -> &mut Uniforms {
        &mut self.uniforms
      }
    }
    impl RendererInterface for Renderer {}
  }
  .into()
}

#[proc_macro_attribute]
pub fn renderer(_: TokenStream, item: TokenStream) -> TokenStream {
  // let c = attr.clone();
  // let attr_ast = parse_macro_input!(attr as AttributeArgs);
  let ast: DeriveInput = syn::parse(item).unwrap();
  let struct_name = ast.ident;

  let mut attributes = vec![];

  if let syn::Data::Struct(data_struct) = ast.data {
    for field in data_struct.fields.iter() {
      let Field { ident, ty, .. } = field;
      // let ident_name = ident.as_ref().unwrap().to_string().repeat(2);
      // let ident = Some(syn::Ident::new(&ident_name, ident.as_ref().unwrap().span()));
      let attr = quote! {
        pub #ident:#ty
      };
      attributes.push(attr)
    }
  }

  quote! {
    pub struct #struct_name{
      #(#attributes)*
      pub color: ColorAttachment,
      pub depth: DepthAttachment,
      pub camera: Camera,
      pub viewport: Viewport,
      pub shader: Shader,
      pub uniforms: Uniforms,
      pub wireframe_mode: bool,
      pub front_face: FrontFace,
      pub cliped_triangles: Vec<Vertex>,
      pub cull: FaceCull
    }

    impl #struct_name {
      pub fn new(w: u32, h: u32, camera: Camera) -> Self {
        Self {
          camera,
          viewport: Viewport { x: 0, y: 0, w, h },
          color: ColorAttachment::new(w, h),
          depth:  DepthAttachment::new(w, h),
          shader: Default::default(),
          uniforms: Default::default(),
          wireframe_mode: Default::default(),
          front_face:FrontFace::CW,
          cull: FaceCull::None,
          cliped_triangles: vec![],
        }
      }
    }


    impl RendererDerive for #struct_name {
      fn clear(&mut self, color: &crate::math::Vec4) {
        self.color.clear(color);
      }

      fn clear_depth(&mut self){
        self.depth.clear(f32::MIN);
      }

      fn get_canvas_width(&self) -> u32 {
        self.color.width()
      }

      fn get_canvas_height(&self) -> u32 {
        self.color.height()
      }

      fn get_frame_image(&self) -> &[u8] {
        self.color.data()
      }

      fn get_shader(&mut self) -> &mut Shader {
        &mut self.shader
      }

      fn get_uniforms(&mut self) -> &mut Uniforms {
        &mut self.uniforms
      }

      fn enable_wireframe(&mut self) {
        self.wireframe_mode = true;
      }

      fn disable_wireframe(&mut self) {
        self.wireframe_mode = false;
      }

      fn toggle_wireframe(&mut self) {
        self.wireframe_mode = !self.wireframe_mode;
      }

      fn set_front_face(&mut self, face:FrontFace){
        self.front_face = face;
      }

      fn set_face_cull(&mut self, cull:FaceCull){
        self.cull = cull;
      }
      fn get_face_cull(&self)->FaceCull{
        self.cull
      }

      fn get_camera(&self) -> &Camera{
        &self.camera
      }


    }
    impl RendererInterface for Renderer {}

  }
  .into()
}
