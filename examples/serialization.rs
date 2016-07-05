#[macro_use] extern crate tinyecs;
extern crate toml;
extern crate rustc_serialize;

use std::marker::PhantomData;

use tinyecs::*;

#[derive(RustcDecodable)]
struct Position { x : f32 }

impl Component for Position {}

#[derive(RustcDecodable)]
struct Velocity { vel : f32 }

impl Component for Velocity {}

fn main() {
    let toml = r#"
         [Position]
         x = 0.0
         [Velocity]
         vel = 1.0
     "#;

    let deserializers = deserializers!(Position, Velocity);

    let entity = Entity::new(0);

    deserializers.deserialize(&entity, &toml);
    entity.refresh();

    assert!(entity.has_component::<Position>());
    assert_eq!(entity.get_component::<Position>().x, 0.0);

    assert!(entity.has_component::<Velocity>());
    assert_eq!(entity.get_component::<Velocity>().vel, 1.0);
}
