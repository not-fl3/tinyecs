extern crate tinyecs;

use tinyecs::*;

pub struct Dead;
impl Component for Dead {}

pub struct Position {
    pub pos : [f32; 3]
}
impl Component for Position {}


pub struct DrawerSystem;
impl System for DrawerSystem {
    fn on_added(&mut self, entity : &mut Entity) {
        println!("drawer added {}", entity.id);
    }

    fn process(&mut self, entity : &mut Entity, world : &mut WorldData) {
        println!("{}", entity.get_component::<Position>().pos[0]);
    }
}

pub struct DeadDrawerSystem;
impl System for DeadDrawerSystem {
    fn process(&mut self, entity : &mut Entity, world : &mut WorldData) {
        println!("is dead {}", entity.get_component::<Position>().pos[0]);
    }
}

pub struct MoverSystem;
impl System for MoverSystem {
    fn on_added(&mut self, entity : &mut Entity) {
        println!("mover added {}", entity.id);
    }

    fn process(&mut self, entity : &mut Entity, world : &mut WorldData) {
        let t : &mut Position = entity.get_component::<Position>();
        t.pos[0] += 0.1;

    }
}


fn main() {
    let mut world = World::new();
    let eid = world.create_entity();
    {
        let mut entity = world.try_get_entity(eid).unwrap();
        entity.add_component(Position {pos : [0.0, 0.0, 0.0]});
    }

    {
        world.refresh_entity(eid);
    }

    world.set_system(MoverSystem, Aspect::all::<Position>());
    world.set_system(DrawerSystem, Aspect::all::<Position>());
    world.set_system(DeadDrawerSystem, Aspect::all2::<Position, Dead>());

    world.update();
    world.update();
}
