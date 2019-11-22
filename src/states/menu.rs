use amethyst::ecs::prelude::*;
use amethyst::input::is_close_requested;
use amethyst::prelude::*;
use amethyst::ui::UiCreator;
use log::debug;

use super::LoadingState;

use crate::gamedata::CustomGameData;

#[derive(Default)]
pub struct MenuState {
    menu_ui: Option<Entity>,
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for MenuState {
    fn on_start(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        debug!("Entering main menu state");

        self.menu_ui = Some(
            data.world
                .exec(|mut creator: UiCreator<'_>| creator.create("ui/menu.ron", ())),
        );
    }

    fn on_stop(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        debug!("Leaving main menu state");

        if let Some(entity) = self.menu_ui {
            let _ = data.world.delete_entity(entity);
        }
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, CustomGameData<'_, '_>>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = event {
            if is_close_requested(&event) {
                return Trans::Quit;
            }
        }

        Trans::None
    }

    fn update(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update_main_menu(&data.world);

        Trans::Switch(Box::new(LoadingState::default()))
    }
}
