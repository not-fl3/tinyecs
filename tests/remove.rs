extern crate tinyecs;

use tinyecs::*;

pub struct ComponentA;
impl Component for ComponentA {}

pub struct SystemA;
impl System for SystemA {
    fn aspect(&self) -> Aspect {
        Aspect::all::<ComponentA>()
    }
    fn process_one(&mut self, e : &mut Entity) {
        e.remove_component::<ComponentA>();
        e.refresh();
    }
}

pub struct SystemB;
impl System for SystemB {
    fn aspect(&self) -> Aspect {
        Aspect::all::<ComponentA>()
    }
    fn process_one(&mut self, e : &mut Entity) {
        e.get_component::<ComponentA>();
    }
}

#[test]
fn test_remove_component() {
    let mut world = World::new();
    let id = {
        let mut entity_manager = world.entity_manager();
        let e = entity_manager.create_entity();
        e.add_component(ComponentA);
        e.refresh();
        e.id
    };

    world.set_system(SystemA);
    world.set_system(SystemB);

    for _ in 0 .. 10 {
        world.update();
    }

    {
        let mut entity_manager = world.entity_manager();
        let entity = entity_manager.try_get_entity(id).unwrap();
        assert_eq!(entity.has_component::<ComponentA>(), false);
    }
}
