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
        .add_bind_source::<Counter>()
        .add_startup_system(setup)
        .run();
}

pub struct Counter {
    pub val: i32,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let mut b = UiBuilder::new(&mut commands, ());
    b.set_default_text_style(TextStyle {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        font_size: 24.,
        color: Color::BLACK,
    });

    b.node()
        .with_unique_name("ui-root")
        .with_style_modifier((StyleSize::FULL, StyleCenterChildren))
        .with_source(Counter { val: 0 })
        .with_children(|b| {
            b.text("?").with_unique_name("counter-label");
            b.button()
                .with_unique_name("add-counter-button")
                .with_style_modifier((
                    StyleSize::px(30., 30.),
                    StyleMargin::all_px(5.),
                    StyleCenterChildren,
                ))
                .with_children(|b| {
                    b.text("+");
                });
            b.button()
                .with_unique_name("sub-counter-button")
                .with_style_modifier((
                    StyleSize::px(30., 30.),
                    StyleMargin::all_px(5.),
                    StyleCenterChildren,
                ))
                .with_children(|b| {
                    b.text("-");
                });
            b.text("Counter value is ?")
                .with_unique_name("counter-msg-label");
        })
        .with_on_source_update(|commands, counter: &Counter| {
            // commands
            //     .entity(b.unique_name_entity("counter-label"))
            //     .insert(Text::from_section("1", TextStyle::default()));
        });
}
