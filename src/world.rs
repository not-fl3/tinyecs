use std::collections::{HashSet};
use time::PreciseTime;

pub use entity::*;
pub use component::*;
pub use system::*;
pub use aspect::*;

use fast_dict::*;

type EntityIdSet = HashSet<i32>;

struct SystemData {
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
struct SelectedEntities {
    pub entity_set    : EntityIdSet,
    pub data_set      : Vec<EntityIdSet>
}

pub struct World {
    entities        : FastDictionary<Entity>,
    systems         : FastDictionary<SystemData>,
    active_systems  : FastDictionary<SelectedEntities>,
    update_time     : PreciseTime,
    last_id         : i32
}

/// part of the world, manipulating entities
pub struct EntityManager<'a> {
    entities : &'a mut FastDictionary<Entity>,
    last_id  : &'a mut i32
}
impl<'a> EntityManager<'a> {
    pub fn create_entity(&mut self) -> &mut Entity {
        (*self.last_id) += 1;
        let e = Entity::new(*self.last_id);
        self.entities.insert(*self.last_id as usize, e);
        self.entities.get_mut(*self.last_id as usize).unwrap()
    }
    pub fn try_get_entity(&mut self, id : i32) -> Option<&mut Entity> {
       self.entities.get_mut(id as usize)
    }

    pub fn get_entities_by_ids<'b>(&mut self, ids : &HashSet<i32>) -> Vec<&'b mut Entity> {
        ids.iter().map(|id| {
            let id = (*id).clone();
            self.entities.get_mut_no_check(id as isize)
        }).collect::<Vec<_>>()
    }
}

/// World handle, that will be accesible in system's process
pub struct WorldHandle<'a> {
    /// Delta from last world tick.
    pub delta          : f32,
    /// Entity manager with access to all worlds entities
    pub entity_manager : EntityManager<'a>
}

impl World {
    /// Constructs new Entity World.
    pub fn new() -> World {
        World {
            last_id        : 0,
            update_time    : PreciseTime::now(),
            entities       : FastDictionary::new(0),
            systems        : FastDictionary::new(0),
            active_systems : FastDictionary::new(0)
        }
    }

    /// Get entity manager for manupalating with entities.
    ///
    /// # Examples
    /// ```
    /// use tinyecs::*;
    /// let mut world = World::new();
    ///
    /// {
    ///   let mut entity_manager = world.entity_manager();
    ///   let _entity = entity_manager.create_entity();
    ///   // _entity.add_component(); or something
    /// }
    /// ```
    pub fn entity_manager<'a>(&'a mut self) -> EntityManager<'a> {
        EntityManager {
                last_id  : &mut self.last_id,
                entities : &mut self.entities
        }
    }

    /// Add new active system.
    pub fn set_system<TSys>(&mut self, mut system : TSys)
        where TSys : 'static + System {
        let aspect = system.aspect();
        let data_aspects = system.data_aspects();

        let len = self.active_systems.vec.len();
        self.active_systems.insert(len,
                                   SelectedEntities {
                                       entity_set : HashSet::new(),
                                       data_set   : vec![HashSet::new(); data_aspects.len()]
                                   });
        let len = self.systems.vec.len();

        system.on_created(&mut EntityManager {
            last_id  : &mut self.last_id,
            entities : &mut self.entities
        });

        self.systems.insert(len, SystemData::new(Box::new(system), aspect, data_aspects));
        for e in self.entities.iter_mut() {
            *e.fresh.borrow_mut() = false;
        }
    }

    /// Tick all systems in world.
    /// All on_added and on_removed will passed inside this method.
    pub fn update(&mut self) {
        let delta = self.update_time.to(PreciseTime::now());
        let float_delta = delta.num_seconds() as f32 + delta.num_milliseconds() as f32 / 1000.0;
        let mut world_data = WorldHandle {
            delta    : float_delta,
            entity_manager : EntityManager {
                last_id  : &mut self.last_id,
                entities : &mut self.entities
            }
        };

        self.update_time = PreciseTime::now();

        {
            profile_region!("refresh entities");
            for e in world_data.entity_manager.entities.iter_mut().filter(|e| {*e.fresh.borrow_mut() == false}) {
                Self::refresh_entity(e, &mut self.systems, &mut self.active_systems);
            }
        }

        {
            profile_region!("all begin frames");
            for (i, ref entities) in self.active_systems.iter().enumerate() {
                if entities.entity_set.len() != 0 {
                    let mut system = &mut self.systems.get_mut(i as usize).unwrap().system;
                    profile_region!(&format!("on_begin_frame: {}", system.get_name()));
                    (**system).on_begin_frame();
                }
            }
        }
        {
            profile_region!("all updates");
            for (i, ref entities) in self.active_systems.iter().enumerate() {
                if entities.entity_set.len() != 0 {
                    let mut refs = world_data.entity_manager.get_entities_by_ids(&entities.entity_set);

                    let mut system = &mut self.systems.get_mut(i as usize).unwrap();

                    {
                        profile_region!(&system.system.get_name());
                        if system.data_aspects.len() == 0 || (entities.data_set.len() != 0 &&
                                                              entities.data_set[0].len() != 0) {
                            let mut some_data = DataList::new(&mut world_data.entity_manager,
                                                              &entities.data_set);
                            system.system.process_all(&mut refs, &mut world_data, &mut some_data);
                        }
                    }
                }
            }
        }
        {
            profile_region!("all end frames");
            for (i, ref entities) in self.active_systems.iter().enumerate() {
                if entities.entity_set.len() != 0 {
                    let mut system = &mut self.systems.get_mut(i as usize).unwrap().system;

                    {
                        profile_region!(&format!("end_frame: {}", system.get_name()));
                        (**system).on_end_frame();
                    }
                }
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
                    profile_region!(&format!("on_added: {}", system.get_name()));

                    entities.entity_set.insert(entity_id);
                    system.on_added(e);
                }
            } else {
                if entities.entity_set.contains(&entity_id) {
                    profile_region!(&format!("on_removed: {}", system.get_name()));

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

        *e.fresh.borrow_mut() = true;
    }

}
