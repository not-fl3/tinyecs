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

# Why another ecs? 

 - mutliple mutable components access
 - no boilerplate for entity and systems creation/accessing
 - no restrictions on component content - non-copyable non-clonable structs is OK
 - entity creation possible alomst everywhere
 - data aspects - possibility to view some additional entities while processing 


# Overview:

  - Entity is set of components identified by unique ID.
  - Component is struct with data.
  - System is behaviour working with components.

- components:
```
struct Position {
    x : i32,
    y : i32,
    z : i32
}
impl Component for Position {}
```

Entities:

```
let mut entity_manager = world.entity_manager();
let entity = entity_manager.create_entity();

entity.add_component(Position {x : 0, y : 0, z : 0});
entity.add_component(Velocity {x : 1});
entity.refresh();
```

Systems:
```
process_entities!((MoveSystem): |pos: Position, vel: Velocity| => {
    pos.x += vel.x;
    println!("Moving! position: {}, velocity: {}", pos.x, vel.x);
});
```

Or without macroses:
```
pub struct MoverSystem;
impl System for MoverSystem {
    fn aspect(&self) -> Aspect {
        aspect_all![Position, Velocity]
    }

    fn process_one(&mut self, entity : &mut Entity) {
        let mut pos = entity.get_component::<Position>();
        let vel     = entity.get_component::<Velocity>(); // no problems with multiple mutable components

        pos.pos.x += vel.x;
    }
}
```

More features, described only in /examples atm:
- Aspects
- Entity creation from system's process
- Data aspects - for additional kind of entities in process
- Different process styles
