use amethyst::prelude::*;
use log::debug;

use super::MenuState;

use crate::gamedata::CustomGameData;

#[derive(Default)]
pub struct LoadingState;

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for LoadingState {
    fn on_start(&mut self, _data: StateData<'_, CustomGameData<'_, '_>>) {
        debug!("Entering loading state");
    }

    fn on_stop(&mut self, _data: StateData<'_, CustomGameData<'_, '_>>) {
        debug!("Leaving loading state");
    }

    fn update(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update_loading(&data.world);

        Trans::Switch(Box::new(MenuState))
    }
}
