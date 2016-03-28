extern crate tinyecs;

use tinyecs::*;


struct Health {
    hp : i32
}
impl Component for Health {}

struct Position {
    x : i32
}
impl Component for Position {}

struct Alive;
impl Component for Alive {}

pub struct BleedZoneSystem;
impl System for BleedZoneSystem {
    fn process_one(&mut self, entity : &mut Entity) {
        let pos = entity.get_component_cell::<Position>();
        let health = entity.get_component_cell::<Health>();

        if pos.borrow_mut().x == 5 {
            health.borrow_mut().hp -= 10;
            println!("You are in bleeding zone, hp: {}", health.borrow().hp);
        }
        if health.borrow().hp <= 0 {
            entity.remove_component::<Alive>();
            entity.refresh();
        }
    }
}

fn main() {
    let mut world = World::new();

    {
        let mut manager = world.entity_manager();
        let entity = manager.create_entity();
        entity.add_component(Health {hp : 100});
        entity.add_component(Position {x : 5});
        entity.add_component(Alive);
        entity.refresh();
    }
    world.set_system(BleedZoneSystem, Aspect::all3::<Position, Health, Alive>());

    for _ in 0..100 {
        world.update();
    }
}
