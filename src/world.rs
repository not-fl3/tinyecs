use std::collections::HashMap;

pub use entity::*;
pub use component::*;
pub use system::*;

pub struct World {
    pub entities       : HashMap<i32, Entity>,
    pub systems        : Vec<(Box<System>, Vec<TypeId>)>,
    pub active_systems : Vec<Vec<i32>>,
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
        let e = Entity {
            id         : self.last_id,
            components : HashMap::new()
        };
        self.entities.insert(self.last_id, e);
        self.last_id
    }

    pub fn refresh_entity(&mut self, entity_id : i32) {
        let e = &mut self.entities.get_mut(&entity_id).unwrap();
        for (i, &(ref system, ref registred)) in self.systems.iter().enumerate() {
            if registred.iter().all(|ty| { e.components.contains_key(ty) }) {
                let entities = self.active_systems.get_mut(i).unwrap();
                if entities.contains(&entity_id) == false {
                    entities.push(entity_id);
                    system.on_added(e);
                }
            }
        }
    }
    pub fn set_system<TSys>(&mut self, system : TSys, types : Vec<TypeId>)
        where TSys : 'static + System {
        self.systems.push((Box::new(system), types));
        self.active_systems.push(Vec::new());
        let entities = self.entities.keys().map(|x| {*x}).collect::<Vec<i32>>();
        for eid in entities {
            self.refresh_entity(eid);
        }
    }

    pub fn update(&mut self) {
        for (i, ref entities) in self.active_systems.iter().enumerate() {
            if entities.len() != 0 {
                for eid in *entities {
                    let entity = self.entities.get_mut(&eid).unwrap();
                    self.systems.get(i as usize).unwrap().0.process(entity);
                }
            }
        }
    }
}
