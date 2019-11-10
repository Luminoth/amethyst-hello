use amethyst::core::bundle::SystemBundle;
use amethyst::ecs::prelude::*;
use amethyst::error::Error;

#[derive(Default)]
pub struct EngineBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for EngineBundle {
    fn build(
        self,
        _world: &mut World,
        _builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        Ok(())
    }
}
