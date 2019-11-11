use amethyst::assets::{AssetStorage, Handle, Loader, PrefabLoader, RonFormat};
use amethyst::audio::{AudioSink, OggFormat};
use amethyst::core::math::Vector3;
use amethyst::core::transform::Transform;
use amethyst::core::Time;
use amethyst::ecs::prelude::*;
use amethyst::input::{is_key_down, VirtualKeyCode};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,
};
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};
use log::debug;

use super::PauseState;

use crate::components::{BallComponent, BoundingBoxComponent, PaddleComponent, PaddleSide};
use crate::gamedata::CustomGameData;
use crate::prefabs::BallPrefab;
use crate::{
    Music, ScoreText, SoundEffects, ARENA_HEIGHT, ARENA_WIDTH, AUDIO_BOUNCE, AUDIO_MUSIC,
    AUDIO_SCORE, PADDLE_HEIGHT, PADDLE_WIDTH,
};

#[derive(Default)]
pub struct GameState {
    game_start_timer: Option<f32>,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

pub fn initialize_audio(world: &mut World) {
    let (sound_effects, music) = {
        let loader = world.read_resource::<Loader>();

        // reduce music volume
        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.25);

        let music = AUDIO_MUSIC
            .iter()
            .map(|file| loader.load(*file, OggFormat, (), &world.read_resource()))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();
        let music = Music { music };

        let sound_effects = SoundEffects {
            bounce: loader.load(AUDIO_BOUNCE, OggFormat, (), &world.read_resource()),
            score: loader.load(AUDIO_SCORE, OggFormat, (), &world.read_resource()),
        };

        (sound_effects, music)
    };

    world.insert(sound_effects);
    world.insert(music);
}

fn initialize_camera(world: &mut World) {
    // center the camera on the arena
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    // add the camera entity
    world
        .create_entity()
        .with(transform)
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // load the spritesheet texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "textures/pong_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    // load the spritesheet description
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "textures/pong_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialize_paddles(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    // create the transform components
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    let y = ARENA_HEIGHT * 0.5;
    left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    // create the bounds component
    let bounds = BoundingBoxComponent::new(
        Vector3::from_element(0.0),
        Vector3::new(PADDLE_WIDTH, PADDLE_HEIGHT, 0.0),
    );

    // create a sprint renderer component
    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 0,
    };

    // create the paddle entities
    world
        .create_entity()
        .with(left_transform)
        .with(bounds.clone())
        .with(sprite_render.clone())
        .with(PaddleComponent::new(PaddleSide::Left))
        .build();

    world
        .create_entity()
        .with(right_transform)
        .with(bounds.clone())
        .with(sprite_render.clone())
        .with(PaddleComponent::new(PaddleSide::Right))
        .build();
}

fn initialize_ball(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let prefab = world.exec(|loader: PrefabLoader<'_, BallPrefab>| {
        loader.load("prefabs/ball.ron", RonFormat, ())
    });

    // create the transform component
    /*let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 0.0);*/

    // create a sprint renderer component
    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 1,
    };

    // create the ball entity
    world
        .create_entity()
        /*.with(transform)
        .with(physical)
        .with(bounds)*/
        .with(prefab)
        .with(sprite_render)
        .with(BallComponent::default())
        .build();
}

fn initialize_scoreboard(world: &mut World) {
    // load the font
    let font = world.read_resource::<Loader>().load(
        "fonts/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    // create the score UI transform components
    let p1_transform = UiTransform::new(
        "P1".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        -50.0,
        -50.0,
        1.0,
        200.0,
        50.0,
    );

    let p2_transform = UiTransform::new(
        "P2".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        50.0,
        -50.0,
        1.0,
        200.0,
        50.0,
    );

    // create the score UI entities
    let p1_score = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            50.0,
        ))
        .build();

    let p2_score = world
        .create_entity()
        .with(p2_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1.0, 1.0, 1.0, 1.0],
            50.0,
        ))
        .build();

    // add the score text resource
    world.insert(ScoreText { p1_score, p2_score });
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for GameState {
    fn on_start(&mut self, data: StateData<'_, CustomGameData<'_, '_>>) {
        debug!("Entering game state");

        let world = data.world;

        // init internal state
        self.game_start_timer.replace(1.0);

        // load resources
        self.sprite_sheet_handle.replace(load_sprite_sheet(world));

        initialize_audio(world);

        // initialize entities
        initialize_camera(world);
        initialize_paddles(world, self.sprite_sheet_handle.clone().unwrap());
        initialize_scoreboard(world);
    }

    fn on_stop(&mut self, _data: StateData<'_, CustomGameData<'_, '_>>) {
        debug!("Leaving game state");
    }

    fn update(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update_game(&data.world);

        if let Some(mut timer) = self.game_start_timer.take() {
            // advance the timer
            {
                let time = data.world.fetch::<Time>();
                timer -= time.delta_seconds();
            }

            // spawn the ball when the game is ready
            if timer <= 0.0 {
                initialize_ball(data.world, self.sprite_sheet_handle.clone().unwrap());
            } else {
                self.game_start_timer.replace(timer);
            }
        }

        Trans::None
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, CustomGameData<'_, '_>>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Push(Box::new(PauseState));
            }
        }

        Trans::None
    }
}
