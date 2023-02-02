use bevy::{
    ecs::query::WorldQuery,
    prelude::*,
    utils::{HashMap, HashSet},
};
use smallvec::SmallVec;

/// button with two state: toggled, not toggled
/// optional component, cannot use with NormalButton
/// entity without both NormalButton and ToggleButton will be treated as NormalButton
#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct ToggleButton {
    pub toggled: bool,
}

/// set action on press up (must be hovered, or will cancel action)
/// default is action on press down
#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct ActionOnRelease;

/// if you need additional mouse button support,
/// used when allow right mouse click, or disable left mouse click
/// optional component
#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct MouseButtonMask(pub SmallVec<[MouseButton; 2]>);

impl MouseButtonMask {
    pub fn new(value: &[MouseButton]) -> Self {
        Self(SmallVec::from_slice(value))
    }
}

/// toggle buttons with same group will be exclusive toggled
/// optional component, must use with ToggleButton
#[derive(Component, Debug, Clone, Default, Reflect, PartialEq, Eq)]
#[reflect(Component, Default, PartialEq)]
pub struct ToggleButtonGroup(pub String);

/// when disable button, will not trigger click event
/// required component
#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct Disabled(pub bool);

/// internal state of button visual state
/// required component
#[derive(Component, Debug, Default, Clone, Copy, Reflect, PartialEq, Eq)]
#[reflect(Component, Default)]
pub enum ButtonVisualState {
    #[default]
    Normal,
    Disabled,
    /// or toggled
    Pressed,
    NormalHovered,
    PressedHovered,
}

/// image/texture button, this component value will change UiImage component
/// with corresponding ButtonVisualState value
/// optional component
#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct ImageButton {
    pub normal: Handle<Image>,
    pub disabled: Handle<Image>,
    pub pressed: Handle<Image>,
    pub normal_hovered: Handle<Image>,
    pub pressed_hovered: Handle<Image>,
}

impl ImageButton {
    pub fn normal(normal: Handle<Image>) -> Self {
        ImageButton {
            normal: normal.clone(),
            disabled: normal.clone(),
            pressed: normal.clone(),
            normal_hovered: normal.clone(),
            pressed_hovered: normal.clone(),
        }
    }

    pub fn normal_hovered(normal: Handle<Image>, hovered: Handle<Image>) -> Self {
        ImageButton {
            normal: normal.clone(),
            disabled: normal.clone(),
            pressed: normal.clone(),
            normal_hovered: hovered.clone(),
            pressed_hovered: normal.clone(),
        }
    }

    pub fn normal_pressed(normal: Handle<Image>, pressed: Handle<Image>) -> Self {
        ImageButton {
            normal: normal.clone(),
            disabled: normal.clone(),
            pressed: pressed.clone(),
            normal_hovered: normal.clone(),
            pressed_hovered: pressed.clone(),
        }
    }

    pub fn normal_disabled_pressed(
        normal: Handle<Image>,
        disabled: Handle<Image>,
        pressed: Handle<Image>,
    ) -> Self {
        ImageButton {
            normal: normal.clone(),
            disabled,
            pressed: pressed.clone(),
            normal_hovered: normal.clone(),
            pressed_hovered: pressed.clone(),
        }
    }
}

#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct ColorButton {
    pub normal: Color,
    pub disabled: Color,
    pub pressed: Color,
    pub hovered: Color,
    pub pressed_hovered: Color,
}

impl ColorButton {
    pub fn new(normal: Color) -> Self {
        Self {
            normal,
            disabled: normal,
            pressed: normal,
            hovered: normal,
            pressed_hovered: normal,
        }
    }
}

/// save internal states
/// required component
#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct ButtonInternalState {
    pub pressing: Option<MouseButton>,
    pub hovering: bool,
}

/// event
#[derive(Clone)]
pub struct ProgrammaticClick(pub Entity);

/// for click handler info
#[derive(Clone, Debug)]
pub struct ButtonClickInfo {
    pub entity: Entity,
    pub name: Option<String>,
    pub mouse_button: Option<MouseButton>,
    pub toggle_state: Option<bool>,
}

/// store click handler
/// optional component
#[derive(Component)]
pub struct OnButtonClick(pub Box<dyn Fn(&mut Commands, &ButtonClickInfo) + 'static + Send + Sync>);

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct ButtonQuery {
    entity: Entity,
    name: Option<&'static Name>,
    toggle: Option<&'static mut ToggleButton>,
    action_on_release: Option<&'static ActionOnRelease>,
    mouse_button_mask: Option<&'static MouseButtonMask>,
    toggle_group: Option<&'static ToggleButtonGroup>,
    disabled: &'static Disabled,
    visual_state: &'static mut ButtonVisualState,
    on_click: Option<&'static OnButtonClick>,
    internal_state: &'static mut ButtonInternalState,
}

pub fn button_system(
    mut set: ParamSet<(
        Query<(ButtonQuery, &Interaction), (Changed<Interaction>, With<Button>)>,
        Query<ButtonQuery>,
        Query<ButtonQuery, (Changed<ButtonInternalState>,)>,
        Query<(&ButtonVisualState, &ImageButton, &mut UiImage), (Changed<ButtonVisualState>,)>,
        Query<
            (&ButtonVisualState, &ColorButton, &mut BackgroundColor),
            (Changed<ButtonVisualState>,),
        >,
        Query<
            (
                Entity,
                Option<&Name>,
                &mut ToggleButton,
                &ToggleButtonGroup,
                Option<&OnButtonClick>,
            ),
            (With<ToggleButtonGroup>, With<ToggleButton>),
        >,
        Query<ButtonQuery, Changed<ToggleButton>>,
        Query<ButtonQuery, Changed<Disabled>>,
    )>,
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    mut hovered: Local<HashSet<Entity>>,
    mut pressed_mouse_entity: Local<HashMap<MouseButton, Entity>>,
) {
    let mut pressed_or_released =
        SmallVec::<[(Entity, MouseButton, bool /* is press */); 2]>::new();
    let mut visual_changed = SmallVec::<[Entity; 2]>::new();

    for (mut q, interaction) in set.p0().iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                if q.internal_state.pressing.is_none() {
                    q.internal_state.pressing = Some(MouseButton::Left);
                    pressed_mouse_entity.insert(MouseButton::Left, q.entity);
                    pressed_or_released.push((q.entity, MouseButton::Left, true));
                    visual_changed.push(q.entity);
                }
            }
            Interaction::Hovered => {
                q.internal_state.hovering = true;
                hovered.insert(q.entity);
                visual_changed.push(q.entity);
            }
            Interaction::None => {
                q.internal_state.hovering = false;
                // TODO: handle entity remove before mouse leave
                hovered.remove(&q.entity);
                visual_changed.push(q.entity);
            }
        }
    }

    for mouse_button in mouse_input.get_just_pressed() {
        if *mouse_button == MouseButton::Left {
            continue;
        }

        for hovered_entity in hovered.iter() {
            if let Ok(mut q) = set.p1().get_mut(*hovered_entity) {
                if q.internal_state.pressing.is_none() {
                    q.internal_state.pressing = Some(*mouse_button);
                    pressed_mouse_entity.insert(*mouse_button, q.entity);
                    pressed_or_released.push((q.entity, *mouse_button, true));
                }
            }
        }
    }

    for mouse_button in mouse_input.get_just_released() {
        if let Some(pressed_entity) = pressed_mouse_entity.remove(mouse_button) {
            if let Ok(mut q) = set.p1().get_mut(pressed_entity) {
                if q.internal_state.pressing == Some(*mouse_button) {
                    q.internal_state.pressing = None;
                    pressed_or_released.push((q.entity, *mouse_button, false));
                }
            }
        }
    }

    // handle click, call click handler
    let mut toggle_group_changed = SmallVec::<[(String, Entity); 1]>::new();
    for (entity, mouse_button, is_press) in pressed_or_released.iter() {
        if let Ok(q) = set.p1().get_mut(*entity) {
            if q.disabled.0 {
                continue;
            }

            if let Some(mouse_button_mask) = q.mouse_button_mask {
                // with mask specified, only mouse button in mask work
                if !mouse_button_mask.0.contains(mouse_button) {
                    continue;
                }
            } else if *mouse_button != MouseButton::Left {
                // with no mask specified, only left mouse click work
                continue;
            }

            if *is_press && q.action_on_release.is_some() {
                // action on release
                continue;
            }

            if !*is_press && q.action_on_release.is_none() {
                // action on press down
                continue;
            }

            if q.action_on_release.is_some() && !q.internal_state.hovering {
                // action on release need release inside
                continue;
            }

            if let Some(ref toggle) = q.toggle {
                if q.toggle_group.is_some() && toggle.toggled {
                    // already toggled button in toggle group, do nothing
                    continue;
                }
            }

            let mut click_info = ButtonClickInfo {
                entity: q.entity,
                name: q.name.map(|n| String::from(n.as_str())),
                mouse_button: Some(*mouse_button),
                toggle_state: None,
            };

            if let Some(mut toggle) = q.toggle {
                // toggle button
                if let Some(ToggleButtonGroup(group_name)) = q.toggle_group {
                    toggle_group_changed.push((group_name.clone(), q.entity));
                }

                toggle.toggled = !toggle.toggled;
                visual_changed.push(q.entity);
                click_info.toggle_state = Some(toggle.toggled);
            } else {
                // normal button
                visual_changed.push(q.entity);
            }

            if let Some(handler) = q.on_click {
                trace!("call click handler, click info:{:?}", click_info);
                handler.0(&mut commands, &click_info);
            }
        }
    }

    // handle directly change value of ToggleButton
    for q in set.p6().iter_mut() {
        if visual_changed.iter().any(|e| *e == q.entity) {
            continue;
        }
        if q.toggle.unwrap().toggled {
            if let Some(ToggleButtonGroup(group_name)) = q.toggle_group {
                toggle_group_changed.push((group_name.clone(), q.entity));
            }
        }
        visual_changed.push(q.entity);
    }

    // handle toggle group
    for (entity, name, mut toggle, toggle_group, on_click) in set.p5().iter_mut() {
        for (group, trigger_entity) in toggle_group_changed.iter() {
            if entity == *trigger_entity || *group != toggle_group.0 || !toggle.toggled {
                continue;
            }

            toggle.toggled = false;
            visual_changed.push(entity);
            if let Some(handler) = on_click {
                let click_info = ButtonClickInfo {
                    entity: entity,
                    name: name.map(|n| String::from(n.as_str())),
                    mouse_button: None,
                    toggle_state: Some(false),
                };
                trace!("call click handler, click info:{:?}", click_info);
                handler.0(&mut commands, &click_info);
            }
        }
    }

    // handle Disabled component change
    for q in set.p7().iter_mut() {
        visual_changed.push(q.entity);
    }

    // handle hover changed set visual state
    for entity in visual_changed.iter() {
        if let Ok(mut q) = set.p1().get_mut(*entity) {
            if q.disabled.0 {
                *q.visual_state = ButtonVisualState::Disabled;
                continue;
            }

            if let Some(toggle) = q.toggle {
                // toggle button
                *q.visual_state = if toggle.toggled {
                    if q.internal_state.hovering {
                        ButtonVisualState::PressedHovered
                    } else {
                        ButtonVisualState::Pressed
                    }
                } else {
                    if q.internal_state.hovering {
                        ButtonVisualState::NormalHovered
                    } else {
                        ButtonVisualState::Normal
                    }
                };
            } else {
                // normal button
                let should_pressed = if let Some(pressed_mouse_button) = q.internal_state.pressing {
                    if let Some(mouse_button_mask) = q.mouse_button_mask {
                        mouse_button_mask.0.contains(&pressed_mouse_button)
                    } else {
                        pressed_mouse_button == MouseButton::Left
                    }
                } else {
                    false
                };

                *q.visual_state = if should_pressed {
                    if q.internal_state.hovering {
                        ButtonVisualState::PressedHovered
                    } else {
                        ButtonVisualState::Pressed
                    }
                } else {
                    if q.internal_state.hovering {
                        ButtonVisualState::NormalHovered
                    } else {
                        ButtonVisualState::Normal
                    }
                };
            }
        }
    }

    // change image according to visual state
    for (visual_state, images, mut ui_image) in set.p3().iter_mut() {
        match *visual_state {
            ButtonVisualState::Normal => {
                ui_image.0 = images.normal.clone().into();
            }
            ButtonVisualState::NormalHovered => {
                ui_image.0 = images.normal_hovered.clone().into();
            }
            ButtonVisualState::Pressed => {
                ui_image.0 = images.pressed.clone().into();
            }
            ButtonVisualState::PressedHovered => {
                ui_image.0 = images.pressed_hovered.clone().into();
            }
            ButtonVisualState::Disabled => {
                ui_image.0 = images.disabled.clone().into();
            }
        }
    }

    // change color according to visual state
    for (visual_state, colors, mut bg) in set.p4().iter_mut() {
        match *visual_state {
            ButtonVisualState::Normal => {
                bg.0 = colors.normal;
            }
            ButtonVisualState::NormalHovered => {
                bg.0 = colors.hovered;
            }
            ButtonVisualState::Pressed => {
                bg.0 = colors.pressed;
            }
            ButtonVisualState::PressedHovered => {
                bg.0 = colors.pressed_hovered;
            }
            ButtonVisualState::Disabled => {
                bg.0 = colors.disabled;
            }
        }
    }
}
