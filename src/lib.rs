/*!
Easy to use entity component system.

# Overview

  - Entity is set of components identified by unique ID.
  - Component is struct with data.
  - System is behaviour working with components.

All components must implement Component trait:

```ignore
struct Position {
  x : i32
}
impl Component for Position {}
```

Entity can be created almost everywhere:


```ignore
let entity = entity_manager.create_entity();

entity.add_component(Position {x : 0, y : 0, z : 0});
entity.add_component(Velocity {x : 1});
entity.refresh();
```

Simplest system can be created with macro.
typically, you will need only process some components, like this:

```ignore
register_system!((MoveSystem): |pos: Position, vel: Velocity| => {
    pos.x += vel.x;
    println!("Moving! position: {}, velocity: {}", pos.x, vel.x);
});
```

This system now must be added to world like this:

```ignore
world.set_system(MoveSystem::new());
```

this macro will be expanded to this:

```ignore
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
#![cfg_attr(feature = "prof", feature(core_intrinsics))]

#[cfg(feature = "prof")]
#[macro_use] extern crate tinyprof;

#[cfg(not(feature = "prof"))]
macro_rules! profile_region { ($name:expr) => {} }

#[cfg(feature = "serialization")]
extern crate toml;
#[cfg(feature = "serialization")] 
extern crate rustc_serialize;

extern crate time;
extern crate vec_map;

mod entity;
mod component;
mod world;
mod system;
mod aspect;

#[cfg(feature = "serialization")]
mod serialization;

pub use world::*;
#[cfg(feature = "serialization")]
pub use serialization::*;
