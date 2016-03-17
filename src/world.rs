use std::collections::{HashSet};
//use time::PreciseTime;
use std::time;
use std::time::Instant;
use std::time::Duration;

pub use entity::*;
pub use component::*;
pub use system::*;
pub use aspect::*;
pub use world_data::*;
use fast_dict::*;

pub type EntityIdSet = HashSet<i32>;

pub struct World {
    pub entities       : FastDictionary<Entity>,
    pub systems        : FastDictionary<(Box<System>, Aspect, Vec<Aspect>)>,
    pub active_systems : FastDictionary<(EntityIdSet, Vec<EntityIdSet>)>,
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
    pub fn refresh_entity(&mut self, entity_id : i32) {
        let e = &mut self.entities.get_mut_no_check(entity_id as isize);
        for (i, &mut (ref mut system, ref aspect, ref data_aspects)) in
            self.systems.iter_mut().enumerate()
        {
            let entities = self.active_systems.get_mut(i).unwrap();

            if aspect.check(e) {
                if entities.0.contains(&entity_id) == false {
                    entities.0.insert(entity_id);
                    system.on_added(e);
                }
            } else {
                if entities.0.contains(&entity_id) {
                    entities.0.remove(&entity_id);
                    system.on_removed(e);
                }
            }

            if entities.1.len() != data_aspects.len() {
                entities.1.resize(data_aspects.len(), HashSet::new());
            }
            for (data_aspect, mut entities) in data_aspects.iter().
                zip(entities.1.iter_mut())
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
                                   (HashSet::new(),
                                    vec![HashSet::new(); data_aspects.len()]));
        let len = self.systems.vec.len();
        self.systems.insert(len, (Box::new(system), types, data_aspects));
        for e in self.entities.iter_mut() {
            e.fresh = false;
        }
    }

    pub fn update(&mut self) {
        let delta = self.update_time.elapsed();
        let float_delta = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1000000000.0;
        let mut world_data = WorldData { delta : float_delta };
        self.update_time = Instant::now();
        let ids = self.entities.iter().
            filter_map(|e| {
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
            if entities.0.len() != 0 {
                let mut system = &mut self.systems.get_mut(i as usize).unwrap().0;
                (**system).on_begin_frame();
                for eid in &entities.0 {
                    if entities.1.len() == 0 {
                        let entity = self.entities.get_mut_no_check(*eid as isize);
                        (**system).process_with_data(entity, &mut world_data, &mut SomeData::None);
                    } else {
                        for eid1 in &entities.1[0] {
                            let (entity, data_entity) = self.entities.get2_mut_no_check(*eid as isize, *eid1 as isize);
                            (**system).process_with_data(entity, &mut world_data, &mut SomeData::Entity(data_entity));
                        }
                    }
                }
                (**system).on_end_frame();
            }
        }
    }
}
