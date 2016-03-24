use std::collections::{HashSet};
use std::time::Instant;

pub use entity::*;
pub use component::*;
pub use system::*;
pub use aspect::*;
pub use world_data::*;
use fast_dict::*;

pub type EntityIdSet = HashSet<i32>;

pub struct SystemData {
    pub system       : Box<System>,
    pub aspect       : Aspect,
    pub data_aspects : Vec<Aspect>
}
impl SystemData {
    pub fn new(system : Box<System>, aspect : Aspect, data_aspects : Vec<Aspect>) -> SystemData {
        SystemData {
            system : system,
            aspect : aspect,
            data_aspects : data_aspects
        }
    }
}
pub struct SelectedEntities {
    pub entity_set    : EntityIdSet,
    pub data_set      : Vec<EntityIdSet>
}
pub struct World {
    pub entities       : FastDictionary<Entity>,
    pub systems        : FastDictionary<SystemData>,
    pub active_systems : FastDictionary<SelectedEntities>,
    update_time        : Instant,
    last_id            : i32
}
impl World {
    pub fn new() -> World {
        World {
            last_id        : 0,
            update_time    : Instant::now(),
            entities       : FastDictionary::new(0),
            systems        : FastDictionary::new(0),
            active_systems : FastDictionary::new(0)
        }
    }

    pub fn create_entity(&mut self) -> i32 {
        self.last_id += 1;
        let e = Entity::new(self.last_id);
        self.entities.insert(self.last_id as usize, e);
        self.last_id
    }

    pub fn try_get_entity(&mut self, id : i32) -> Option<&mut Entity> {
       self.entities.get_mut(id as usize)
    }
    pub fn set_system<TSys>(&mut self, system : TSys, types : Aspect)
        where TSys : 'static + System {
        self.set_system_with_data(system, types, Vec::new());
    }

    pub fn set_system_with_data<TSys>(&mut self,
                                      system : TSys,
                                      types : Aspect,
                                      data_aspects : Vec<Aspect>)
        where TSys : 'static + System {
        let len = self.active_systems.vec.len();
        self.active_systems.insert(len,
                                   SelectedEntities {
                                       entity_set : HashSet::new(),
                                       data_set   : vec![HashSet::new(); data_aspects.len()]
                                   });
        let len = self.systems.vec.len();
        self.systems.insert(len, SystemData::new(Box::new(system), types, data_aspects));
        for e in self.entities.iter_mut() {
            e.fresh = false;
        }
    }

    pub fn update(&mut self) {
        let delta = self.update_time.elapsed();
        let float_delta = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1000000000.0;
        let mut world_data = WorldData { delta : float_delta };

        self.update_time = Instant::now();

        for e in self.entities.iter_mut() {
            if e.fresh == false {
                Self::refresh_entity(e, &mut self.systems, &mut self.active_systems);
            }
        }

        for (i, ref entities) in self.active_systems.iter().enumerate() {
            if entities.entity_set.len() != 0 {
                let mut system = &mut self.systems.get_mut(i as usize).unwrap().system;
                (**system).on_begin_frame();
            }
        }

        for (i, ref entities) in self.active_systems.iter().enumerate() {
            if entities.entity_set.len() != 0 {
                let ids = entities.entity_set.clone();

                let ref mut e = self.entities;
                let mut refs  = ids.iter().map(|id| {
                    let id = (*id).clone();
                    e.get_mut_no_check(id as isize)
                }).collect::<Vec<_>>();
                let mut system = &mut self.systems.get_mut(i as usize).unwrap().system;
                if entities.data_set.len() == 0 {
                    system.process_all(&mut refs, &mut world_data, &mut SomeData::None)
                } else {
                    for eid1 in &entities.data_set[0] {
                        let data_entity = e.get_mut_no_check(*eid1 as isize);
                        system.process_all(&mut refs, &mut world_data, &mut SomeData::Entity(data_entity));
                    }

                }
            }
        }

        for (i, ref entities) in self.active_systems.iter().enumerate() {
            if entities.entity_set.len() != 0 {
                let mut system = &mut self.systems.get_mut(i as usize).unwrap().system;
                (**system).on_end_frame();
            }
        }
    }

    fn refresh_entity(e : &mut Entity,
                      systems : &mut FastDictionary<SystemData>,
                      active_systems : &mut FastDictionary<SelectedEntities>) {
        let entity_id = e.id;

        for (i, &mut SystemData { ref mut system, ref aspect, ref data_aspects }) in
            systems.iter_mut().enumerate()
        {
            let entities = active_systems.get_mut(i).unwrap();

            if aspect.check(e) {
                if entities.entity_set.contains(&entity_id) == false {
                    entities.entity_set.insert(entity_id);
                    system.on_added(e);
                }
            } else {
                if entities.entity_set.contains(&entity_id) {
                    entities.entity_set.remove(&entity_id);
                    system.on_removed(e);
                }
            }

            if entities.data_set.len() != data_aspects.len() {
                entities.data_set.resize(data_aspects.len(), HashSet::new());
            }
            for (data_aspect, mut entities) in data_aspects.iter().
                zip(entities.data_set.iter_mut())
            {
                if data_aspect.check(e) {
                    if entities.contains(&entity_id) == false {
                        entities.insert(entity_id);
                    }
                } else {
                    if entities.contains(&entity_id) {
                        entities.remove(&entity_id);
                    }
                }
            }
        }
    }

}


