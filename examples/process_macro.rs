#[macro_use] extern crate tinyecs;

use tinyecs::*;

struct Position {
    x : i32
}
impl Component for Position {}

struct Velocity {
    x : i32
}
impl Component for Velocity {}

process_entities!((MoveSystem): |pos: Position, vel: Velocity| => {
    pos.x += vel.x;
    println!("Moving! position: {}, velocity: {}", pos.x, vel.x);
});

fn main() {
    let mut world = World::new();

    {
        let mut entity_manager = world.entity_manager();
        let entity = entity_manager.create_entity();

        entity.add_component(Position {x : 0});
        entity.add_component(Velocity {x : 1});
        entity.refresh();
    }
    world.set_system(MoveSystem);
    world.update();

    world.update();
    world.update();
}
