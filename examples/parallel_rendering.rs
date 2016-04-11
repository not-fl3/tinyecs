#[macro_use] extern crate tinyecs;

use tinyecs::*;

pub struct Position {
    pub pos : [f32; 3]
}
impl Component for Position {}

pub struct Renderable;
impl Component for Renderable {}

pub struct RenderSystem;
impl System for RenderSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(Renderable)
    }

    fn on_added(&mut self, entity : &mut Entity) {
        println!("drawer added {}", entity.id);
    }

    fn process_one(&mut self, entity : &mut Entity) {
        let _ = entity.get_component::<Renderable>();
        println!("rendering entity: {}", entity.id);
    }
}

pub struct MoverSystem;
impl System for MoverSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(Position)
    }

    fn process_one(&mut self, entity : &mut Entity) {
        let mut pos = entity.get_component::<Position>();
        pos.pos[0] += 0.1;
        println!("Moved! {}", pos.pos[0]);
    }
}


fn main() {
    let mut world = World::new();

    {
        let mut entity_manager = world.entity_manager();
        let entity = entity_manager.create_entity();
        entity.add_component(Position {pos : [0.0, 0.0, 0.0]});
        entity.add_component(Renderable);
        entity.refresh();
    }

    world.set_parallel_system(RenderSystem, 1);
    world.set_system(MoverSystem);

    world.update();
    world.update();
}
