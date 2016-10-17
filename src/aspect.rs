use component::*;
use entity::*;
use std::any::{Any, TypeId};

/// data for systems, storing which components they should be intrested in
pub struct Aspect {
    pub accept_types     : Vec<TypeId>,
    pub not_accept_types : Vec<TypeId>,
    pub optional         : bool
}
impl Aspect {
    pub fn check(&self, entity : &Entity) -> bool {
        self.accept_types.iter().all(|ty| { entity.components.borrow().keys().any(|t| t == ty )}) &&
            self.not_accept_types.iter().any(|ty| { entity.components.borrow().keys().any(|t| t == ty) }) == false
    }
}

/// make aspect for all of this types
#[macro_export]
macro_rules! aspect_all{( $ ($aspect:ty), * ) => {
    {
        Aspect {
            accept_types : vec![$( {
                use std::any::TypeId;
                TypeId::of::<$aspect>()
            }),*],
            not_accept_types : Vec::new(),
            optional : false
        }
    }
}}

impl Aspect {
    pub fn all<T : Any + Component>() -> Aspect {
        Aspect {
            accept_types : vec![TypeId::of::<T>()],
            not_accept_types : Vec::new(),
            optional : false
        }
    }
    pub fn all2<T : Any + Component, T1 : Any + Component>() -> Aspect {
        Aspect {
            accept_types : vec![TypeId::of::<T>(), TypeId::of::<T1>()],
            not_accept_types : Vec::new(),
            optional : false
        }
    }

    pub fn optional(self) -> Aspect {
        Aspect {
            accept_types : self.accept_types,
            not_accept_types : self.not_accept_types,
            optional : true
        }
    }
    pub fn except<T : Any + Component>(mut self) -> Aspect {
        self.not_accept_types.push(TypeId::of::<T>());
        Aspect {
            accept_types : self.accept_types,
            not_accept_types : self.not_accept_types,
            optional : false
        }
    }

    pub fn except2<T : Any + Component, T1 : Any + Component>(mut self) -> Aspect {
        self.not_accept_types.push(TypeId::of::<T>());
        self.not_accept_types.push(TypeId::of::<T1>());
        Aspect {
            accept_types : self.accept_types,
            not_accept_types : self.not_accept_types,
            optional : false
        }
    }
    pub fn except3<T : Any + Component, T1 : Any + Component, T2 : Any + Component>(mut self) -> Aspect {
        self.not_accept_types.push(TypeId::of::<T>());
        self.not_accept_types.push(TypeId::of::<T1>());
        self.not_accept_types.push(TypeId::of::<T2>());
        Aspect {
            accept_types : self.accept_types,
            not_accept_types : self.not_accept_types,
            optional : false
        }
    }
}
