use entity::*;

pub trait System {
    fn on_begin_frame(&mut self) {
    }
    fn on_added(&mut self, _ : &mut Entity) {
    }
    fn on_removed(&self, _ : &mut Entity) {
    }

    fn on_end_frame(&mut self) {
    }

    fn process(&mut self, entity : &mut Entity);
}
