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
        .add_event::<MyClickEvent>()
        .add_startup_system(setup)
        .add_system(handle_my_click_event)
        .run();
}

#[derive(Clone)]
struct MyClickEvent(pub String);

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
        .with_children(|b| {
            b.node()
                .with_style_modifier((FlexDirection::Row, FlexWrap::Wrap, StyleCenterChildren))
                .with_children(|b| {
                    let container_style = (
                        StyleSize::new(Val::Percent(100.), Val::Px(80.)),
                        StyleCenterChildren,
                    );
                    let button_size = Size::new(Val::Auto, Val::Px(50.));
                    let style = (
                        button_size,
                        StyleCenterChildren,
                        StyleMargin::all_px(10.),
                        StylePadding::all_px(10.),
                    );
                    let color_button = ColorButton {
                        normal: Color::WHITE,
                        disabled: Color::GRAY,
                        pressed: Color::GREEN,
                        hovered: Color::ORANGE,
                        pressed_hovered: Color::SEA_GREEN,
                    };

                    b.node()
                        .with_style_modifier(container_style)
                        .with_name("normal buttons")
                        .with_children(|b| {
                            b.button()
                                .with_name("normal-button")
                                .with_style_modifier(style)
                                .with_color_button(color_button.clone())
                                .with_send_event_click(MyClickEvent("normal".into()))
                                .with_children(|b| {
                                    b.text("normal");
                                });

                            b.button()
                                .with_name("release-action-button")
                                .with_style_modifier(style)
                                .with_color_button(color_button.clone())
                                .with_send_event_click(MyClickEvent(
                                    "action on release inside".into(),
                                ))
                                // click action on mouse release inside
                                .with_action_on_release()
                                .with_children(|b| {
                                    b.text("normal: action_on_release");
                                });

                            b.button()
                                .with_name("right-click-button")
                                .with_style_modifier(style)
                                .with_color_button(color_button.clone())
                                // specify mouse mask
                                .with_mouse_button_mask(&[MouseButton::Right])
                                .with_send_event_click(MyClickEvent("right click".into()))
                                .with_children(|b| {
                                    b.text("normal: right click only");
                                });

                            b.button()
                                .with_name("disabled-button")
                                .with_disabled(true)
                                .with_style_modifier(style)
                                .with_color_button(color_button.clone())
                                .with_children(|b| {
                                    b.text("disabled");
                                });
                        });

                    b.node()
                        .with_name("toggle buttons")
                        .with_style_modifier(container_style)
                        .with_children(|b| {
                            b.button()
                                .with_name("toggle-button")
                                .with_toggle(false)
                                .with_style_modifier(style)
                                .with_color_button(color_button.clone())
                                .with_on_button_click(
                                    |_commands: &mut Commands, info: &ButtonClickInfo| {
                                        info!(
                                            "toggle button clicked, toggle:{}",
                                            info.toggle_state.unwrap()
                                        );
                                    },
                                )
                                .with_children(|b| {
                                    b.text("toggle");
                                });

                            b.button()
                                .with_name("release-action-toggle")
                                .with_toggle(false)
                                .with_style_modifier(style)
                                .with_color_button(color_button.clone())
                                .with_send_event_click(MyClickEvent(
                                    "toggle: action on release inside".into(),
                                ))
                                // click action on mouse release inside
                                .with_action_on_release()
                                .with_children(|b| {
                                    b.text("toggle: action_on_release");
                                });

                            b.button()
                                .with_name("right-click-toggle")
                                .with_toggle(false)
                                .with_style_modifier(style)
                                .with_color_button(color_button.clone())
                                // specify mouse mask
                                .with_mouse_button_mask(&[MouseButton::Right])
                                .with_send_event_click(MyClickEvent("right click toggle".into()))
                                .with_children(|b| {
                                    b.text("toggle: right click only");
                                });
                        });

                    b.node()
                        .with_name("toggle button groups")
                        .with_style_modifier(container_style)
                        .with_children(|b| {
                            b.button()
                                .with_name("toggle-0-group-0")
                                .with_toggle(true)
                                .with_toggle_group("group-0")
                                .with_style_modifier(style)
                                .with_color_button(color_button.clone())
                                .with_send_event_click(MyClickEvent("toggle-0-group-0".into()))
                                .with_children(|b| {
                                    b.text("toggle-0-group-0");
                                });

                            b.button()
                                .with_name("toggle-1-group-0")
                                .with_toggle(false)
                                .with_toggle_group("group-0")
                                .with_style_modifier(style)
                                .with_color_button(color_button.clone())
                                .with_send_event_click(MyClickEvent("toggle-1-group-0".into()))
                                .with_children(|b| {
                                    b.text("toggle-1-group-0");
                                });

                            b.button()
                                .with_name("toggle-2-group-0")
                                .with_toggle(false)
                                .with_toggle_group("group-0")
                                .with_style_modifier(style)
                                .with_color_button(color_button.clone())
                                .with_send_event_click(MyClickEvent("toggle-2-group-0".into()))
                                .with_children(|b| {
                                    b.text("toggle-2-group-0");
                                });

                            b.button()
                                .with_name("toggle-3-group-1")
                                .with_toggle(true)
                                .with_toggle_group("group-1")
                                .with_style_modifier(style)
                                .with_color_button(color_button.clone())
                                .with_send_event_click(MyClickEvent("toggle-3-group-1".into()))
                                .with_children(|b| {
                                    b.text("toggle-3-group-1");
                                });

                            b.button()
                                .with_name("toggle-4-group-1")
                                .with_toggle(false)
                                .with_toggle_group("group-1")
                                .with_style_modifier(style)
                                .with_color_button(color_button.clone())
                                .with_send_event_click(MyClickEvent("toggle-4-group-1".into()))
                                .with_children(|b| {
                                    b.text("toggle-4-group-1");
                                });
                        });
                });
        });
}

fn handle_my_click_event(mut event_reader: EventReader<MyClickEvent>) {
    for ev in event_reader.iter() {
        info!("MyClickEvent, msg:{}", ev.0);
    }
}
