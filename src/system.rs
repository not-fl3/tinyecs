use entity::*;
use aspect::Aspect;
use world::{WorldHandle, EntityManager};
use std::collections::HashSet;

#[doc(hidden)]
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

#[doc(hidden)]
#[macro_export]
macro_rules! impl_data_aspect {
    ( $( $dataaspect:expr ),* ) => {
        fn data_aspects(&self) -> Vec<Aspect> {
            vec!($($dataaspect),*)
        }
    }
}
#[doc(hidden)]
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

/// Create struct and impl System trait for it
///
/// ```ignore
/// register_system!((MoveSystem): |_pos: Position, _vel: Velocity| => {
/// });
/// ```
///
/// ```ignore
/// register_system!((AiSystem): |_bot: Bot, _pos: Position, _vel: Velocity|
///                 with (_players: aspect_all!(Player, Position),
///                       _targets: aspect_all!(SomeTarget, Position)) => {
/// });
/// ```
///
/// ```ignore
/// register_system!((BotControlSystem
///                   aspect aspect_all!(Position, Bot).except2::<Punch, Jump>()):
///                 |bot : Bot, pos : Position|
///                 with (scores: aspect_all!(ScoreTarget, Position),
///                       players: aspect_all!(Player, Position),
///                       objects: aspect_all!(ScoreObject, RigidBody)) => {
/// });
/// ```
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

/// Usefull when you have one component, and want to have another, built on given one.
/// Like this:
///
/// ```ignore
/// transit_system!(Glutin2HeavySystem: RenderData => HeavyGuiData,
///    |render_data| { HeavyGuiData::new(&mut render_data.facade.borrow_mut() ) });
/// ```
/// With this system, HeavyGuiData will automatically be added to entities with RenderData.
#[macro_export]
macro_rules! transit_system {
    ($name:ident: $from:ty => $to:ty, |$var:ident| $how:expr) => {
        pub struct $name;
        impl_new!($name);
        impl System for $name {
            fn aspect(&self) -> Aspect {
                aspect_all!($from).except::<$to>()
            }

            fn process_one(&mut self, entity : &mut Entity) {
                let $var = entity.get_component::<$from>();
                entity.add_component($how);
                entity.refresh();
            }
        }
    };
}

#[macro_export]
macro_rules! transit_with_data_system {
    ($name:ident: $from:ty => $to:ty, ($( $datavar:ident: $datatype:ty ), *), |$var:ident, ($( $evar:ident: $etype:ty ), *)| $how:expr) => {
        pub struct $name;
        impl_new!($name);
        impl System for $name {
            fn aspect(&self) -> Aspect {
                aspect_all!($from).except::<$to>()
            }
            fn data_aspects(&self) -> Vec<Aspect> {
                vec![$(aspect_all!($datatype),)*]
            }
            fn process_d(&mut self, entity : &mut Entity, data : &mut DataList) {
                let $var = entity.get_component::<$from>();
                $( let mut $evar = entity.get_component::<$etype>(); )*
                $( let mut $datavar = data.unwrap_entity().get_component::<$datatype>(); )*

                entity.add_component($how);
                entity.refresh();
            }
        }
    };
}

#[macro_export]
macro_rules! transit_sync_system {
    ($name:ident: $from:ty => $to:ty, |$var:ident| $how:expr) => {
        pub struct $name;
        impl_new!($name);
        impl System for $name {
            fn aspect(&self) -> Aspect {
                aspect_all!($from).except::<$to>()
            }

            fn process_one(&mut self, entity : &mut Entity) {
                let $var = entity.read_sync_component::<$from>();
                entity.add_component($how);
                entity.refresh();
            }
        }
    };
}

/// list with additional entitiy packs from data aspect
///
/// Strongly recommends not use this ever, only for macroses!
pub struct DataList<'a> {
    data : Vec<Vec<&'a mut Entity>>
}
impl<'b> DataList<'b> {
    pub fn unwrap_entity<'a>(&'a self) -> &'a Entity {
        &self.data[0][0]
    }

    pub fn unwrap_entity_nth<'a>(&'a self, n : usize) -> &'a Entity {
        &self.data[n][0]
    }
    pub fn unwrap_entity_mut<'a>(&'a mut self) -> &'a mut Entity {
        &mut self.data[0][0]
    }

    pub fn unwrap_all<'a>(&'a mut self) -> &'a mut Vec<&'b mut Entity> {
        &mut self.data[0]
    }

    pub fn unwrap_nth<'a>(&'a self, n : usize) -> &'a Vec<&'b mut Entity> {
        &self.data[n]
    }
    pub fn unwrap_mut_nth<'a>(&'a mut self, n : usize) -> &'a mut Vec<&'b mut Entity> {
        &mut self.data[n]
    }


    pub fn new(entity_manager : &mut EntityManager<'b>, ids : &Vec<HashSet<i32>>) -> DataList<'b> {
        DataList {
            data : ids.iter().map(|i| {entity_manager.get_entities_by_ids(&i)}).collect()
        }
    }
}

/// System traits
///
/// You can implement one of those processes, but if you implement process_all - only it will be called, and if you dont implement process_all - all process_* will be called.
///
/// Most common case - implement only process_one.
pub trait System {
    /// System will subscribe only on components, sutisfied by this aspect.
    fn aspect(&self) -> Aspect;

    /// For each returned aspect, one additional entity pack DataList will be received.
    /// Strongly recomends use it only with registration macro.
    fn data_aspects(&self) -> Vec<Aspect> {
        Vec::new()
    }

    #[cfg(feature = "prof")]
    fn get_name(&self) -> String {
        use std::intrinsics::*;

        let type_name =
            unsafe {
                type_name::<Self>()
            };
        type_name.to_string()

    }
    fn on_created(&mut self, _ : &mut EntityManager) {

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
