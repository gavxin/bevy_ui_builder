use bevy::prelude::*;
use bevy_ui_builder::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(UiBuilderPlugin)
        .register_data_source::<Inventory>(true)
        .register_data_source::<Item>(true)
        .register_component_bind::<Inventory, Item>()
        .register_event_bind::<MyEvent, Inventory>()
        .add_startup_system(setup)
        .run();
}

#[derive(Component, Clone)]
pub struct Inventory {
    pub items: Vec<Item>,
}

#[derive(Component, Clone)]
pub struct Item(String, u32);

#[derive(Clone)]
pub struct MyEvent;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let mut b = UiBuilder::new(&mut commands, ());
    b.set_default_text_style(TextStyle {
        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
        font_size: 24.,
        color: Color::BLACK,
    });

    let inventory = Inventory {
        items: vec![Item("a".into(), 1), Item("b".into(), 3)],
    };

    let mut inventory_entity = Entity::from_raw(u32::MAX);
    b.node()
        .with_name("inventory")
        .pull_last(&mut inventory_entity)
        .with_component(inventory)
        .with_style_modifier((StyleSize::FULL, StyleCenterChildren, FlexDirection::Column))
        .with_children(|b| {
            b.text("").with_bind_source(
                inventory_entity,
                |_, inventory: &Inventory, mut text: Mut<Text>| {
                    text.sections[0].value = format!(
                        "total num: {}",
                        inventory.items.iter().fold(0, |s, a| s + a.1)
                    );
                },
            );

            let mut items_entity = Entity::from_raw(u32::MAX);
            let text_style = b.default_text_style.clone();
            b.node()
                .with_name("items")
                .pull_last(&mut items_entity)
                .with_style_modifier((StyleCenterChildren, FlexDirection::Column))
                .with_on_remote_change(
                    inventory_entity,
                    move |commands: &mut Commands, inventory: &Inventory| {
                        commands.entity(items_entity).despawn_descendants();

                        let mut b = UiBuilder::new(commands, ());
                        b.set_default_text_style(text_style.clone());
                        b.set_parent(items_entity);

                        for (idx, item) in inventory.items.iter().enumerate() {
                            b.node()
                                .with_name(format!("item{}", idx))
                                .with_component(item.clone())
                                .with_bind_source(
                                    inventory_entity,
                                    move |_, inventory: &Inventory, mut item: Mut<Item>| {
                                        *item = inventory.items[idx].clone();
                                    },
                                )
                                .with_children(|b| {
                                    let parent = b.parent();
                                    b.text("").with_bind_source(
                                        parent,
                                        |_, item: &Item, mut text: Mut<Text>| {
                                            text.sections[0].value =
                                                format!("name:{} amount:{}", item.0, item.1);
                                        },
                                    );
                                });
                        }
                    },
                );

            // controller
            b.button()
                .with_name("btn")
                .with_style_modifier((StyleCenterChildren, StyleMargin::all_px(15.0)))
                .with_send_event_click(MyEvent)
                .with_event_bind(
                    inventory_entity,
                    |_commands, _ev: &MyEvent, mut inventory: Mut<Inventory>| {
                        info!("button clicked");
                        for item in inventory.as_mut().items.iter_mut() {
                            item.1 += 1;
                        }
                        inventory.items.push(Item("c".into(), 1));
                    },
                )
                .with_children(|b| {
                    b.text("all amount++\nadd item c");
                });
        });
}
