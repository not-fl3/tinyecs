use std::collections::HashSet;
use time::PreciseTime;
use vec_map::VecMap;


pub use entity::*;
pub use component::*;
pub use system::*;
pub use aspect::*;

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
    entities         : VecMap<Entity>,
    systems          : Vec<(SystemData, SelectedEntities)>,
    update_time      : PreciseTime,
    deserializers    : ::std::rc::Rc<Box<::serialization::Deserializers>>,
    last_id          : i32,
}

/// part of the world, manipulating entities
pub struct EntityManager<'a> {
    deserializers : ::std::rc::Rc<Box<::serialization::Deserializers>>,
    new_entities  : Vec<(i32, Entity)>,
    entities      : &'a mut VecMap<Entity>,
    last_id       : &'a mut i32
}
impl<'a> Drop for EntityManager<'a> {
    fn drop(&mut self) {
        self.apply_creation();
    }
}

impl<'a> EntityManager<'a> {
    pub fn create_entity_with_id(&mut self, id : i32) -> &Entity {
        (*self.last_id) = id;

        let e = Entity::new(id);

        self.new_entities.push((id,  e));

        &self.new_entities.last().unwrap().1
    }

    pub fn create_entity(&mut self) -> &Entity {
        *self.last_id += 1;
        let id = self.last_id.clone();
        self.create_entity_with_id(id)
    }

    pub fn try_get_entity(&mut self, id : i32) -> Option<&mut Entity> {
       self.entities.get_mut(id as usize)
    }

    pub fn get_entities_by_aspect(&mut self, aspect : Aspect) -> Vec<&'a mut Entity> {
        self.entities.iter_mut().filter_map(|(_, entity)| {
            if aspect.check(entity) {
                Some(unsafe {
                    ::std::mem::transmute(entity)
                })
            } else {
                None
            }
        }).collect::<Vec<_>>()
    }

    pub fn deserializers(&self) -> ::std::rc::Rc<Box<::serialization::Deserializers>> {
        self.deserializers.clone()
    }
    pub fn get_entities_by_ids(&mut self, ids : &HashSet<i32>) -> Vec<&'a mut Entity> {
        ids.iter().map(|id| {
            let e : &mut Entity = self.entities.get_mut(*id as usize).unwrap();
            unsafe {
                ::std::mem::transmute(e)
            }
        }).collect::<Vec<_>>()
    }

    pub fn remove_entity(&mut self, id : i32) {
       self.entities.get_mut(id as usize).unwrap().delete()
    }

    pub fn apply_creation(&mut self) {
        for (id, entity) in self.new_entities.drain(0 ..) {
            if let Some(_) = self.entities.insert(id as usize, entity) {
                panic!("Inserting to already existing entity!");
            }
        }
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
    pub fn new() -> World {
        World {
            last_id       : 0,
            update_time   : PreciseTime::now(),
            entities      : VecMap::with_capacity(3000),
            systems       : Vec::new(),
            deserializers : ::std::rc::Rc::new(Box::new(()))
        }
    }

    /// add deserializers for more custom components
    /// TODO not really add, just replace
    pub fn add_deserializers(&mut self, deserializers : Box<::serialization::Deserializers>) {
        self.deserializers = ::std::rc::Rc::new(deserializers);
    }

    /// Get entity manager for manupalating with entities.
    ///
    /// # Examples
    /// ```ignore
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
            last_id       : &mut self.last_id,
            entities      : &mut self.entities,
            deserializers : self.deserializers.clone(),
            new_entities  : Vec::new()
        }
    }

    /// Add new active system.
    pub fn set_system<TSys>(&mut self, mut system : TSys)
        where TSys : 'static + System {
        let aspect = system.aspect();
        let data_aspects = system.data_aspects();

        system.on_created(&mut EntityManager {
            last_id       : &mut self.last_id,
            entities      : &mut self.entities,
            deserializers : self.deserializers.clone(),
            new_entities  : Vec::new()
        });
        self.systems.push((SystemData::new(Box::new(system), aspect, data_aspects),
                                        SelectedEntities {
                                            entity_set : HashSet::new(),
                                            data_set   : vec![HashSet::new(); 0]
                                        }));
        for (_, e) in self.entities.iter_mut() {
            e.refresh();
        }
    }

    /// Tick all systems in world.
    /// All on_added and on_removed will passed inside this method.
    pub fn update(&mut self) {
        let delta = self.update_time.to(PreciseTime::now());
        let float_delta = delta.num_seconds() as f32 + delta.num_milliseconds() as f32 / 1000.0;

        self.update_time = PreciseTime::now();

        let mut systems = &mut self.systems;

        {
            profile_region!("refresh entities");
            for (_, e) in self.entities.iter_mut().filter(|&(_, ref e)| {e.is_fresh() == false || e.is_deleted() }) {
                let deleted = e.is_deleted();
                Self::refresh_entity(e, systems, deleted);
                e.set_fresh();
            }
        }

        let mut world_data = WorldHandle {
            delta    : float_delta,
            entity_manager    : EntityManager {
                last_id       : &mut self.last_id,
                entities      : &mut self.entities,
                deserializers : self.deserializers.clone(),
                new_entities  : Vec::new()
            }
        };


        {
            profile_region!("all begin frames");
            for &mut (ref mut system, ref entities) in systems.iter_mut() {
                if entities.entity_set.len() != 0 {
                    profile_region!(&format!("on_begin_frame: {}", system.system.get_name()));
                    (*system.system).on_begin_frame();
                }
            }
        }
        {
            profile_region!("all updates");
            for &mut (ref mut system, ref mut entities) in systems.iter_mut() {
                if entities.entity_set.len() != 0 {
                    let mut refs = world_data.entity_manager.get_entities_by_ids(&entities.entity_set);

                    {
                        profile_region!(&system.system.get_name());
                        let data_len = system.data_aspects.len();

                        if (0 .. data_len).any(|n| {
                            system.data_aspects[n].optional == false && entities.data_set[n].len() == 0
                        }) == false {
                            let mut some_data = DataList::new(&mut world_data.entity_manager, &entities.data_set);
                            (*system.system).process_all(&mut refs, &mut world_data, &mut some_data);
                        } else {
                            (*system.system).process_no_data();
                        }
                    }
                } else {
                    (*system.system).process_no_entities();
                }
            }
        }

        {
            profile_region!("all end frames");
            for &mut(ref mut system, ref entities) in systems.iter_mut() {
                if entities.entity_set.len() != 0 {
                    let mut refs = world_data.entity_manager.get_entities_by_ids(&entities.entity_set);

                    profile_region!(&format!("end_frame: {}", system.system.get_name()));
                    (*system.system).on_end_frame(&mut refs);
                }
            }
        }

    }

    fn refresh_entity(e : &mut Entity,
                      systems : &mut Vec<(SystemData, SelectedEntities)>,
                      deleted : bool) {
        {
            let mut deleted = e.removed_components.borrow_mut();
            let mut components = e.components.borrow_mut();

            for del in deleted.drain() {
                components.remove(&del);
            }
        }

        for & mut(SystemData { ref mut system, ref mut aspect, ref mut data_aspects }, ref mut entities) in systems.iter_mut() {
            if deleted == false && aspect.check(e) {
                if entities.entity_set.contains(&e.id) == false {
                    profile_region!(&format!("on_added: {}", system.get_name()));

                    entities.entity_set.insert(e.id);
                    system.on_added(e);
                }
            } else {
                if entities.entity_set.contains(&e.id) {
                    profile_region!(&format!("on_removed: {}", system.get_name()));
                    entities.entity_set.remove(&e.id);
                    system.on_removed(e);
                }
            }

            if entities.data_set.len() != data_aspects.len() {
                entities.data_set.resize(data_aspects.len(), HashSet::new());
            }
            for (data_aspect, mut entities) in data_aspects.iter().
                zip(entities.data_set.iter_mut())
            {
                if deleted == false && data_aspect.check(e) {
                    if entities.contains(&e.id) == false {
                        entities.insert(e.id);
                    }
                } else {
                    if entities.contains(&e.id) {
                        entities.remove(&e.id);
                    }
                }
            }
        }

    }

}
