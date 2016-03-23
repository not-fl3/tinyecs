extern crate tinyecs;

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
    fn process_with_data(&mut self,
                         entity : &mut Entity,
                         _      : &mut WorldData,
                         data   : &mut SomeData) {
        let cam = data.unwrap_entity();
        let cam = cam.get_component::<Camera>();

        let (_, mesh) = entity.get_components::<Position, Mesh>();

        println!("{}, seen from camera pos: {:?}", mesh.mesh, cam.pos);
    }
}

pub struct DeferRenderSystem;

impl System for DeferRenderSystem {
    fn process_all(&mut self,
                   entities : &mut Vec<&mut Entity>,
                   data   : &mut SomeData) {
        entities.sort_by(|mut e1, mut e2| {
            let defer1 = e1.get_component_nomut::<DeferMesh>().order;
            let defer2 = e2.get_component_nomut::<DeferMesh>().order;
            defer1.cmp(&defer2)
        });
        for entity in entities {
            let cam = data.unwrap_entity();
            let cam = cam.get_component::<Camera>();

            let (_, mesh) = entity.get_components::<Position, Mesh>();

            println!("{}, seen from camera pos: {:?}", mesh.mesh, cam.pos);
        }
    }
}

fn main() {
    let mut world = World::new();
    let player = world.create_entity();
    {
        let mut entity = world.try_get_entity(player).unwrap();
        entity.add_component(Position {pos : [0.0, 0.0, 0.0]});
        entity.add_component(Mesh {mesh : "player".to_string()});
        entity.refresh();
    }

    // will process all entities with Position and Mesh,
    // and in this process all entities with Camera will be accessable
    world.set_system_with_data(RenderSystem,
                               Aspect::all2::<Position, Mesh>(),
                               vec![Aspect::all::<Camera>()]);

    // same system, but we got all entities enstead of processing one by one
    world.set_system_with_data(DeferRenderSystem,
                               Aspect::all2::<Position, Mesh>(),
                               vec![Aspect::all::<Camera>()]);

    world.update();
    world.update();
}
