use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::core::transform::Transform;
use amethyst::core::{ArcThreadPool, Time};
use amethyst::ecs::prelude::*;
use amethyst::input::{is_key_down, VirtualKeyCode};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture,
};
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};
use log::debug;

use super::PauseState;

use crate::components::{BallComponent, PaddleComponent, PaddleSide, PADDLE_WIDTH};
use crate::systems;
use crate::{ScoreText, ARENA_HEIGHT, ARENA_WIDTH};

#[derive(PartialEq)]
pub enum RunningState {
    Running,
    Paused,
}

impl Default for RunningState {
    fn default() -> Self {
        RunningState::Running
    }
}

#[derive(Default)]
pub struct GameState<'a, 'b> {
    dispatcher: Option<Dispatcher<'a, 'b>>,

    game_start_timer: Option<f32>,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
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

    // create a sprint renderer component
    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 0,
    };

    // create the paddle entities
    world
        .create_entity()
        .with(left_transform)
        .with(sprite_render.clone())
        .with(PaddleComponent::new(PaddleSide::Left))
        .build();

    world
        .create_entity()
        .with(right_transform)
        .with(sprite_render.clone())
        .with(PaddleComponent::new(PaddleSide::Right))
        .build();
}

fn initialize_ball(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    // create the transform component
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 0.0);

    // create a sprint renderer component
    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 1,
    };

    // create the ball entity
    world
        .create_entity()
        .with(transform)
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

impl<'a, 'b> SimpleState for GameState<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        debug!("Entering game state");

        let mut world = data.world;

        let mut dispatcher_builder = DispatcherBuilder::new();

        // add game systems
        dispatcher_builder.add(
            systems::PaddleInputSystem::default().pausable(RunningState::Running),
            "paddle_input_system",
            &[],
        );
        dispatcher_builder.add(
            systems::BallMovementSystem::default().pausable(RunningState::Running),
            "ball_movement_system",
            &[],
        );
        dispatcher_builder.add(
            systems::BallCollisionSystem::default().pausable(RunningState::Running),
            "ball_collision_system",
            &["paddle_input_system", "ball_movement_system"],
        );
        dispatcher_builder.add(
            systems::ScoreSystem::default().pausable(RunningState::Running),
            "score_system",
            &["ball_movement_system"],
        );

        let mut dispatcher = dispatcher_builder
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);
        self.dispatcher.replace(dispatcher);

        // init internal state
        self.game_start_timer.replace(1.0);

        // load resources
        self.sprite_sheet_handle.replace(load_sprite_sheet(world));

        // initialize entities
        initialize_camera(&mut world);
        initialize_paddles(&mut world, self.sprite_sheet_handle.clone().unwrap());
        initialize_scoreboard(&mut world);
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        debug!("Leaving game state");
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // run the dispatcher
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }

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
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Push(Box::new(PauseState));
            }
        }

        Trans::None
    }
}
