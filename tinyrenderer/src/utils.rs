use std::{collections::HashMap, hash::Hash};

pub fn swap_and_move<T: Default>(val: &mut T) -> T {
  std::mem::replace(val, Default::default())
}

pub fn merge_hash_map<K: Eq + Hash, V>(map1: HashMap<K, V>, map2: HashMap<K, V>) -> HashMap<K, V> {
  let mut res = HashMap::new();
  res.extend(map1);
  res.extend(map2);
  res
}
