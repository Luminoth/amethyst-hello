use amethyst::core::SystemDesc;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{System, SystemData, World};

#[derive(SystemDesc)]
pub struct DebugSystem;

impl<'a> System<'a> for DebugSystem {
    type SystemData = ();

    fn run(&mut self, _data: Self::SystemData) {
        /*amethyst_imgui::with(|ui| {
            ui.show_demo_window(&mut true);
        });*/
    }
}
