use amethyst::input::{is_key_down, VirtualKeyCode};
use amethyst::prelude::*;
use log::debug;

use crate::gamedata::CustomGameData;
use crate::RunningState;

#[derive(Default)]
pub struct PauseState;

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for PauseState {
    fn on_start(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        debug!("Entering pause state");

        *data.world.write_resource::<RunningState>() = RunningState::Paused;
    }

    fn on_stop(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        debug!("Leaving pause state");

        *data.world.write_resource::<RunningState>() = RunningState::Running;
    }

    fn update(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update_paused(&data.world);

        Trans::None
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, CustomGameData<'_, '_>>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Pop;
            }
        }

        Trans::None
    }
}
