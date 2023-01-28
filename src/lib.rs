pub mod bind;
pub mod builder;
pub mod buttons;
pub mod modifiers;

use bevy::prelude::*;
use buttons::*;
pub struct UiBuilderPlugin;

impl Plugin for UiBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ToggleButton>()
            .register_type::<ActionOnRelease>()
            .register_type::<MouseButtonMask>()
            .register_type::<ToggleButtonGroup>()
            .register_type::<Disabled>()
            .register_type::<ButtonVisualState>()
            .register_type::<ImageButton>()
            .register_type::<ColorButton>()
            .register_type::<ButtonInternalState>()
            .add_system(button_system);
    }
}
