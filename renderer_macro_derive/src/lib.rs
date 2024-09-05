extern crate proc_macro;

use proc_macro::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse::Parse, parse_macro_input, AttributeArgs, DeriveInput, Field, Lit};

#[proc_macro_attribute]
pub fn object_3d(args: TokenStream, input: TokenStream) -> TokenStream {
  let attr_ast = parse_macro_input!(args as AttributeArgs);
  let children = match &attr_ast[0] {
    syn::NestedMeta::Meta(syn::Meta::Path(p)) => {
      quote! {
        children:Vec<#p>,
      }
    }
    _ => quote! {},
  };
  let obj_trait = match &attr_ast[1] {
    syn::NestedMeta::Meta(syn::Meta::Path(p)) => {
      quote! {#p}
    }

    _ => quote! {},
  };

  let ast: DeriveInput = syn::parse(input).unwrap();
  let struct_name = ast.ident;
  let mut attributes = vec![];

  if let syn::Data::Struct(data_struct) = ast.data {
    for field in data_struct.fields.iter() {
      let Field { ident, ty, vis, .. } = field;
      // let ident_name = ident.as_ref().unwrap().to_string().repeat(2);
      // let ident = Some(syn::Ident::new(&ident_name, ident.as_ref().unwrap().span()));
      let attr = quote! {
        #vis #ident:#ty,
      };
      attributes.push(attr)
    }
  }

  quote! {
    pub struct #struct_name{
      #(#attributes)*
      parent: std::cell::RefCell<Option<std::rc::Rc<dyn #obj_trait>>>,
      #children
      matrix: crate::math::Mat4,
      matrix_global: crate::math::Mat4,
      position: crate::math::Vec3,
      rotation: crate::math::Vec3,
      scale: crate::math::Vec3,
      visible: bool,
      cast_shadow: bool,
      receive_shadow: bool,
      user_data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
    }


    impl #obj_trait for #struct_name {
      fn transform_matrix(&self) -> &crate::math::Mat4 {
        self.matrix
      }
      fn set_parent(&self, parent: Rc<dyn #obj_trait>){
        let mut p = self.parent.borrow_mut();
        *p = Some(parent);
      };
      fn get_parent(&self) -> Option<Rc<dyn #obj_trait>>{
        self.parent.borrow().map_or(None, |p| Some(p.clone()))
      };
    }

  }
  .into()
}
