use std::collections::{HashMap, HashSet};


pub use entity::*;
pub use component::*;
pub use system::*;
pub use aspect::*;

pub struct World {
    pub entities       : HashMap<i32, Entity>,
    pub systems        : Vec<(Box<System>, Aspect)>,
    pub active_systems : Vec<HashSet<i32>>,
    last_id            : i32
}
impl World {
    pub fn new() -> World {
        World {
            last_id        : 0,
            entities       : HashMap::new(),
            systems        : Vec::new(),
            active_systems : Vec::new()
        }
    }

    pub fn create_entity(&mut self) -> i32 {
        self.last_id += 1;
        let e = Entity::new(self.last_id);
        self.entities.insert(self.last_id, e);
        self.last_id
    }

    pub fn refresh_entity(&mut self, entity_id : i32) {
        let e = &mut self.entities.get_mut(&entity_id).unwrap();
        for (i, &(ref system, ref registred)) in self.systems.iter().enumerate() {
            let entities = self.active_systems.get_mut(i).unwrap();

            if registred.check(e) {
                if entities.contains(&entity_id) == false {
                    entities.insert(entity_id);
                    system.on_added(e);
                }
            } else {
                if entities.contains(&entity_id) {
                    entities.remove(&entity_id);
                    system.on_removed(e);
                }

            }
        }
    }
    pub fn set_system<TSys>(&mut self, system : TSys, types : Aspect)
        where TSys : 'static + System {
        self.systems.push((Box::new(system), types));
        self.active_systems.push(HashSet::new());
        for (_, e) in self.entities.iter_mut() {
            e.fresh = false;
        }
    }

    pub fn update(&mut self) {
        let ids = self.entities.iter().
            filter_map(|(_, e)| {
                if e.fresh {
                    None
                } else {
                    Some(e.id)
                }}).
            collect::<Vec<i32>>();
        for i in ids {
            self.refresh_entity(i);
        }
        for (i, ref entities) in self.active_systems.iter().enumerate() {
            if entities.len() != 0 {
                self.systems.get_mut(i as usize).unwrap().0.on_begin_frame();
                for eid in *entities {
                    let entity = self.entities.get_mut(&eid).unwrap();
                    self.systems.get_mut(i as usize).unwrap().0.process(entity);
                }
                self.systems.get_mut(i as usize).unwrap().0.on_end_frame();
            }
        }
    }
}
