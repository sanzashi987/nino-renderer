extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

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

#[proc_macro_attribute(RendererStruct)]
pub fn renderer_struct(attr: TokenStream, item: TokenStream) -> TokenStream {



}
