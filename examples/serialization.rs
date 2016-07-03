#[macro_use] extern crate tinyecs;
extern crate toml;

use std::marker::PhantomData;

use tinyecs::*;
use toml::*;

struct Position { x : f32 }
impl Component for Position {}

struct Velocity { vel : f32 }
impl Component for Velocity {}

impl Deserializable for Position {
    fn deserialize(val : &Value) -> Position {
        let x = val.lookup("x").unwrap();

        Position { x : x.as_float().unwrap() as f32 }

    }
}

impl Deserializable for Velocity {
    fn deserialize(val : &Value) -> Velocity {
        let vel = val.lookup("vel").unwrap();

        Velocity { vel : vel.as_float().unwrap() as f32 }
    }
}

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
