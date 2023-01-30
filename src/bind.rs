use bevy::{
    prelude::*,
    ui::{widget::ImageMode, FocusPolicy},
};

use crate::builder::UiBuilder;

pub trait Bindable: 'static + Send + Sync {}
impl<T: 'static + Send + Sync> Bindable for T {}

/// data source, change this component will invoke bounded handler
#[derive(Component)]
pub struct Source<T: Bindable>(pub T);

/// add to same entity which has Source<T>
///
/// must call app.add_bind_source::<T>() to register T as bindable
#[derive(Component)]
pub struct OnSourceUpdate<T: Bindable>(pub Box<dyn Fn(&mut Commands, &T) + 'static + Send + Sync>);

/// bind text content to data source Source<T>, when source changes, invoke handler function
/// and handler function result will be set to Text first section value
#[derive(Component)]
pub struct BindTextContent<T: Bindable> {
    pub source: Entity,
    pub handler: Box<dyn Fn(&T, &String) -> Option<String> + 'static + Send + Sync>,
}

/// works for ui components only
#[derive(Component)]
pub struct BindUiComponent<T: Bindable, C: Component> {
    pub source: Entity,
    pub handler: Box<dyn Fn(&T, &C) -> Option<C> + 'static + Send + Sync>,
}

/// bind child source to parent source
/// require same entity has Source<Child>
///
/// must call app.add_bind_source_relation::<Parent, Child>() before use
#[derive(Component)]
pub struct BindSource<Parent: Bindable, Child: Bindable> {
    pub source: Entity,
    pub handler: Box<dyn Fn(&mut Commands, &Parent, &mut Child) + 'static + Send + Sync>,
}

/// bind C component to data source Source<T>, when source changes, invoke handler function
/// require same entity has component C
///
/// must call app.add_bind_component::<T, C>()
#[derive(Component)]
pub struct BindComponent<T: Bindable, C: Component> {
    pub source: Entity,
    pub handler: Box<dyn Fn(&mut Commands, &T, &mut C) + 'static + Send + Sync>,
}

pub fn bind_system<T: Bindable>(
    mut commands: Commands,
    mut query: Query<(Entity, &Source<T>, Option<&OnSourceUpdate<T>>), Changed<Source<T>>>,
    mut bind_text_content_query: Query<
        (&BindTextContent<T>, &mut Text),
        Without<BindUiComponent<T, Text>>,
    >,
    mut style_query: Query<(Entity, &mut Style, &BindUiComponent<T, Style>)>,
    mut background_color_query: Query<(
        Entity,
        &mut BackgroundColor,
        &BindUiComponent<T, BackgroundColor>,
    )>,
    mut focus_policy_query: Query<(Entity, &mut FocusPolicy, &BindUiComponent<T, FocusPolicy>)>,
    mut visibility_query: Query<(Entity, &mut Visibility, &BindUiComponent<T, Visibility>)>,
    mut z_index_query: Query<(Entity, &mut ZIndex, &BindUiComponent<T, ZIndex>)>,
    mut image_mode_query: Query<(Entity, &mut ImageMode, &BindUiComponent<T, ImageMode>)>,
    mut image_query: Query<(Entity, &mut UiImage, &BindUiComponent<T, UiImage>)>,
    mut text_query: Query<(Entity, &mut Text, &BindUiComponent<T, Text>)>,
) {
    for (source_entity, source, on_update) in query.iter_mut() {
        if let Some(on_update) = on_update {
            (on_update.0)(&mut commands, &source.0);
        }

        for (bind_text_content, mut text) in bind_text_content_query.iter_mut() {
            if bind_text_content.source != source_entity || text.sections.len() == 0 {
                continue;
            }
            let new_value = (bind_text_content.handler)(&source.0, &text.sections[0].value);
            if let Some(val) = new_value {
                if val != text.sections[0].value {
                    text.sections[0].value = val;
                }
            }
        }

        for (_entity, mut style, bind) in style_query.iter_mut() {
            if bind.source == source_entity {
                if let Some(val) = (bind.handler)(&source.0, &style) {
                    *style = val;
                }
            }
        }

        for (_entity, mut background_color, bind) in background_color_query.iter_mut() {
            if bind.source == source_entity {
                if let Some(val) = (bind.handler)(&source.0, &background_color) {
                    *background_color = val;
                }
            }
        }

        for (_entity, mut focus_policy, bind) in focus_policy_query.iter_mut() {
            if bind.source == source_entity {
                if let Some(val) = (bind.handler)(&source.0, &focus_policy) {
                    *focus_policy = val;
                }
            }
        }

        for (_entity, mut visibility, bind) in visibility_query.iter_mut() {
            if bind.source == source_entity {
                if let Some(val) = (bind.handler)(&source.0, &visibility) {
                    *visibility = val;
                }
            }
        }

        for (_entity, mut z_index, bind) in z_index_query.iter_mut() {
            if bind.source == source_entity {
                if let Some(val) = (bind.handler)(&source.0, &z_index) {
                    *z_index = val;
                }
            }
        }

        for (_entity, mut image_mode, bind) in image_mode_query.iter_mut() {
            if bind.source == source_entity {
                if let Some(val) = (bind.handler)(&source.0, &image_mode) {
                    *image_mode = val;
                }
            }
        }

        for (_entity, mut image, bind) in image_query.iter_mut() {
            if bind.source == source_entity {
                if let Some(val) = (bind.handler)(&source.0, &image) {
                    *image = val;
                }
            }
        }

        for (_entity, mut text, bind) in text_query.iter_mut() {
            if bind.source == source_entity {
                if let Some(val) = (bind.handler)(&source.0, &text) {
                    *text = val;
                }
            }
        }
    }
}

pub fn bind_parent_child_system<Parent: Bindable, Child: Bindable>(
    mut commands: Commands,
    mut query: Query<(Entity, &Source<Parent>), Changed<Source<Parent>>>,
    mut child_query: Query<(&mut Source<Child>, &BindSource<Parent, Child>)>,
) {
    for (parent_entity, parent_source) in query.iter_mut() {
        for (mut child_source, bind) in child_query.iter_mut() {
            if bind.source == parent_entity {
                (bind.handler)(&mut commands, &parent_source.0, &mut child_source.0);
            }
        }
    }
}

pub fn bind_generic_component<T: Bindable, C: Component>(
    mut commands: Commands,
    mut query: Query<(Entity, &Source<T>), Changed<Source<T>>>,
    mut component_query: Query<(&mut C, &BindComponent<T, C>)>,
) {
    for (source_entity, source) in query.iter_mut() {
        for (mut component, bind) in component_query.iter_mut() {
            if bind.source == source_entity {
                (bind.handler)(&mut commands, &source.0, &mut component);
            }
        }
    }
}

pub trait UiBuilderBindExt {
    fn with_source<T: Bindable>(&mut self, data: T) -> &mut Self;
    fn with_on_source_update<T: Bindable>(
        &mut self,
        handler: impl Fn(&mut Commands, &T) + 'static + Send + Sync,
    ) -> &mut Self;
}

impl<'w, 's, 'a, C> UiBuilderBindExt for UiBuilder<'w, 's, 'a, C> {
    fn with_source<T: Bindable>(&mut self, data: T) -> &mut Self {
        self.commands.entity(self.last()).insert(Source::<T>(data));
        self
    }

    fn with_on_source_update<T: Bindable>(
        &mut self,
        handler: impl Fn(&mut Commands, &T) + 'static + Send + Sync,
    ) -> &mut Self {
        self.commands
            .entity(self.last())
            .insert(OnSourceUpdate::<T>(Box::new(handler)));
        self
    }
}
