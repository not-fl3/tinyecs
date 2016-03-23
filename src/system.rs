use entity::*;
use world_data::*;

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

    fn process(&mut self, _ : &mut Entity, _ : &mut WorldData) {
    }

    fn process_with_data(&mut self,
                         entity : &mut Entity,
                         world : &mut WorldData,
                         _ : &mut SomeData) {
        self.process(entity, world);
    }

    fn process_all(&mut self, _ : &mut Vec<&mut Entity>, _ : &mut SomeData) {
    }

}
