TinyECS
=======

Another Entity-Component-System, written in Rust.

Usage
-----

Usage

Add the following to the Cargo.toml in your project:
```
[dependencies]
tinyecs = "*"
```
and import using: 
```
extern crate tinyecs;
use tinyecs::*;
```

Features:
--------

- simple components createion:
```
struct Position {
    x : i32,
    y : i32,
    z : i32
}
impl Component for Position {}
```

- simple entities creation:

```
let mut entity_manager = world.entity_manager();
let entity = entity_manager.create_entity();

entity.add_component(Position {x : 0, y : 0, z : 0});
entity.add_component(Velocity {x : 1});
entity.refresh();
```

- simple macro for systems creation:
```
process_entities!((MoveSystem): |pos: Position, vel: Velocity| => {
    pos.x += vel.x;
    println!("Moving! position: {}, velocity: {}", pos.x, vel.x);
});
```

- or without macroses:
```
pub struct MoverSystem;
impl System for MoverSystem {
    fn aspect(&self) -> Aspect {
        aspect_all![Position, Dead]
    }

    fn process_one(&mut self, entity : &mut Entity) {
        let mut pos = entity.get_component::<Position>();
        let vel     = entity.get_component::<Velocity>(); // no problems with multiple mutable components

        pos.pos.x += vel.x;
    }
}
```


More examples in /examples folder :)
