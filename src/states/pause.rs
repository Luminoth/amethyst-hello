use amethyst::input::{is_key_down, VirtualKeyCode};
use amethyst::prelude::*;
use log::debug;

use super::game::RunningState;

pub struct PauseState;

impl SimpleState for PauseState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        debug!("Entering pause state");

        *data.world.write_resource::<RunningState>() = RunningState::Paused;
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        debug!("Leaving pause state");

        *data.world.write_resource::<RunningState>() = RunningState::Running;
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Pop;
            }
        }

        Trans::None
    }
}
