#[macro_use] extern crate tinyecs;

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
    fn aspect(&self) -> Aspect {
        aspect_all![Position, Health, Alive]
    }

    fn process_one(&mut self, entity : &mut Entity) {
        let pos = entity.get_component::<Position>();
        let mut health = entity.get_component::<Health>();

        if pos.x == 5 {
            health.hp -= 10;
            println!("You are in bleeding zone, hp: {}", health.hp);
        }
        if health.hp <= 0 {
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
    world.set_system(BleedZoneSystem);

    for _ in 0..100 {
        world.update();
    }
}
