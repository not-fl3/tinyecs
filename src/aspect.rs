use component::*;
use entity::*;

pub struct Aspect {
    pub accept_types     : Vec<TypeId>,
    pub not_accept_types : Vec<TypeId>
}
impl Aspect {
    pub fn check(&self, entity : &Entity) -> bool {
        self.accept_types.iter().all(|ty| { entity.components.borrow().contains_key(ty) }) &&
            self.not_accept_types.iter().any(|ty| { entity.components.borrow().contains_key(ty) }) == false
    }
}
impl Aspect {
    pub fn all<T : Any + Component>() -> Aspect {
        Aspect {
            accept_types : vec![TypeId::of::<T>()],
            not_accept_types : Vec::new()
        }
    }
    pub fn all2<T : Any + Component, T1 : Any + Component>() -> Aspect {
        Aspect {
            accept_types : vec![TypeId::of::<T>(), TypeId::of::<T1>()],
            not_accept_types : Vec::new()
        }
    }
    pub fn all3<T : Any + Component, T1 : Any + Component, T2 : Any + Component>() -> Aspect {
        Aspect {
            accept_types : vec![TypeId::of::<T>(), TypeId::of::<T1>(), TypeId::of::<T2>()],
            not_accept_types : Vec::new()
        }
    }
    pub fn all4<T : Any + Component,
                T1 : Any + Component,
                T2 : Any + Component,
                T3 : Any + Component>() -> Aspect {
        Aspect {
            accept_types : vec![TypeId::of::<T>(), TypeId::of::<T1>(), TypeId::of::<T2>(), TypeId::of::<T3>()],
            not_accept_types : Vec::new()
        }
    }

    pub fn all5<T : Any + Component,
                T1 : Any + Component,
                T2 : Any + Component,
                T3 : Any + Component,
                T4 : Any + Component>() -> Aspect {
        Aspect {
            accept_types : vec![TypeId::of::<T>(), TypeId::of::<T1>(), TypeId::of::<T2>(), TypeId::of::<T3>(), TypeId::of::<T4>()],
            not_accept_types : Vec::new()
        }
    }

    pub fn except<T : Any + Component>(mut self) -> Aspect {
        self.not_accept_types.push(TypeId::of::<T>());
        Aspect {
            accept_types : self.accept_types,
            not_accept_types : self.not_accept_types
        }
    }
    pub fn except2<T : Any + Component, T1 : Any + Component>(mut self) -> Aspect {
        self.not_accept_types.push(TypeId::of::<T>());
        self.not_accept_types.push(TypeId::of::<T1>());
        Aspect {
            accept_types : self.accept_types,
            not_accept_types : self.not_accept_types
        }
    }
    pub fn except3<T : Any + Component, T1 : Any + Component, T2 : Any + Component>(mut self) -> Aspect {
        self.not_accept_types.push(TypeId::of::<T>());
        self.not_accept_types.push(TypeId::of::<T1>());
        self.not_accept_types.push(TypeId::of::<T2>());
        Aspect {
            accept_types : self.accept_types,
            not_accept_types : self.not_accept_types
        }
    }
}
