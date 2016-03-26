use entity::*;
use world::WorldData;

pub enum SomeData<'a> {
    None,
    Entity(&'a mut Entity)
}
impl<'b> SomeData<'b> {
    pub fn unwrap_entity<'a>(&'a mut self) -> &'a mut Entity {
        match self {
            &mut SomeData::None => panic!("not entity data"),
            &mut SomeData::Entity(ref mut e) => e
        }
    }
}

pub trait System {
    fn on_begin_frame(&mut self) {
    }

    fn on_added(&mut self, _ : &mut Entity) {
    }

    fn on_removed(&self, _ : &mut Entity) {
    }

    fn on_end_frame(&mut self) {
    }
    #[inline]
    fn process_w(&mut self, _ : &mut Entity, _ : &mut WorldData) {
    }

    #[inline]
    fn process_d(&mut self, _ : &mut Entity, _ : &mut SomeData) {
    }
    #[inline]
    fn process_wd(&mut self, _ : &mut Entity, _ : &mut WorldData, _ : &mut SomeData) {
    }

    #[inline]
    fn process_one(&mut self, _ : &mut Entity) {
    }

    fn process_all(&mut self, entities : &mut Vec<&mut Entity>, world: &mut WorldData, data : &mut SomeData) {
        for e in entities.iter_mut() {
            self.process_one(e);
            self.process_w(e, world);
            self.process_d(e, data);
            self.process_wd(e, world, data);
        }
    }
}

