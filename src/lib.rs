mod bind;
mod builder;
mod buttons;
mod modifiers;
pub mod prelude;

use bevy::prelude::*;
use bind::*;
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

pub trait BindTrait {
    /// register T as bindable
    fn add_bind_source<T: Bindable>(&mut self) -> &mut Self;

    /// register pair <Parent, Child>, when Parent changed, Child will be updated
    fn add_bind_source_relation<Parent: Bindable, Child: Bindable>(&mut self) -> &mut Self;

    fn add_bind_component<T: Bindable, C: Component>(&mut self) -> &mut Self;
}

impl BindTrait for App {
    fn add_bind_source<T: Bindable>(&mut self) -> &mut Self {
        self.add_system(bind_system::<T>);
        self
    }

    fn add_bind_source_relation<Parent: Bindable, Child: Bindable>(&mut self) -> &mut Self {
        self.add_system(bind_parent_child_system::<Parent, Child>);
        self
    }

    fn add_bind_component<T: Bindable, C: Component>(&mut self) -> &mut Self {
        self.add_system(bind_generic_component::<T, C>);
        self
    }
}
