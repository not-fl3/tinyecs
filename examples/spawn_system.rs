#[macro_use] extern crate tinyecs;

use tinyecs::*;

pub struct SomeComponent {
    some_data : String
}
impl Component for SomeComponent {}

pub struct SpawnPoint {
    data  : &'static str,
    count : i32
}
impl Component for SpawnPoint {}

pub struct SpawnSystem;
impl System for SpawnSystem {
    fn aspect(&self) -> Aspect {
        Aspect::all::<SpawnPoint>()
    }

    fn process_w(&mut self, entity : &mut Entity, world : &mut WorldData) {
        let mut spawn_point = entity.get_component::<SpawnPoint>();

        if spawn_point.count > 0 {
            let spawned = world.entity_manager.create_entity();
            spawned.add_component(SomeComponent { some_data : spawn_point.data.to_string() });
            spawned.refresh();

            spawn_point.count -= 1;
        }
    }
}

fn main() {
    let mut world = World::new();

    {
        let mut w = world.entity_manager();
        let entity = w.create_entity();

        entity.add_component(SpawnPoint {data : "player", count : 5});
        entity.refresh();
    }

    world.set_system(SpawnSystem);

    world.update();
    world.update();
}