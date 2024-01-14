use lazy_static::lazy_static;
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

// lazy_static! {
//     static ref FRUIT_STORAGE: Storage = {
//         Storage::new()
//     };
// }
//
// pub fn add(key: String, value: i32) {
//     // FRUIT_STORAGE.insert(key, value);
// }

// struct Storage {
//     fruit_storage: HashMap<String, i32>
// }

pub type FruitStorage = HashMap<String, i32>;

// impl Deref for Storage {
//     type Target = HashMap<String, i32>;
//
//     fn deref(&self) -> &Self::Target {
//         &self.fruit_storage
//     }
// }
//
// impl DerefMut for Storage {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.fruit_storage
//     }
// }
