/*!
Easy to use entity component system.

# Overview

  - Entity is set of components identified by unique ID.
  - Component is struct with data.
  - System is behaviour working with components.

All components must implement Component trait:

```
struct Position {
  x : i32
}
impl Component for Position {}
```

Entity can be created almost everywhere:


```
let entity = entity_manager.create_entity();

entity.add_component(Position {x : 0, y : 0, z : 0});
entity.add_component(Velocity {x : 1});
entity.refresh();
```

Simplest system can be created with macro.
typically, you will need only process some components, like this:

```
process_entities!((MoveSystem): |pos: Position, vel: Velocity| => {
    pos.x += vel.x;
    println!("Moving! position: {}, velocity: {}", pos.x, vel.x);
});
```

This system now must be added to world like this: 

```
world.set_system(MoveSystem::new());
```

this macro will be expanded to this:

```
pub struct MoveSystem;
impl System for MoveSystem {
  fn aspect() -> Aspect {
    aspect_all![Position, Velocity]
  }

  fn process_one(&mut self, entity: &mut Entity) {
    let mut pos = entity.get_component::<Position>();
    let mut vel = entity.get_component::<Velocity>();
    pos.x += vel.x;
    println!("Moving! position: {}, velocity: {}", pos.x, vel.x);
  }
}
```

*/
#![feature(cell_extras)]

mod entity;
mod component;
mod world;
mod system;
mod aspect;
mod fast_dict;

pub use world::*;

