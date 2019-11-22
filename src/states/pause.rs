use amethyst::ecs::prelude::*;
use amethyst::input::{is_close_requested, is_key_down, VirtualKeyCode};
use amethyst::prelude::*;
use log::debug;

use crate::gamedata::CustomGameData;
use crate::RunningState;

pub struct PauseState {
    ui: Entity,
}

impl PauseState {
    pub fn new(ui: Entity) -> Self {
        Self { ui }
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for PauseState {
    fn on_start(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        debug!("Entering pause state");

        *data.world.write_resource::<RunningState>() = RunningState::Paused;
    }

    fn on_stop(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        debug!("Leaving pause state");

        *data.world.write_resource::<RunningState>() = RunningState::Running;
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) {
                return Trans::Quit;
            } else if is_key_down(&event, VirtualKeyCode::Escape) {
                let _ = data.world.delete_entity(self.ui);
                return Trans::Pop;
            }
        }

        Trans::None
    }

    fn update(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update_paused(&data.world);

        Trans::None
    }
}
