use amethyst::prelude::*;
use log::debug;

use super::GameState;

use crate::gamedata::CustomGameData;

#[derive(Default)]
pub struct MenuState;

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for MenuState {
    fn on_start(&mut self, _data: StateData<'_, CustomGameData<'_, '_>>) {
        debug!("Entering main menu state");
    }

    fn on_stop(&mut self, _data: StateData<'_, CustomGameData<'_, '_>>) {
        debug!("Leaving main menu state");
    }

    fn update(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update_main_menu(&data.world);

        Trans::Switch(Box::new(GameState::default()))
    }
}
