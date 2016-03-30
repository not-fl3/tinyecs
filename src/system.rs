use entity::*;
use aspect::Aspect;
use world::{WorldData, EntityManager};
use std::collections::HashSet;

#[macro_export]
macro_rules! process_entities {
    ( ($name:ident): |$( $t:ty:  $varname:ident ), *| => $code:expr) => {
        struct $name;
        impl System for $name {
            fn aspect(&self) -> Aspect {
                Aspect {
                    accept_types : vec!($(TypeId::of::<$t>()),*),
                    not_accept_types : Vec::new()
                }
            }
            fn process_one(&mut self, entity : &mut Entity) {
                $( let mut $varname = entity.get_component::<$t>(); )*
                $code
            }
        }
    };
}
pub enum SomeData<'a> {
    None,
    Entity(Vec<&'a mut Entity>)
}
impl<'b> SomeData<'b> {
    pub fn unwrap_entity<'a>(&'a mut self) -> &'a mut Entity {
        match self {
            &mut SomeData::None => panic!("not entity data"),
            &mut SomeData::Entity(ref mut e) => &mut e[0]
        }
    }

    pub fn unwrap_all<'a>(&'a mut self) -> &'a mut Vec<&'b mut Entity> {
        match *self {
            SomeData::None => panic!("not entity data"),
            SomeData::Entity(ref mut e) => e
        }
    }


    pub fn new(entity_manager : &mut EntityManager, ids : &Vec<HashSet<i32>>) -> SomeData<'b> {
        if ids.len() == 0 {
            SomeData::None
        } else if ids.len() == 1 {
            SomeData::Entity(entity_manager.get_entities_by_ids(&ids[0]))
        } else {
            panic!("Atm only 1 pack of data entities supported");
        }
    }
}

pub trait System {
    fn aspect(&self) -> Aspect;
    fn data_aspects(&self) -> Vec<Aspect> {
        Vec::new()
    }
    fn on_begin_frame(&mut self) {
    }

    fn on_added(&mut self, _ : &mut Entity) {
    }

    fn on_removed(&self, _ : &mut Entity) {
    }

    fn on_end_frame(&mut self) {
    }
    #[inline]
    fn process_w(&mut self, _ : &mut Entity, _ : &mut WorldData) {
    }

    #[inline]
    fn process_d(&mut self, _ : &mut Entity, _ : &mut SomeData) {
    }
    #[inline]
    fn process_wd(&mut self, _ : &mut Entity, _ : &mut WorldData, _ : &mut SomeData) {
    }

    #[inline]
    fn process_one(&mut self, _ : &mut Entity) {
    }

    fn process_all(&mut self, entities : &mut Vec<&mut Entity>, world: &mut WorldData, data : &mut SomeData) {
        for e in entities.iter_mut() {
            self.process_one(e);
            self.process_w(e, world);
            self.process_d(e, data);
            self.process_wd(e, world, data);
        }
    }
}

