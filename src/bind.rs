use bevy::{
    ecs::event::Event,
    prelude::*,
    ui::{widget::ImageMode, FocusPolicy},
};

use crate::builder::UiBuilder;

/// detect changes of the S component and call the callback
/// add this component to same entity which has S component
///
/// app.register_data_source::<S>() is needed
#[derive(Component)]
pub struct OnChange<S: Component>(pub Box<dyn Fn(&mut Commands, &S) + 'static + Send + Sync>);

pub fn on_change_system<S: Component>(
    mut commands: Commands,
    mut query: Query<(&S, &OnChange<S>), Changed<S>>,
) {
    for (s, on_change) in query.iter_mut() {
        (on_change.0)(&mut commands, s);
    }
}

#[derive(Component)]
pub struct EventBind<E: Event, T: Component> {
    pub target: Entity,
    pub handler: Box<dyn Fn(&mut Commands, &E, Mut<T>) + 'static + Send + Sync>,
}

pub fn event_bind_system<E: Event, T: Component>(
    mut commands: Commands,
    mut event_reader: EventReader<E>,
    event_bind_query: Query<&EventBind<E, T>>,
    mut target_query: Query<(Entity, &mut T)>,
) {
    for ev in event_reader.iter() {
        for event_bind in event_bind_query.iter() {
            for (target_entity, t) in target_query.iter_mut() {
                if event_bind.target == target_entity {
                    (event_bind.handler)(&mut commands, ev, t);
                }
            }
        }
    }
}

/// detect changes of the remote entity's S component
///
/// app.register_data_source::<S>() is needed
#[derive(Component)]
pub struct OnRemoteChange<S: Component> {
    pub source: Entity,
    pub handler: Box<dyn Fn(&mut Commands, &S) + 'static + Send + Sync>,
}

pub fn on_remote_change_system<S: Component>(
    mut commands: Commands,
    target_query: Query<(Entity, &S), Changed<S>>,
    mut query: Query<&OnRemoteChange<S>>,
) {
    for (source_entity, s) in target_query.iter() {
        for on_remote_change in query.iter_mut() {
            if on_remote_change.source == source_entity {
                (on_remote_change.handler)(&mut commands, s);
            }
        }
    }
}

/// when S component change, call handler to change T component
///
/// app.register_bind::<S, T>() is needed
#[derive(Component)]
pub struct SelfBind<S: Component, T: Component>(
    pub Box<dyn Fn(&mut Commands, &S, Mut<T>) + 'static + Send + Sync>,
);

pub fn self_bind_system<S: Component, T: Component>(
    mut commands: Commands,
    mut query: Query<(&S, &SelfBind<S, T>, &mut T), Changed<S>>,
) {
    for (s, self_bind, t) in query.iter_mut() {
        (self_bind.0)(&mut commands, s, t);
    }
}

/// detect changes of the S component and change T component
/// add this component to same entity which has S and T component
///
/// app.register_bind::<S, T>() is needed
#[derive(Component)]
pub struct BindRemote<S: Component, T: Component>(pub Vec<BindRemoteItem<S, T>>);

/// detect changes of the S component and change remote entity's T component
/// add this component to same entity which has S component
///
/// app.register_bind::<S, T>() is needed
pub struct BindRemoteItem<S: Component, T: Component> {
    pub source: Entity,
    pub handler: Box<dyn Fn(&mut Commands, &S, Mut<T>) + 'static + Send + Sync>,
}

pub fn component_bind_system<S: Component, T: Component>(
    mut commands: Commands,
    mut query_remote: Query<(&S, &BindRemote<S, T>), Changed<S>>,
    mut target_query: Query<&mut T>,
) {
    for (s, bind_remote) in query_remote.iter_mut() {
        for item in bind_remote.0.iter() {
            if let Ok(t) = target_query.get_mut(item.source) {
                (item.handler)(&mut commands, s, t);
            }
        }
    }
}

/// detect changes of remote source entity's S component and call handler
/// add this component to same entity which has T component
///
/// app.register_bind::<S, T>() is needed
#[derive(Component)]
pub struct BindSource<S: Component, T: Component> {
    pub source_entity: Entity,
    pub handler: Box<dyn Fn(&mut Commands, &S, Mut<T>) + 'static + Send + Sync>,
}

pub fn bind_source_system<S: Component, T: Component>(
    mut commands: Commands,
    mut source_query: Query<(Entity, &S), Changed<S>>,
    mut target_query: Query<(&BindSource<S, T>, &mut T)>,
) {
    for (source_entity, s) in source_query.iter_mut() {
        for (bind_source, t) in target_query.iter_mut() {
            if bind_source.source_entity == source_entity {
                (bind_source.handler)(&mut commands, s, t);
            }
        }
    }
}

pub trait UiBuilderBindExt {
    fn with_on_change<S: Component>(
        &mut self,
        handler: impl Fn(&mut Commands, &S) + 'static + Send + Sync,
    ) -> &mut Self;

    fn with_event_bind<E: Event, T: Component>(
        &mut self,
        source: Entity,
        handler: impl Fn(&mut Commands, &E, Mut<T>) + 'static + Send + Sync,
    ) -> &mut Self;

    fn with_on_remote_change<S: Component>(
        &mut self,
        source: Entity,
        handler: impl Fn(&mut Commands, &S) + 'static + Send + Sync,
    ) -> &mut Self;

    fn with_self_bind<S: Component, T: Component>(
        &mut self,
        handler: impl Fn(&mut Commands, &S, Mut<T>) + 'static + Send + Sync,
    ) -> &mut Self;

    fn with_bind_remote<S: Component, T: Component>(
        &mut self,
        source: Entity,
        handler: impl Fn(&mut Commands, &S, Mut<T>) + 'static + Send + Sync,
    ) -> &mut Self;

    fn with_bind_multiple_remote<S: Component, T: Component>(
        &mut self,
        binds: Vec<BindRemoteItem<S, T>>,
    ) -> &mut Self;

    fn with_bind_source<S: Component, T: Component>(
        &mut self,
        source: Entity,
        handler: impl Fn(&mut Commands, &S, Mut<T>) + 'static + Send + Sync,
    ) -> &mut Self;
}

impl<'w, 's, 'a, C> UiBuilderBindExt for UiBuilder<'w, 's, 'a, C> {
    /// call handler function when last entity's S component change
    /// last entity has S component
    ///
    /// app.register_data_source::<S>() is needed
    fn with_on_change<S: Component>(
        &mut self,
        handler: impl Fn(&mut Commands, &S) + 'static + Send + Sync,
    ) -> &mut Self {
        self.commands
            .entity(self.last())
            .insert(OnChange::<S>(Box::new(handler)));
        self
    }

    /// when E event happen, call handler function
    fn with_event_bind<E: Event, T: Component>(
        &mut self,
        target: Entity,
        handler: impl Fn(&mut Commands, &E, Mut<T>) + 'static + Send + Sync,
    ) -> &mut Self {
        self.commands.entity(self.last()).insert(EventBind::<E, T> {
            target,
            handler: Box::new(handler),
        });
        self
    }

    /// call handler function when remote entity's S component change
    ///
    /// app.register_data_source::<S>() is needed
    fn with_on_remote_change<S: Component>(
        &mut self,
        source: Entity,
        handler: impl Fn(&mut Commands, &S) + 'static + Send + Sync,
    ) -> &mut Self {
        self.commands
            .entity(self.last())
            .insert(OnRemoteChange::<S> {
                source,
                handler: Box::new(handler),
            });
        self
    }

    /// last entity has S and T component, S is data source,
    /// when S change, will call handler to change T component
    ///
    /// app.register_data_source::<S>() is needed
    fn with_self_bind<S: Component, T: Component>(
        &mut self,
        handler: impl Fn(&mut Commands, &S, Mut<T>) + 'static + Send + Sync,
    ) -> &mut Self {
        self.commands
            .entity(self.last())
            .insert(SelfBind::<S, T>(Box::new(handler)));
        self
    }

    /// last entity has S and the other remote entity has T component,
    /// when S change, will call handler to change remote entity's T component
    ///
    /// app.register_data_source::<S>() is needed
    fn with_bind_remote<S: Component, T: Component>(
        &mut self,
        source: Entity,
        handler: impl Fn(&mut Commands, &S, Mut<T>) + 'static + Send + Sync,
    ) -> &mut Self {
        self.commands
            .entity(self.last())
            .insert(BindRemote::<S, T>(vec![BindRemoteItem {
                source,
                handler: Box::new(handler),
            }]));
        self
    }

    /// multiple remote entity version of with_bind_remote
    fn with_bind_multiple_remote<S: Component, T: Component>(
        &mut self,
        binds: Vec<BindRemoteItem<S, T>>,
    ) -> &mut Self {
        self.commands
            .entity(self.last())
            .insert(BindRemote::<S, T>(binds));
        self
    }

    /// last entity has T component, need react changes of data source entity's S component
    ///
    /// app.register_data_source::<S>() is needed
    fn with_bind_source<S: Component, T: Component>(
        &mut self,
        source: Entity,
        handler: impl Fn(&mut Commands, &S, Mut<T>) + 'static + Send + Sync,
    ) -> &mut Self {
        self.commands
            .entity(self.last())
            .insert(BindSource::<S, T> {
                source_entity: source,
                handler: Box::new(handler),
            });
        self
    }
}

pub trait AppBindExt {
    fn register_component_bind<S: Component, T: Component>(&mut self) -> &mut Self;
    fn register_event_bind<E: Event, T: Component>(&mut self) -> &mut Self;
    fn register_data_source<S: Component>(&mut self, with_ui: bool) -> &mut Self;
}

impl AppBindExt for App {
    /// register bind (S, T), when S change, can effect T
    fn register_component_bind<S: Component, T: Component>(&mut self) -> &mut Self {
        self.add_system(self_bind_system::<S, T>);
        self.add_system(component_bind_system::<S, T>);
        self.add_system(bind_source_system::<S, T>);
        self
    }

    /// register bind (E, T), when E event happen, can effect T
    fn register_event_bind<E: Event, T: Component>(&mut self) -> &mut Self {
        self.add_event::<E>();
        self.add_system(event_bind_system::<E, T>);
        self
    }

    /// register data source component only
    fn register_data_source<S: Component>(&mut self, with_ui: bool) -> &mut Self {
        self.add_system(on_change_system::<S>);
        self.add_system(on_remote_change_system::<S>);

        if with_ui {
            self.register_component_bind::<S, Style>();
            self.register_component_bind::<S, BackgroundColor>();
            self.register_component_bind::<S, FocusPolicy>();
            self.register_component_bind::<S, Visibility>();
            self.register_component_bind::<S, ZIndex>();
            self.register_component_bind::<S, ImageMode>();
            self.register_component_bind::<S, UiImage>();
            self.register_component_bind::<S, Text>();
        }
        self
    }
}
