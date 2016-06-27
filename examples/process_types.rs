#[macro_use] extern crate tinyecs;

use tinyecs::*;

pub struct Position {
    pub pos : [f32; 3]
}
impl Component for Position {}

pub struct Mesh {
    pub mesh : String
}
impl Component for Mesh {}

pub struct DeferMesh {
    pub order : i32
}
impl Component for DeferMesh {}

pub struct Camera {
    pub pos : [f32; 3]
}
impl Component for Camera {}

pub struct RenderSystem;

impl System for RenderSystem {
    fn aspect(&self) -> Aspect {
        aspect_all![Position, Mesh]
    }
    fn data_aspects(&self) -> Vec<Aspect> {
        vec![Aspect::all::<Camera>()]
    }
    fn process_d(&mut self, entity : &mut Entity, data : &mut DataList) {
        let cam = data.unwrap_entity();
        let cam = cam.get_component::<Camera>();

        let pos = entity.get_component::<Position>();
        let mesh = entity.get_component::<Mesh>();

        println!("{}, {}, seen from camera pos: {:?}", mesh.mesh, pos.pos[0], cam.pos);
    }
}

pub struct DeferRenderSystem;

impl System for DeferRenderSystem {
    fn aspect(&self) -> Aspect {
        aspect_all![Position, Mesh]
    }
    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all![Camera]]
    }
    fn process_all(&mut self,
                   entities : &mut Vec<&mut Entity>,
                   _   : &mut WorldHandle,
                   data   : &mut DataList) {
        entities.sort_by(|e1, e2| {
            let defer1 = e1.get_component::<DeferMesh>();
            let defer2 = e2.get_component::<DeferMesh>();
            defer1.order.cmp(&defer2.order)
        });
        for entity in entities {
            let cam = data.unwrap_entity();
            let cam = cam.get_component::<Camera>();

            let mesh = entity.get_component::<Mesh>();

            println!("{}, seen from camera pos: {:?}", mesh.mesh, cam.pos);
        }
    }
}

fn main() {
    let mut world = World::new();

    {
        let mut entity_manager = world.entity_manager();
        let entity = entity_manager.create_entity();

        entity.add_component(Position {pos : [0.0, 0.0, 0.0]});
        entity.add_component(Mesh {mesh : "player".to_string()});
        entity.refresh();
    }

    {
        let mut entity_manager = world.entity_manager();
        let entity = entity_manager.create_entity();
        entity.add_component(Camera {pos : [0.0, 0.0, 0.0]});
        entity.refresh();
    }
    // will process all entities with Position and Mesh,
    // and in this process all entities with Camera will be accessable
    world.set_system(RenderSystem);


    // same system, but we will use another implementetion inside it, for processing all entities at once
    world.set_system(DeferRenderSystem);

    world.update();
    world.update();
}
