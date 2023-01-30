use bevy::{log::LogPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ui_builder::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: bevy::log::Level::INFO,
            filter: "wgpu=error,bevy_ui_builder=trace".to_string(),
        }))
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(UiBuilderPlugin)
        .register_data_source::<Counter>(true)
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
        .with_style_modifier((StyleSize::FULL, StyleCenterChildren))
        .with_component(Counter { val: 0 })
        //
        // this example present how to use on_change
        //
        .with_on_change(|_commands, counter: &Counter| {
            //
            // when Counter component change, this callback will be called
            //
            info!("current counter is {}", counter.val);
        })
        .with_children(|b| {
            b.text("please see terminal output");
            b.button()
                .with_name("add-counter-button")
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
