use std::rc::Rc;
use std::cell::RefCell;

#[macro_use] extern crate tinyecs;

use tinyecs::*;

pub struct GlutinFacade;
impl GlutinFacade {
    pub fn new() -> GlutinFacade {
        GlutinFacade
    }
    pub fn draw_something(&mut self, some : &str) {
        println!("{}", some);
    }
}
pub struct HeavyGuiData;
impl Component for HeavyGuiData {}
impl HeavyGuiData {
    pub fn new(_ : &GlutinFacade) -> HeavyGuiData {
        HeavyGuiData
    }
}

transit_system!(Glutin2HeavySystem: RenderData => HeavyGuiData,
    |render_data| { HeavyGuiData::new(&render_data.facade.borrow_mut() ) });

pub struct Renderable;
impl Component for Renderable {}

pub struct GuiWindow;
impl Component for GuiWindow {}

pub struct RenderData {
    facade : Rc<RefCell<GlutinFacade>>
}
impl Component for RenderData {}


pub struct RenderSystem {
    facade : Rc<RefCell<GlutinFacade>>
}
impl RenderSystem {
    pub fn new() -> RenderSystem {
        RenderSystem {
            facade : Rc::new(RefCell::new(GlutinFacade::new()))
        }
    }
}

impl System for RenderSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(Renderable)
    }
    fn on_created(&mut self, entity_manager : &mut EntityManager) {
        let data_entity = entity_manager.create_entity();
        data_entity.add_component(RenderData {facade : self.facade.clone()});
    }
    fn process_one(&mut self, _ : &mut Entity) {
        self.facade.borrow_mut().draw_something("triangles triangles");
    }
}

pub struct GuiSystem;

impl System for GuiSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(GuiWindow)
    }
    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all!(RenderData, HeavyGuiData)]
    }

    fn process_d(&mut self, _ : &mut Entity, data : &mut DataList) {
        let render_data = data.unwrap_entity().get_component::<RenderData>();
        let _gui_data   = data.unwrap_entity().get_component::<HeavyGuiData>();
        render_data.facade.borrow_mut().draw_something("gui gui gui");
    }
}

fn main() {
    let mut world = World::new();
    world.set_system(RenderSystem::new());
    world.set_system(GuiSystem);
    world.set_system(Glutin2HeavySystem);

    {
        let mut entity_manager = world.entity_manager();

        {
            let mesh = entity_manager.create_entity();

            mesh.add_component(Renderable);
            mesh.refresh();
        }
        {
            let window = entity_manager.create_entity();

            window.add_component(GuiWindow);
            window.refresh();
        }
    }
    world.update();
    world.update();
    world.update();
}
