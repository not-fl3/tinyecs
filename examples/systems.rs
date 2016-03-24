#[macro_use] extern crate tinyecs;

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

    fn process_one(&mut self, entity : &mut Entity) {
        println!("{}", entity.get_component::<Position>().pos[0]);
    }
}

pub struct DeadDrawerSystem;
impl System for DeadDrawerSystem {
    fn process_one(&mut self, entity : &mut Entity) {
        println!("is dead {}", entity.get_component::<Position>().pos[0]);
    }
}

pub struct MoverSystem;
impl System for MoverSystem {
    fn process_one(&mut self, entity : &mut Entity) {
        let t : &mut Position = entity.get_component::<Position>();
        t.pos[0] += 0.1;
        println!("Moved! {}", t.pos[0]);
    }
}


fn main() {
    let mut world = World::new();
    let eid = world.create_entity();
    {
        let mut entity = world.try_get_entity(eid).unwrap();
        entity.add_component(Position {pos : [0.0, 0.0, 0.0]});
        entity.refresh();
    }

    // if you have position, you will be drawn
    world.set_system(DrawerSystem, Aspect::all::<Position>());
    // except you are dead
    world.set_system(MoverSystem, Aspect::all::<Position>().except::<Dead>());
    // but only if you are dead your corpse will be draw, too
    world.set_system(DeadDrawerSystem, Aspect::all2::<Position, Dead>());

    world.update();
    world.update();
}
