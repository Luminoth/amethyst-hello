use amethyst::ecs::*;

#[derive(Default)]
pub struct DebugSystem;

impl<'a> System<'a> for DebugSystem {
    type SystemData = ();

    fn run(&mut self, _data: Self::SystemData) {
        amethyst_imgui::with(|ui| {
            ui.show_demo_window(&mut true);
        });
    }
}
