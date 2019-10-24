use amethyst::prelude::*;
use log::debug;

use super::MenuState;

pub struct LoadingState;

impl SimpleState for LoadingState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        debug!("Entering loading state");
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        debug!("Leaving loading state");
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::Switch(Box::new(MenuState))
    }
}
