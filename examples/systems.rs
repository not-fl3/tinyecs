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
    fn aspect(&self) -> Aspect {
        Aspect::all::<Position>()
    }

    fn on_added(&mut self, entity : &mut Entity) {
        println!("drawer added {}", entity.id);
    }

    fn process_one(&mut self, entity : &mut Entity) {
        let pos = entity.get_component::<Position>();
        println!("{}", pos.pos[0]);
    }
}

pub struct DeadDrawerSystem;
impl System for DeadDrawerSystem {
    fn aspect(&self) -> Aspect {
        Aspect::all::<Position>().except::<Dead>()
    }
    fn process_one(&mut self, entity : &mut Entity) {
        let pos = entity.get_component::<Position>();
        println!("is dead {}", pos.pos[0]);
    }
}

pub struct MoverSystem;
impl System for MoverSystem {
    fn aspect(&self) -> Aspect {
        Aspect::all2::<Position, Dead>()
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
        entity.refresh();
    }

    // if you have position, you will be drawn
    world.set_system(DrawerSystem);
    // except you are dead
    world.set_system(MoverSystem);
    // but only if you are dead your corpse will be draw, too
    world.set_system(DeadDrawerSystem);

    world.update();
    world.update();
}
