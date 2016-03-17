use std::collections::{HashMap};

use entity::*;
use world_data::*;

pub enum SomeData<'a> {
    None,
    Entity(&'a mut Entity)
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

    fn process(&mut self, entity : &mut Entity, world : &mut WorldData) {
        self.process_with_data(entity, world, &mut SomeData::None);
    }

    fn process_with_data(&mut self, entity : &mut Entity, world : &mut WorldData, _ : &mut SomeData) {
        self.process(entity, world);
    }
}
