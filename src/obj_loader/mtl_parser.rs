use std::collections::HashMap;

use super::{error::Error, token_requester::TokenRequester, Material, MtlLib};

pub struct MtlLibParser<'a> {
  requester: &'a mut TokenRequester<'a>,
}

impl<'a> MtlLibParser<'a> {
  pub fn new(requester: &'a mut TokenRequester<'a>) -> Self {
    Self { requester }
  }

  pub fn parse(&mut self) -> Result<MtlLib, Error> {
    let mut mtllib = MtlLib {
      materials: HashMap::new(),
    };

    let mut mtl:Option<Material> = None;

    Ok(mtllib)
  }
}
