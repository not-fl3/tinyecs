use entity::*;

pub trait System {
    fn on_added(&self, entity : &mut Entity) {
        println!("added {}", entity.id);
    }
    fn process(&self, entity : &mut Entity);
}
