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

struct Player;
impl Component for Player {}

struct Bot;
impl Component for Bot {}

struct SomeTarget;
impl Component for SomeTarget {}

register_system!((MoveSystem): |_pos: Position, _vel: Velocity| => {
    _pos.x += _vel.x;
    println!("Moving! position: {}, velocity: {}", _pos.x, _vel.x);
});

register_system!((AiSystem): |_bot: Bot, _pos: Position, _vel: Velocity|
                 with (_players: aspect_all!(Player, Position),
                       _targets: aspect_all!(SomeTarget, Position)) => {
    _pos.x += _vel.x;
    for target in _targets {
        let Position {x, ..} = *target.get_component::<Position>();
        if _pos.x >= x {
            println!("Maybe attack this target?");
        }
    }
    println!("Moving! position: {}, velocity: {}", _pos.x, _vel.x);
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
