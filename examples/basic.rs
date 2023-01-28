use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ui_builder::{builder::UiBuilder, buttons::ColorButton, modifiers::*, UiBuilderPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(UiBuilderPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    let mut b = UiBuilder::new(&mut commands, &());
    b.node()
        .with_name("ui-root")
        .with_bg_color(Color::GOLD)
        .with_style_modifier((StyleSize::FULL, StyleCenterChildren))
        .with_children(|b| {
            let color_button = ColorButton {
                normal: Color::rgb(0.9, 0.5, 0.5),
                disabled: Color::rgb(0.1, 0.5, 0.5),
                pressed: Color::rgb(0.3, 0.5, 0.5),
                hovered: Color::rgb(0.5, 0.6, 0.5),
                pressed_hovered: Color::rgb(0.5, 0.5, 0.8),
            };
            b.button()
                .with_name("button2")
                .with_style_modifier(StyleSize::px(200., 50.))
                .with_color_button(color_button.clone())
                .with_children(|b| {
                    b.text("normal button");
                });
            b.button()
                .with_name("button")
                .with_style_modifier((
                    StyleSize::px(200., 50.),
                    StyleMargin(UiRect::all(Val::Px(10.))),
                ))
                .with_color_button(color_button.clone())
                .with_toggle(false)
                .with_children(|b| {
                    b.text("toggle button");
                });
        });
}
