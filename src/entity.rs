use std::collections::HashMap;

pub use std::any::{Any, TypeId};

use std::rc::Rc;
use std::cell::RefCell;
use component::*;

pub struct Entity {
    pub id              : i32,
    pub components      : HashMap<TypeId, Box<Any>>,
    pub fresh           : bool
}

impl Entity {
    pub fn new(id : i32) -> Entity {
        Entity {
            id              : id,
            components      : HashMap::new(),
            fresh           : true
        }
    }
    pub fn refresh(&mut self) {
        self.fresh = false;
    }
    pub fn add_component<T : Any + Component>(&mut self, component : T) {
        self.components.insert(TypeId::of::<T>(), Box::new(Rc::new(RefCell::new(Box::new(component)))));
    }

    pub fn remove_component<T : Any>(&mut self) {
        self.components.remove(&TypeId::of::<T>());
    }
    pub fn has_component<T : Any>(&mut self) -> bool {
        self.components.contains_key(&TypeId::of::<T>())
    }

    pub fn get_component_cell<T : Any>(&mut self) -> Rc<RefCell<Box<T>>> {
        let component = self.components.get_mut(&TypeId::of::<T>()).unwrap();
        let c : &mut Rc<RefCell<Box<T>>> = component.downcast_mut().unwrap();
        c.clone()
    }

    pub fn get_component_nomut<T : Any>(&self) -> &T {
        self.components.get(&TypeId::of::<T>()).unwrap().downcast_ref().unwrap()
    }
}

