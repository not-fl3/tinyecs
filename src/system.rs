use entity::*;
use aspect::Aspect;
use world::{WorldHandle, EntityManager};
use std::collections::HashSet;

#[macro_export]
macro_rules! impl_aspect {
    ( $( $t:ty ),* ) => {
        fn aspect(&self) -> Aspect {
            use std::any::TypeId;
            Aspect {
                accept_types : vec!($(TypeId::of::<$t>()),*),
                not_accept_types : Vec::new()
            }
        }
    }
}

#[macro_export]
macro_rules! impl_data_aspect {
    ( $( $dataaspect:expr ),* ) => {
        fn data_aspects(&self) -> Vec<Aspect> {
            vec!($($dataaspect),*)
        }
    }
}
#[macro_export]
macro_rules! impl_new {
    ($name:ident) => {
        impl $name {
            pub fn new() -> $name {
                $name
            }
        }
    }
}
#[macro_export]
macro_rules! register_system {
    ( ($name:ident aspect $aspect:expr): $entity:ident |$( $varname:ident: $t:ty ), *| with ($( $datavar:ident: $dataaspect:expr ), *) => $code:expr) => {
        pub struct $name;
        impl_new!($name);
        impl System for $name {
            fn aspect(&self) -> Aspect {
                $aspect
            }
            impl_data_aspect!($($dataaspect),*);

            fn process_d(&mut self, $entity : &mut Entity, data : &mut DataList) {
                let mut _n = 0;
                $( let mut $datavar = data.unwrap_nth(_n); _n += 1; )*
                $( let mut $varname = $entity.get_component::<$t>(); )*
                $code
            }
        }
    };

    ( ($name:ident): |$( $varname:ident: $t:ty ), *| with ($( $datavar:ident: $aspect:expr ), *) => $code:expr) => {
        pub struct $name;
        impl_new!($name);
        impl System for $name {
            impl_aspect!($($t),*);
            impl_data_aspect!($($aspect),*);

            fn process_d(&mut self, entity : &mut Entity, data : &mut DataList) {
                let mut _n = 0;
                $( let mut $datavar = data.unwrap_nth(_n); _n += 1; )*
                $( let mut $varname = entity.get_component::<$t>(); )*
                $code
            }
        }
    };

    ( ($name:ident): |$( $varname:ident: $t:ty ), *| => $code:expr) => {
        pub struct $name;
        impl_new!($name);
        impl System for $name {
            impl_aspect!($($t),*);

            fn process_one(&mut self, entity : &mut Entity) {
                $( let mut $varname = entity.get_component::<$t>(); )*
                $code
            }
        }
    };
}

pub struct DataList<'a> {
    data : Vec<Vec<&'a mut Entity>>
}
impl<'b> DataList<'b> {
    pub fn unwrap_entity<'a>(&'a mut self) -> &'a mut Entity {
        &mut self.data[0][0]

    }

    pub fn unwrap_all<'a>(&'a mut self) -> &'a mut Vec<&'b mut Entity> {
        &mut self.data[0]
    }

    pub fn unwrap_nth<'a>(&'a self, n : usize) -> &'a Vec<&'b mut Entity> {
        &self.data[n]
    }


    pub fn new(entity_manager : &mut EntityManager, ids : &Vec<HashSet<i32>>) -> DataList<'b> {
        DataList {
            data : ids.iter().map(|i| {entity_manager.get_entities_by_ids(&i)}).collect()
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
    fn process_w(&mut self, _ : &mut Entity, _ : &mut WorldHandle) {
    }

    #[inline]
    fn process_d(&mut self, _ : &mut Entity, _ : &mut DataList) {
    }
    #[inline]
    fn process_wd(&mut self, _ : &mut Entity, _ : &mut WorldHandle, _ : &mut DataList) {
    }

    #[inline]
    fn process_one(&mut self, _ : &mut Entity) {
    }

    fn process_all(&mut self, entities : &mut Vec<&mut Entity>, world: &mut WorldHandle, data : &mut DataList) {
        for e in entities.iter_mut() {
            self.process_one(e);
            self.process_w(e, world);
            self.process_d(e, data);
            self.process_wd(e, world, data);
        }
    }
}

