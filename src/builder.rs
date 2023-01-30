use std::borrow::Cow;

use bevy::{ecs::event::Event, prelude::*, ui::widget::ImageMode, utils::HashMap};

use crate::{buttons::*, modifiers::*};

/// UI Builder
pub struct UiBuilder<'w, 's, 'a, C> {
    pub commands: &'a mut Commands<'w, 's>,
    pub context: C,
    pub parent: Option<Entity>,
    pub last: Option<Entity>,
    pub default_text_style: TextStyle,
    pub last_text_content: String,
    pub name_entity_map: HashMap<Cow<'static, str>, Entity>,
}

impl<'w, 's, 'a, C> UiBuilder<'w, 's, 'a, C> {
    /// create ui builder
    pub fn new(commands: &'a mut Commands<'w, 's>, context: C) -> Self {
        Self {
            commands,
            context,
            parent: None,
            last: None,
            default_text_style: TextStyle::default(),
            last_text_content: String::new(),
            name_entity_map: HashMap::default(),
        }
    }

    /// get last created entity
    /// with_* fn will modify last entity
    pub fn last(&self) -> Entity {
        self.last.expect("no last entity")
    }

    /// get last entity as option
    pub fn get_last(&self) -> Option<Entity> {
        self.last
    }

    /// set last created entity
    /// with_* fn will modify last entity
    pub fn set_last(&mut self, e: Entity) -> &mut Self {
        self.last = Some(e);
        self
    }

    /// get parent
    pub fn parent(&self) -> Entity {
        self.parent.expect("no parent entity")
    }

    /// get parent as option
    pub fn get_parent(&self) -> Option<Entity> {
        self.parent
    }

    /// set parent entity
    pub fn set_parent(&mut self, e: Entity) -> &mut Self {
        self.parent = Some(e);
        self
    }

    /// assign last entity value to param
    pub fn pull_last(&mut self, e: &mut Entity) -> &mut Self {
        *e = self.last();
        self
    }

    /// assign last entity value to option param
    pub fn pull_last_as_option(&mut self, e: &mut Option<Entity>) -> &mut Self {
        *e = Some(self.last());
        self
    }

    /// change default font
    pub fn set_default_font(&mut self, font: Handle<Font>) -> &mut Self {
        self.default_text_style.font = font;
        self
    }

    /// change default text style
    pub fn set_default_text_style(&mut self, text_style: TextStyle) -> &mut Self {
        self.default_text_style = text_style;
        self
    }

    fn put_new_node(&mut self, e: Entity) {
        if let Some(p) = self.parent {
            self.commands.entity(p).add_child(e);
        }
        self.last = Some(e);
    }

    /// create new node entity (NodeBundle)
    pub fn node(&mut self) -> &mut Self {
        let e = self.commands.spawn(NodeBundle::default()).id();
        self.put_new_node(e);
        self
    }

    /// create new image entity (ImageBundle)
    pub fn image(&mut self, image: Handle<Image>) -> &mut Self {
        let e = self
            .commands
            .spawn(ImageBundle {
                image: image.into(),
                ..default()
            })
            .id();
        self.put_new_node(e);
        self
    }

    /// change last entity image mode
    pub fn with_image_mode(&mut self, image_mode: ImageMode) -> &mut Self {
        self.commands.entity(self.last()).insert(image_mode);
        self
    }

    /// create new text entity (TextBundle)
    pub fn text(&mut self, text: impl Into<String>) -> &mut Self {
        self.last_text_content = text.into();
        let e = self
            .commands
            .spawn(TextBundle::from_section(
                self.last_text_content.clone(),
                self.default_text_style.clone(),
            ))
            .id();
        self.put_new_node(e);
        self
    }

    /// create base button
    pub fn button(&mut self) -> &mut Self {
        let e = self
            .commands
            .spawn((
                ButtonBundle::default(),
                Disabled(false),
                ButtonVisualState::Normal,
                ButtonInternalState::default(),
            ))
            .id();
        self.put_new_node(e);
        self
    }

    /// button: add click handler
    pub fn with_on_button_click(
        &mut self,
        handler: impl Fn(&mut Commands, &ButtonClickInfo) + 'static + Send + Sync,
    ) -> &mut Self {
        self.commands
            .entity(self.last())
            .insert(OnButtonClick(Box::new(handler)));
        self
    }

    /// button: send event on click
    /// will overwrite other click handler
    pub fn with_send_event_click<E: Event + Clone>(&mut self, e: E) -> &mut Self {
        self.commands
            .entity(self.last())
            .insert(OnButtonClick(Box::new(
                move |commands: &mut Commands, _info: &ButtonClickInfo| {
                    send_event(commands, e.clone());
                },
            )));
        self
    }

    /// change last button mode to toggle mode
    pub fn with_toggle(&mut self, toggle: bool) -> &mut Self {
        self.commands
            .entity(self.last())
            .insert(ToggleButton { toggled: toggle });
        self
    }

    /// toggle mode in specific group, only one button in group can be toggled
    /// only work with toggle mode
    pub fn with_toggle_group(&mut self, group_name: impl Into<String>) -> &mut Self {
        self.commands
            .entity(self.last())
            .insert(ToggleButtonGroup(group_name.into()));
        self
    }

    /// change click action on release
    pub fn with_action_on_release(&mut self) -> &mut Self {
        self.commands.entity(self.last()).insert(ActionOnRelease);
        self
    }

    /// change mouse button mask
    pub fn with_mouse_button_mask(&mut self, mask: &[MouseButton]) -> &mut Self {
        self.commands
            .entity(self.last())
            .insert(MouseButtonMask::new(mask));
        self
    }

    /// change button images when state changed
    pub fn with_image_button(&mut self, value: impl Into<ImageButton>) -> &mut Self {
        self.commands.entity(self.last()).insert(value.into());
        self
    }

    /// change button background color when state changed
    /// use with button()
    pub fn with_color_button(&mut self, value: ColorButton) -> &mut Self {
        self.commands
            .entity(self.last())
            .insert((BackgroundColor(value.normal), value));
        self
    }

    /// create children on last entity
    pub fn with_children(&mut self, build_fn: impl FnOnce(&mut Self)) -> &mut Self {
        let old_parent = self.parent;
        let old_last = self.last;
        self.parent = self.last;
        self.last = None;
        build_fn(self);
        self.parent = old_parent;
        self.last = old_last;
        self
    }

    /// add Name component
    /// 
    /// bevy_inspector_egui will show name as entity name, easier to find entity
    pub fn with_name(&mut self, name: impl Into<Cow<'static, str>>) -> &mut Self {
        self.commands.entity(self.last()).insert(Name::new(name));
        self
    }

    /// add Name component
    /// save entity to name_entity_map for later use get_entity_with_unique_name()
    pub fn with_unique_name(&mut self, name: impl Into<Cow<'static, str>>) -> &mut Self {
        let name = name.into();
        let old = self.name_entity_map.insert(name.clone(), self.last());
        assert!(old.is_none(), "duplicate name: {}", name);
        self.commands.entity(self.last()).insert(Name::new(name));
        self
    }

    /// get entity by unique name
    pub fn get_entity_with_unique_name(&mut self, name: impl Into<Cow<'static, str>>) -> Entity {
        self.name_entity_map
            .get(&name.into())
            .expect("unique name not found")
            .clone()
    }

    /// insert or overwrite last entity component
    pub fn with_component(&mut self, c: impl Component) -> &mut Self {
        self.commands.entity(self.last()).insert(c);
        self
    }

    /// set last entity disabled as true
    pub fn with_disabled(&mut self, value: bool) -> &mut Self {
        self.commands.entity(self.last()).insert(Disabled(value));
        self
    }

    /// modify last entity style (Style component)
    /// call this multiple time will overwrite previous style
    pub fn with_style_modifier(&mut self, style_modifier: impl StyleModifier) -> &mut Self {
        let mut style = Style::default();
        style_modifier.modify(&mut style);
        self.commands.entity(self.last()).insert(style);
        self
    }

    /// modify last entity text (Text component)
    /// call this multiple time will overwrite previous text
    pub fn with_text_modifier(&mut self, text_modifier: impl TextModifier) -> &mut Self {
        let mut text = Text {
            sections: vec![TextSection {
                value: self.last_text_content.clone(),
                style: self.default_text_style.clone(),
            }],
            ..default()
        };
        text_modifier.modify(&mut text);
        self.commands.entity(self.last()).insert(text);
        self
    }

    /// modify last entity visibility (Visibility component)
    /// call this multiple time will overwrite previous state
    pub fn with_visibility(&mut self, visible: bool) -> &mut Self {
        self.commands.entity(self.last()).insert(Visibility {
            is_visible: visible,
        });
        self
    }

    /// modify last entity background color (BackgroundColor component)
    /// call this multiple time will overwrite previous color
    pub fn with_bg_color(&mut self, color: Color) -> &mut Self {
        self.commands
            .entity(self.last())
            .insert(BackgroundColor(color));
        self
    }
}

/// helper function for send abitrary event with Commands
pub fn send_event<E: Event>(commands: &mut Commands, e: E) {
    commands.add(|w: &mut World| {
        let mut events_resource = w.resource_mut::<Events<_>>();
        events_resource.send(e);
    });
}
