use bevy::prelude::*;
use bevy_ui_builder::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(UiBuilderPlugin)
        //
        // register data component, this will automatically bind (Counter, Text) pair
        //
        .register_bind_data_source::<Counter>(true)
        .add_startup_system(setup)
        .add_event::<MyClickEvent>()
        .add_system(handle_my_click_event)
        .run();
}

#[derive(Component)]
pub struct Counter {
    pub val: i32,
}

#[derive(Clone)]
pub struct MyClickEvent;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let mut b = UiBuilder::new(&mut commands, ());
    b.set_default_text_style(TextStyle {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        font_size: 24.,
        color: Color::BLACK,
    });

    b.node()
        .with_name("ui-root")
        //
        // add data component
        //
        .with_component(Counter { val: 0 })
        .with_style_modifier((StyleSize::FULL, StyleCenterChildren))
        .with_children(|b| {
            b.text("?").with_name("counter-text");

            b.button()
                .with_unique_name("add-counter-button")
                .with_style_modifier((
                    StyleSize::px(30., 30.),
                    StyleMargin::all_px(5.),
                    StyleCenterChildren,
                ))
                .with_send_event_click(MyClickEvent)
                .with_children(|b| {
                    b.text("+");
                });
        });

    // we need to get the entity of 'counter-text' to bind to it
    let counter_text_entity = b.get_entity_with_unique_name("counter-text");

    b
        //
        // bind remote 'counter-text' entity's Text component
        //
        .with_bind_to_target(
            counter_text_entity,
            |_, counter: &Counter, mut text: Mut<Text>| {
                text.sections[0].value = format!("current counter is {}", counter.val);
            },
        );
}

fn handle_my_click_event(
    mut event_reader: EventReader<MyClickEvent>,
    mut query: Query<&mut Counter>,
) {
    for _ in event_reader.iter() {
        for mut counter in query.iter_mut() {
            counter.val += 1;
        }
    }
}
