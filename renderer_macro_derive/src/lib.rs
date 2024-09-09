extern crate proc_macro;

use proc_macro::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse::Parse, parse_macro_input, AttributeArgs, DeriveInput, Field, Lit};

#[proc_macro_attribute]
pub fn object_3d(args: TokenStream, input: TokenStream) -> TokenStream {
  let attr_ast = parse_macro_input!(args as AttributeArgs);
  // let enum_name = match &attr_ast[0] {
  //   syn::NestedMeta::Meta(syn::Meta::Path(p)) => {
  //     quote! {#p }
  //   }
  //   _ => quote! {},
  // };
  let obj_trait = match &attr_ast[0] {
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
      // children: std::cell::RefCell<Vec<#enum_name>>,
      children: std::cell::RefCell<Vec<Box<dyn #obj_trait>>>,
      matrix: crate::math::Mat4,
      global_matrix: std::cell::RefCell<crate::math::Mat4>,

      position: crate::math::Vec3,
      rotation: crate::math::Vec3,
      scale: crate::math::Vec3,
      visible: bool,
      cast_shadow: bool,
      receive_shadow: bool,
      user_data: std::collections::HashMap<String, Box<dyn std::any::Any>>,
    }


    impl #obj_trait for #struct_name {
      fn matrix(&self) -> crate::math::Mat4 {
        self.matrix
      }
      fn global_matrix(& self) -> crate::math::Mat4 {
        if let Some(p) = self.parent.borrow().as_ref() {
          let m = p.global_matrix();
          let mut gm = self.global_matrix.borrow_mut();
          *gm = m * *gm;
        }
        *self.global_matrix.borrow()
      }
      fn set_parent(&self, parent: std::rc::Rc<dyn #obj_trait>){
        let mut p = self.parent.borrow_mut();
        *p = Some(parent);
      }
      fn get_parent(&self) -> Option<std::rc::Rc<dyn #obj_trait>> {
        if let Some(p) = self.parent.borrow().as_ref() {
          Some(p.clone())
        } else {
          None
        }
      }
      fn add(&self, val: Box<dyn #obj_trait>) {
        let mut children  = self.children.borrow_mut();
        children.push(val);
      }
    }

  }
  .into()
}
