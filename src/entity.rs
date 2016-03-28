use std::collections::HashMap;

pub use std::any::{Any, TypeId};

use std::rc::Rc;
use std::cell::RefCell;
use component::*;

pub struct Entity {
    pub id              : i32,
    pub components      : Vec<Box<Any>>,
    pub type_id_mapping : HashMap<TypeId, i32>,
    pub last_type_id    : i32,
    pub fresh           : bool
}

impl Entity {
    pub fn new(id : i32) -> Entity {
        Entity {
            id              : id,
            components      : Vec::new(),
            type_id_mapping : HashMap::new(),
            last_type_id    : 0,
            fresh           : true
        }
    }
    pub fn refresh(&mut self) {
        self.fresh = false;
    }
    pub fn add_component<T : Any + Component>(&mut self, component : T) {
        self.components.push(Box::new(Rc::new(RefCell::new(Box::new(component)))));
        self.type_id_mapping.insert(TypeId::of::<T>(), self.components.len() as i32 - 1);
    }

    pub fn remove_component<T : Any>(&mut self) {
        self.type_id_mapping.remove(&TypeId::of::<T>());
    }
    pub fn has_component<T : Any>(&mut self) -> bool {
        self.type_id_mapping.contains_key(&TypeId::of::<T>())
    }

    pub fn get_component_cell<T : Any>(&mut self) -> Rc<RefCell<Box<T>>> {
        let component = self.components.get_mut(self.type_id_mapping[&TypeId::of::<T>()] as usize).unwrap();
        let c : &mut Rc<RefCell<Box<T>>> = component.downcast_mut().unwrap();
        c.clone()
    }

    pub fn get_component_nomut<T : Any>(&self) -> &T {
        self.components.get(self.type_id_mapping[&TypeId::of::<T>()] as usize).unwrap().downcast_ref().unwrap()
    }
}

