use amethyst::prelude::*;
use log::debug;

use super::GameState;

pub struct MenuState;

impl SimpleState for MenuState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        debug!("Entering main menu state");
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        debug!("Leaving main menu state");
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::Switch(Box::new(GameState))
    }
}
