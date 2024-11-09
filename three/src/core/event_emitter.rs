use std::{any::Any, collections::HashMap, rc::Rc};

#[derive(Default)]
pub struct EventEmitter {
  listeners: HashMap<String, HashMap<i32, Box<dyn FnMut(Rc<dyn Any>)>>>,
  uid: i32,
}

impl EventEmitter {
  pub fn emit(&mut self, event: &str, payload: Box<dyn Any>) {
    let payload_ref: Rc<dyn Any> = payload.into();

    let subscribers = self.listeners.get_mut(event);

    if let Some(cbs) = subscribers {
      cbs.values_mut().for_each(|cb| {
        cb(payload_ref.clone());
      });
    }
  }

  pub fn on(&mut self, event: &str, cb: Box<dyn FnMut(Rc<dyn Any>)>) -> (String, i32) {
    self
      .listeners
      .entry(event.to_string())
      .or_insert_with(HashMap::new)
      .insert(self.uid, cb);
    let res = (event.to_string(), self.uid);
    self.uid += 1;
    res
  }

  pub fn off(&mut self, event: &str, uid: i32) {
    self.listeners.entry(event.to_string()).and_modify(|e| {
      e.remove(&uid);
    });
  }
}
