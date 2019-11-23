use amethyst::core::bundle::SystemBundle;
use amethyst::ecs::prelude::*;
use amethyst::error::Error;

#[derive(Default)]
pub struct MenuBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for MenuBundle {
    fn build(
        self,
        _world: &mut World,
        _builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        /*builder.add(
            UiEventHandlerSystemDesc::default().build(world),
            "ui_event_handler",
            &[],
        );*/

        Ok(())
    }
}
