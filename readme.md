# bevy ui builder

Simple ui builder, a simple wrapper of bevy_ui, fluent api design.

## Warning

Under heavy developments, cannot guarantee API stability.

## Features

- build bevy_ui basic elements: node, text, image, button
- provide various common button patten, includes
    - toggle button
    - click action on mouse button release inside

## Example

```rust
commands.spawn(Camera2dBundle::default());
let mut b = UiBuilder::new(&mut commands, &());
b.node()
    .with_name("ui-root")
    .with_style_modifier((StyleSize::FULL, StyleCenterChildren))
    .with_children(|b| {
        b.button()
            .with_name("normal-button")
            .with_style_modifier(style)
            .with_color_button(color_button.clone())
            .with_send_event_click(MyClickEvent("normal".into()))
            .with_children(|b| {
                b.text("normal");
            });
    });
```

see other examples in `examples` folder

## Similar Projects

- https://github.com/Anti-Alias/bevy_ui_dsl
- https://github.com/TheRawMeatball/ui4
