use std::collections::HashMap;

pub use std::any::{Any, TypeId};

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
        self.components.push(Box::new(component));
        self.type_id_mapping.insert(TypeId::of::<T>(), self.components.len() as i32 - 1);
    }

    pub fn remove_component<T : Any>(&mut self) {
        self.type_id_mapping.remove(&TypeId::of::<T>());
    }
    pub fn get_component<T : Any>(&mut self) -> &mut T {
        self.components.get_mut(self.type_id_mapping[&TypeId::of::<T>()] as usize).unwrap().downcast_mut().unwrap()
    }
    pub fn get_components<T : Any, T1 : Any>(&mut self) -> (&mut T, &mut T1) {
        use std::slice::from_raw_parts_mut;

        let i = self.type_id_mapping[&TypeId::of::<T>()];
        let i1 = self.type_id_mapping[&TypeId::of::<T1>()];
        let ptr = self.components.as_mut_ptr();

        unsafe {
            (from_raw_parts_mut(ptr.offset(i as isize), 1).get_mut(0).unwrap().downcast_mut().unwrap(),
             from_raw_parts_mut(ptr.offset(i1 as isize), 1).get_mut(0).unwrap().downcast_mut().unwrap())
        }
    }

    pub fn get_components3<T : Any, T1 : Any, T2 : Any>(&mut self) -> (&mut T, &mut T1, &mut T2) {
        use std::slice::from_raw_parts_mut;

        let i = self.type_id_mapping[&TypeId::of::<T>()];
        let i1 = self.type_id_mapping[&TypeId::of::<T1>()];
        let i2 = self.type_id_mapping[&TypeId::of::<T2>()];
        let ptr = self.components.as_mut_ptr();

        unsafe {
            (from_raw_parts_mut(ptr.offset(i as isize), 1).get_mut(0).unwrap().downcast_mut().unwrap(),
             from_raw_parts_mut(ptr.offset(i1 as isize), 1).get_mut(0).unwrap().downcast_mut().unwrap(),
            from_raw_parts_mut(ptr.offset(i2 as isize), 1).get_mut(0).unwrap().downcast_mut().unwrap())
        }
    }


}
