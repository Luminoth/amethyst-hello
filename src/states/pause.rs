use amethyst::input::{is_key_down, VirtualKeyCode};
use amethyst::prelude::*;
use log::debug;

pub struct PauseState;

impl SimpleState for PauseState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        debug!("Entering pause state");
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        debug!("Leaving pause state");
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
