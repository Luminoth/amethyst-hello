use amethyst::assets::{Completion, Handle, ProgressCounter};
use amethyst::ecs::prelude::*;
use amethyst::input::is_close_requested;
use amethyst::prelude::*;
use amethyst::ui::{UiCreator, UiLoader, UiPrefab};
use log::{debug, error, info};

use super::GameState;

use crate::gamedata::CustomGameData;

#[derive(Default)]
pub struct LoadingState {
    progress: ProgressCounter,

    //scene: Option<Handle<Prefab<MyPrefabData>>>,
    load_ui: Option<Entity>,
    paused_ui: Option<Handle<UiPrefab>>,
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for LoadingState {
    fn on_start(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        debug!("Entering loading state");

        // TODO: load the scene

        self.load_ui = Some(data.world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/fps.ron", &mut self.progress);
            creator.create("ui/loading.ron", &mut self.progress)
        }));

        self.paused_ui = Some(
            data.world
                .exec(|loader: UiLoader<'_>| loader.load("ui/paused.ron", &mut self.progress)),
        );
    }

    fn on_stop(&mut self, _data: StateData<'_, CustomGameData<'_, '_>>) {
        debug!("Leaving loading state");
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
        data.data.update_loading(&data.world);

        match self.progress.complete() {
            Completion::Failed => {
                error!("Failed loading assets");
                Trans::Quit
            }
            Completion::Complete => {
                info!("Assets loaded, swapping state");
                if let Some(entity) = self.load_ui {
                    let _ = data.world.delete_entity(entity);
                }
                Trans::Switch(Box::new(GameState::new(
                    //self.scene.as_ref().unwrap().clone(),
                    self.paused_ui.as_ref().unwrap().clone(),
                )))
            }
            Completion::Loading => Trans::None,
        }
    }
}
