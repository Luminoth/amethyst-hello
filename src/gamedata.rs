use amethyst::core::bundle::SystemBundle;
use amethyst::core::deferred_dispatcher_operation::{
    AddBarrier, AddBundle, AddSystem, DispatcherOperation,
};
use amethyst::core::ArcThreadPool;
use amethyst::ecs::prelude::*;
use amethyst::prelude::*;
use amethyst::DataDispose;

// NOTE: a lot of this is copied from the base GameData / GameDataBuilder implementation
// there doesn't seem to be a more "correct" way to go about it

pub struct CustomGameData<'a, 'b> {
    engine_dispatcher: Option<Dispatcher<'a, 'b>>,
    game_dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> CustomGameData<'a, 'b> {
    fn update_engine(&mut self, world: &World) {
        if let Some(engine_dispatcher) = &mut self.engine_dispatcher {
            engine_dispatcher.dispatch(world);
        }
    }

    pub fn update_loading(&mut self, world: &World) {
        self.update_engine(world);
    }

    pub fn update_main_menu(&mut self, world: &World) {
        self.update_engine(world);
    }

    pub fn update_game(&mut self, world: &World) {
        if let Some(game_dispatcher) = &mut self.game_dispatcher {
            game_dispatcher.dispatch(world);
        }

        self.update_engine(world);
    }

    pub fn update_paused(&mut self, world: &World) {
        self.update_engine(world);
    }

    pub fn dispose(&mut self, mut world: &mut World) {
        if let Some(game_dispatcher) = self.game_dispatcher.take() {
            game_dispatcher.dispose(&mut world);
        }

        if let Some(engine_dispatcher) = self.engine_dispatcher.take() {
            engine_dispatcher.dispose(&mut world);
        }
    }
}

impl DataDispose for CustomGameData<'_, '_> {
    fn dispose(&mut self, world: &mut World) {
        self.dispose(world);
    }
}

pub struct CustomGameDataBuilder<'a, 'b> {
    engine_operations: Vec<Box<dyn DispatcherOperation<'a, 'b>>>,
    game_operations: Vec<Box<dyn DispatcherOperation<'a, 'b>>>,
}

impl<'a, 'b> Default for CustomGameDataBuilder<'a, 'b> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, 'b> CustomGameDataBuilder<'a, 'b> {
    pub fn new() -> Self {
        Self {
            engine_operations: Vec::new(),
            game_operations: Vec::new(),
        }
    }

    pub fn with_engine_barrier(mut self) -> Self {
        self.engine_operations.push(Box::new(AddBarrier));

        self
    }

    pub fn with_engine_bundle<B>(mut self, bundle: B) -> Self
    where
        B: SystemBundle<'a, 'b> + 'static,
    {
        self.engine_operations.push(Box::new(AddBundle { bundle }));

        self
    }

    pub fn with_engine<S, N>(mut self, system: S, name: N, dependencies: &[N]) -> Self
    where
        S: for<'c> System<'c> + 'static + Send,
        N: Into<String> + Clone,
    {
        let name = name.into();
        let dependencies = dependencies
            .iter()
            .map(Clone::clone)
            .map(Into::into)
            .collect();

        let dispatcher_operation = Box::new(AddSystem {
            system,
            name,
            dependencies,
        }) as Box<dyn DispatcherOperation<'a, 'b> + 'static>;

        self.engine_operations.push(dispatcher_operation);

        self
    }

    #[allow(dead_code)]
    pub fn with_game_barrier(mut self) -> Self {
        self.game_operations.push(Box::new(AddBarrier));

        self
    }

    pub fn with_game_bundle<B>(mut self, bundle: B) -> Self
    where
        B: SystemBundle<'a, 'b> + 'static,
    {
        self.game_operations.push(Box::new(AddBundle { bundle }));

        self
    }

    #[allow(dead_code)]
    pub fn with_game<S, N>(mut self, system: S, name: N, dependencies: &[N]) -> Self
    where
        S: for<'c> System<'c> + 'static + Send,
        N: Into<String> + Clone,
    {
        let name = name.into();
        let dependencies = dependencies
            .iter()
            .map(Clone::clone)
            .map(Into::into)
            .collect();

        let dispatcher_operation = Box::new(AddSystem {
            system,
            name,
            dependencies,
        }) as Box<dyn DispatcherOperation<'a, 'b> + 'static>;

        self.game_operations.push(dispatcher_operation);

        self
    }
}

impl<'a, 'b> DataInit<CustomGameData<'a, 'b>> for CustomGameDataBuilder<'a, 'b> {
    fn build(self, world: &mut World) -> CustomGameData<'a, 'b> {
        let engine_dispatcher = build_dispatcher(world, self.engine_operations);
        let game_dispatcher = build_dispatcher(world, self.game_operations);

        CustomGameData {
            engine_dispatcher: Some(engine_dispatcher),
            game_dispatcher: Some(game_dispatcher),
        }
    }
}

fn build_dispatcher<'a, 'b>(
    mut world: &mut World,
    dispatcher_operations: Vec<Box<dyn DispatcherOperation<'a, 'b>>>,
) -> Dispatcher<'a, 'b> {
    #[cfg(not(no_threading))]
    let pool = (*world.read_resource::<ArcThreadPool>()).clone();

    let mut dispatcher_builder = DispatcherBuilder::new();

    dispatcher_operations
        .into_iter()
        .try_for_each(|dispatcher_operation| {
            dispatcher_operation.exec(world, &mut dispatcher_builder)
        })
        .unwrap_or_else(|e| panic!("Failed to set up dispatcher: {}", e));

    #[cfg(not(no_threading))]
    let mut dispatcher = dispatcher_builder.with_pool(pool).build();
    #[cfg(no_threading)]
    let mut dispatcher = dispatcher_builder.build();
    dispatcher.setup(&mut world);

    dispatcher
}
