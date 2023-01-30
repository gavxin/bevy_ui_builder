# bevy ui builder

Simple ui builder, a simple wrapper of bevy_ui, fluent api design.

## Warning

Under heavy development, cannot guarantee API stability.

## Features

- build bevy_ui basic elements: node, text, image, button
- various buttons, includes
    - toggle button
    - click action on mouse button release inside
- various binding

## Basics

### construct ui hierarchy

before start, you need to know about bevy_ui 4 basic ui elements

- node(NodeBundle)
- image(ImageBundle)
- text(TextBundle)
- button(ButtonBundle)

ui builder has corresponding functions to build entire ui

- node()
- image(handle_to_image)
- button()
- text(content)

```rust
// before start, add plugin first
.add_plugin(UiBuilderPlugin)

// usage
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ...
    let mut b = UiBuilder::new(&mut commands, ());
    // create new node element
    b.node()
        // style modifier
        .with_style_modifier((StyleSize::FULL, StyleCenterChildren))
        .with_children(|b| {
            // create image element
            b.image(asset_server.load("image/file.png"));

            // create button element
            b.button()
                .with_children(|b| {
                    // create text element
                    b.text("text content");
                });
        });
    // ...
}
```

### modify style

- change `Style` component with `.with_style_modifiers(...)`
- change `Text` component with `.with_text_modifier(...)`
- change `Visibility` component with `.with_visibility(...)`
- change `BackgroundColor` component with `.with_bg_color(...)`

> NOTE: duplicated call will overwrite previous state

For more `StyleModifier` and `TextModifier` , please see [src/modifiers.rs](src/modifiers.rs)

### button

There is a button [example](examples/buttons.rs)



#### toggle button

- toggle button: `.with_toggle(false)`
- toggle button with toggle group: `.with_toggle(false).with_toggle_group("group_name")`

#### how click action happen

- `.with_action_on_release()`: action performed on mouse button release inside
- `.with_mouse_button_mask(...)`: react on additional mouse right / middle button, or can disable default left mouse button

#### how button looks

every button has following visual states,

- normal
- disabled
- normal_hovered
- pressed
- pressed_hovered

library provide following two ways

- `.with_image_button()`: change UiImage when visual state change.
- `.with_color_button()`: change BackgroundColor when visual state change.

If you need more customize style, you can use `Changed<>` subscribe `ButtonVisualState` component changes, and work on your styles.

### binding

- S: data source, component
- T: modify target, component
- source entity: has a data source component `S`
- target entity: has target component `T`

| api                                                  | description                                                  |
| ---------------------------------------------------- | ------------------------------------------------------------ |
| `with_bind_source::<S, T>(source, handler)`          | when remote entity `source` component `S` change, call handler function to modify current entity component `T`. see [example](examples/bind_source.rs) |
| `with_on_self_change::<S>(handler)`                  | when current entity component `S` change, call handler function. see [example](examples/on_self_change.rs) |
| `with_event_bind_to_target::<E, T>(target, handler)` | when event E happen, call handler function to modify entity `target` component `T`. see [example](examples/inventory.rs) |
| `with_on_source_change::<S>(source, handler)`        | when remote entity `source` component `S` change, call handler function. see [example](examples/inventory.rs) |
| `with_self_bind::<S, T>(handler)`                    | when current entity component `S` change, call handler function to modify current entity component `T`. see [example](examples/self_bind.rs) |
| `with_bind_to_target::<S, T>(target, handler)`       | when current entity component `S` change, call handler function to modify remote entity `target` component `T`. see [example](examples/bind_to_target.rs) |
| `with_bind_to_multiple_targets::<S, T>(binds)`       | when current entity component `S` change, call handler function to modify remote entity `target` component `T` |

## Similar Projects

This library is inspired from following libraries

- https://github.com/Anti-Alias/bevy_ui_dsl
- https://github.com/TheRawMeatball/ui4

## License

MIT OR Apache-2.0

