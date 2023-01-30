use bevy::{
    ecs::event::Event,
    prelude::*,
    ui::{widget::ImageMode, FocusPolicy},
};

use crate::builder::UiBuilder;

/// when current entity component `S` change, call handler function
///
/// app.register_bind_data_source::<S>() is needed
#[derive(Component)]
pub struct OnSelfChange<S: Component>(pub Box<dyn Fn(&mut Commands, &S) + 'static + Send + Sync>);

pub fn on_self_change_system<S: Component>(
    mut commands: Commands,
    mut query: Query<(&S, &OnSelfChange<S>), Changed<S>>,
) {
    for (s, on_change) in query.iter_mut() {
        (on_change.0)(&mut commands, s);
    }
}

/// when remote entity `source` component `S` change, call handler function
///
/// app.register_bind_data_source::<S>() is needed
#[derive(Component)]
pub struct OnSourceChange<S: Component> {
    pub source: Entity,
    pub handler: Box<dyn Fn(&mut Commands, &S) + 'static + Send + Sync>,
}

pub fn on_source_change_system<S: Component>(
    mut commands: Commands,
    target_query: Query<(Entity, &S), Changed<S>>,
    mut query: Query<&OnSourceChange<S>>,
) {
    for (source_entity, s) in target_query.iter() {
        for on_remote_change in query.iter_mut() {
            if on_remote_change.source == source_entity {
                (on_remote_change.handler)(&mut commands, s);
            }
        }
    }
}

/// when event E happen, call handler function to modify entity `target`
/// component `T`
///
/// app.register_bind_event::<E, T>()
#[derive(Component)]
pub struct EventBindToTarget<E: Event, T: Component> {
    pub target: Entity,
    pub handler: Box<dyn Fn(&mut Commands, &E, Mut<T>) + 'static + Send + Sync>,
}

pub fn event_bind_to_target_system<E: Event, T: Component>(
    mut commands: Commands,
    mut event_reader: EventReader<E>,
    event_bind_query: Query<&EventBindToTarget<E, T>>,
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

/// when current entity component `S` change, call handler function to modify
/// current entity component `T`
///
/// app.register_bind_component::<S, T>() is needed
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

/// when current entity component `S` change, call handler function to modify
/// remote entity `target` component `T`
///
/// app.register_bind_component::<S, T>() is needed
#[derive(Component)]
pub struct BindToTarget<S: Component, T: Component>(pub Vec<BindToTargetItem<S, T>>);

/// when current entity component `S` change, call handler function to modify
/// multiple entities component `T`
///
/// app.register_bind_component::<S, T>() is needed
pub struct BindToTargetItem<S: Component, T: Component> {
    pub target: Entity,
    pub handler: Box<dyn Fn(&mut Commands, &S, Mut<T>) + 'static + Send + Sync>,
}

pub fn bind_to_target_system<S: Component, T: Component>(
    mut commands: Commands,
    mut query: Query<(&S, &BindToTarget<S, T>), Changed<S>>,
    mut target_query: Query<&mut T>,
) {
    for (s, bind_to_target) in query.iter_mut() {
        for item in bind_to_target.0.iter() {
            if let Ok(t) = target_query.get_mut(item.target) {
                (item.handler)(&mut commands, s, t);
            }
        }
    }
}

/// when remote entity `source` component `S` change, call handler function to
/// modify current entity component `T`
///
/// app.register_bind_component::<S, T>() is needed
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
    fn with_on_self_change<S: Component>(
        &mut self,
        handler: impl Fn(&mut Commands, &S) + 'static + Send + Sync,
    ) -> &mut Self;

    fn with_event_bind_to_target<E: Event, T: Component>(
        &mut self,
        target: Entity,
        handler: impl Fn(&mut Commands, &E, Mut<T>) + 'static + Send + Sync,
    ) -> &mut Self;

    fn with_on_source_change<S: Component>(
        &mut self,
        source: Entity,
        handler: impl Fn(&mut Commands, &S) + 'static + Send + Sync,
    ) -> &mut Self;

    fn with_self_bind<S: Component, T: Component>(
        &mut self,
        handler: impl Fn(&mut Commands, &S, Mut<T>) + 'static + Send + Sync,
    ) -> &mut Self;

    fn with_bind_to_target<S: Component, T: Component>(
        &mut self,
        source: Entity,
        handler: impl Fn(&mut Commands, &S, Mut<T>) + 'static + Send + Sync,
    ) -> &mut Self;

    fn with_bind_to_multiple_targets<S: Component, T: Component>(
        &mut self,
        binds: Vec<BindToTargetItem<S, T>>,
    ) -> &mut Self;

    fn with_bind_source<S: Component, T: Component>(
        &mut self,
        source: Entity,
        handler: impl Fn(&mut Commands, &S, Mut<T>) + 'static + Send + Sync,
    ) -> &mut Self;
}

impl<'w, 's, 'a, C> UiBuilderBindExt for UiBuilder<'w, 's, 'a, C> {
    /// when current entity component `S` change, call handler function
    ///
    /// app.register_bind_data_source::<S>() is needed
    fn with_on_self_change<S: Component>(
        &mut self,
        handler: impl Fn(&mut Commands, &S) + 'static + Send + Sync,
    ) -> &mut Self {
        self.commands
            .entity(self.last())
            .insert(OnSelfChange::<S>(Box::new(handler)));
        self
    }

    /// when event E happen, call handler function to modify entity `target`
    /// component `T`
    ///
    /// app.register_bind_event::<E, T>()
    fn with_event_bind_to_target<E: Event, T: Component>(
        &mut self,
        target: Entity,
        handler: impl Fn(&mut Commands, &E, Mut<T>) + 'static + Send + Sync,
    ) -> &mut Self {
        self.commands
            .entity(self.last())
            .insert(EventBindToTarget::<E, T> {
                target,
                handler: Box::new(handler),
            });
        self
    }

    /// when remote entity `source` component `S` change, call handler function
    ///
    /// app.register_bind_data_source::<S>() is needed
    fn with_on_source_change<S: Component>(
        &mut self,
        source: Entity,
        handler: impl Fn(&mut Commands, &S) + 'static + Send + Sync,
    ) -> &mut Self {
        self.commands
            .entity(self.last())
            .insert(OnSourceChange::<S> {
                source,
                handler: Box::new(handler),
            });
        self
    }

    /// when current entity component `S` change, call handler function to modify
    /// current entity component `T`
    ///
    /// app.register_bind_component::<S, T>() is needed
    fn with_self_bind<S: Component, T: Component>(
        &mut self,
        handler: impl Fn(&mut Commands, &S, Mut<T>) + 'static + Send + Sync,
    ) -> &mut Self {
        self.commands
            .entity(self.last())
            .insert(SelfBind::<S, T>(Box::new(handler)));
        self
    }

    /// when current entity component `S` change, call handler function to modify
    /// remote entity `target` component `T`
    ///
    /// app.register_bind_component::<S, T>() is needed
    fn with_bind_to_target<S: Component, T: Component>(
        &mut self,
        target: Entity,
        handler: impl Fn(&mut Commands, &S, Mut<T>) + 'static + Send + Sync,
    ) -> &mut Self {
        self.commands
            .entity(self.last())
            .insert(BindToTarget::<S, T>(vec![BindToTargetItem {
                target,
                handler: Box::new(handler),
            }]));
        self
    }

    /// when current entity component `S` change, call handler function to modify
    /// remote entity `target` component `T`
    ///
    /// app.register_bind_component::<S, T>() is needed
    fn with_bind_to_multiple_targets<S: Component, T: Component>(
        &mut self,
        binds: Vec<BindToTargetItem<S, T>>,
    ) -> &mut Self {
        self.commands
            .entity(self.last())
            .insert(BindToTarget::<S, T>(binds));
        self
    }

    /// when remote entity `source` component `S` change, call handler function to
    /// modify current entity component `T`
    ///
    /// app.register_bind_component::<S, T>() is needed
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
    fn register_bind_component<S: Component, T: Component>(&mut self) -> &mut Self;
    fn register_bind_event<E: Event, T: Component>(&mut self) -> &mut Self;
    fn register_bind_data_source<S: Component>(&mut self, with_ui: bool) -> &mut Self;
}

impl AppBindExt for App {
    /// register bind (S, T), when S change, can effect T
    fn register_bind_component<S: Component, T: Component>(&mut self) -> &mut Self {
        self.add_system(self_bind_system::<S, T>);
        self.add_system(bind_to_target_system::<S, T>);
        self.add_system(bind_source_system::<S, T>);
        self
    }

    /// register bind (E, T), when E event happen, can effect T
    fn register_bind_event<E: Event, T: Component>(&mut self) -> &mut Self {
        self.add_event::<E>();
        self.add_system(event_bind_to_target_system::<E, T>);
        self
    }

    /// register data source component
    fn register_bind_data_source<S: Component>(&mut self, with_ui: bool) -> &mut Self {
        self.add_system(on_self_change_system::<S>);
        self.add_system(on_source_change_system::<S>);

        if with_ui {
            self.register_bind_component::<S, Style>();
            self.register_bind_component::<S, BackgroundColor>();
            self.register_bind_component::<S, FocusPolicy>();
            self.register_bind_component::<S, Visibility>();
            self.register_bind_component::<S, ZIndex>();
            self.register_bind_component::<S, ImageMode>();
            self.register_bind_component::<S, UiImage>();
            self.register_bind_component::<S, Text>();
        }
        self
    }
}
