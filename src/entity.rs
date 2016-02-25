use std::collections::HashMap;

pub use std::any::{Any, TypeId};

pub struct Entity {
    pub id         : i32,
    pub components : HashMap<TypeId, Box<Any>>
}

impl Entity {
    pub fn add_component<T : Any>(&mut self, component : T) {
        self.components.insert(TypeId::of::<T>(), Box::new(component));
    }

    pub fn get_component<T : Any>(&mut self) -> &mut T {
        self.components.get_mut(&TypeId::of::<T>()).unwrap().downcast_mut().unwrap()
    }
}
