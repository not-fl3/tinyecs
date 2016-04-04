use std::collections::HashMap;
use std::ops::{Deref, DerefMut, Drop};
use std::any::{Any, TypeId};

use std::cell::RefCell;
use component::*;

pub struct Entity {
    pub id                 : i32,
    pub components         : RefCell<HashMap<TypeId, Box<Any>>>,
    pub fresh              : RefCell<bool>
}

#[doc(hidden)]
pub struct ComponentGuard<'a, T : Any> {
    component  : Option<Box<T>>,
    collection : &'a RefCell<HashMap<TypeId, Box<Any>>>
}
impl <'a, T : Any> Deref for ComponentGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.component.as_ref().unwrap()
    }
}

impl <'a, T : Any> DerefMut for ComponentGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.component.as_mut().unwrap()
    }
}
impl<'a, T : Any> Drop for ComponentGuard<'a, T> {
    fn drop(&mut self) {
        self.component.take().and_then(|component| {
            self.collection.borrow_mut().insert(TypeId::of::<T>(), component)
        });
    }
}

impl Entity {
    pub fn new(id : i32) -> Entity {
        Entity {
            id                 : id,
            components         : RefCell::new(HashMap::new()),
            fresh              : RefCell::new(false)
        }
    }

    /// Mark this entity as not refreshed.
    /// On beginning of next frame new registered components will affect their systems.
    pub fn refresh(&self) {
        *self.fresh.borrow_mut() = false;
    }
    pub fn add_component<T : Any + Component>(&self, component : T) {
        self.components.borrow_mut().insert(TypeId::of::<T>(), Box::new(component));
    }

    /// Remove component of given type from entity
    /// Be carefull, if this component is borrowed at this moment, it will not be really deleted.
    pub fn remove_component<T : Any>(&self) {
        self.components.borrow_mut().remove(&TypeId::of::<T>());
    }

    pub fn has_component<T : Any>(&self) -> bool {
        self.components.borrow().contains_key(&TypeId::of::<T>())
    }

    pub fn get_component<T : Any + Component>(&self) -> ComponentGuard<T> {
        let component = self.components.borrow_mut().remove(&TypeId::of::<T>()).unwrap();
        let c : Box<T> = component.downcast().unwrap();

        ComponentGuard {
            component: Some(c),
            collection: &self.components,
        }
    }

    #[doc(hidden)]
    pub fn get_components<T : Any + Component, T1 : Any + Component>(&self) -> (ComponentGuard<T>, ComponentGuard<T1>) {
        (self.get_component::<T>(), self.get_component::<T1>())
    }
    #[doc(hidden)]
    pub fn get_components3<T : Any + Component, T1 : Any + Component, T2 : Any + Component>(&self) -> (ComponentGuard<T>, ComponentGuard<T1>, ComponentGuard<T2>) {
        (self.get_component::<T>(), self.get_component::<T1>(), self.get_component::<T2>())
    }
}

#[test]
fn get_component_test() {
    use world::World;
    struct Position {x : i32};
    impl Component for Position{}
    let mut world = World::new();
    let eid = {
        let mut entity_manager = world.entity_manager();
        let entity = entity_manager.create_entity();
        {
            entity.add_component(Position {x : 2});
            entity.refresh();
        }
        entity.id
    };
    world.update();
    {
        let mut entity_manager = world.entity_manager();
        let entity = entity_manager.try_get_entity(eid).unwrap();
        let mut some = entity.get_component::<Position>();
        entity.refresh();
        some.x += 1;
    }
    {
        let mut entity_manager = world.entity_manager();
        let entity = entity_manager.try_get_entity(eid).unwrap();

        entity.remove_component::<Position>();
        entity.refresh();
    }
    world.update();
    {
        let mut entity_manager = world.entity_manager();
        let entity = entity_manager.try_get_entity(eid).unwrap();
        assert_eq!(entity.has_component::<Position>(), false);
    }

}
