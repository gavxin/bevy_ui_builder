use bevy::prelude::*;

pub trait StyleModifier {
    fn modify(self, style: &mut Style);
}

impl StyleModifier for Style {
    fn modify(self, style: &mut Style) {
        *style = self;
    }
}

impl StyleModifier for Display {
    fn modify(self, style: &mut Style) {
        style.display = self;
    }
}

impl StyleModifier for PositionType {
    fn modify(self, style: &mut Style) {
        style.position_type = self;
    }
}

impl StyleModifier for Direction {
    fn modify(self, style: &mut Style) {
        style.direction = self;
    }
}

impl StyleModifier for FlexDirection {
    fn modify(self, style: &mut Style) {
        style.flex_direction = self;
    }
}

impl StyleModifier for FlexWrap {
    fn modify(self, style: &mut Style) {
        style.flex_wrap = self;
    }
}

impl StyleModifier for AlignItems {
    fn modify(self, style: &mut Style) {
        style.align_items = self;
    }
}

impl StyleModifier for AlignSelf {
    fn modify(self, style: &mut Style) {
        style.align_self = self;
    }
}

impl StyleModifier for AlignContent {
    fn modify(self, style: &mut Style) {
        style.align_content = self;
    }
}

impl StyleModifier for JustifyContent {
    fn modify(self, style: &mut Style) {
        style.justify_content = self;
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct StylePosition(pub UiRect);

impl StyleModifier for StylePosition {
    fn modify(self, style: &mut Style) {
        style.position = self.0;
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct StyleMargin(pub UiRect);

impl StyleModifier for StyleMargin {
    fn modify(self, style: &mut Style) {
        style.margin = self.0;
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct StylePadding(pub UiRect);

impl StyleModifier for StylePadding {
    fn modify(self, style: &mut Style) {
        style.padding = self.0;
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct StyleBorder(pub UiRect);

impl StyleModifier for StyleBorder {
    fn modify(self, style: &mut Style) {
        style.border = self.0;
    }
}

macro_rules! impl_for_ui_rect {
    ($class: ident) => {
        impl $class {
            pub fn new(rect: UiRect) -> Self {
                Self(rect)
            }

            pub fn all(val: Val) -> Self {
                Self(UiRect::all(val))
            }

            pub fn all_px(val: f32) -> Self {
                Self(UiRect::all(Val::Px(val)))
            }
        }

        impl From<UiRect> for $class {
            fn from(rect: UiRect) -> Self {
                Self(rect)
            }
        }
    };
}

impl_for_ui_rect!(StylePosition);
impl_for_ui_rect!(StyleMargin);
impl_for_ui_rect!(StylePadding);
impl_for_ui_rect!(StyleBorder);

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct StyleFlexGrow(pub f32);

impl StyleModifier for StyleFlexGrow {
    fn modify(self, style: &mut Style) {
        style.flex_grow = self.0;
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct StyleFlexShrink(pub f32);

impl StyleModifier for StyleFlexShrink {
    fn modify(self, style: &mut Style) {
        style.flex_shrink = self.0;
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct StyleFlexBasis(pub Val);

impl StyleModifier for StyleFlexBasis {
    fn modify(self, style: &mut Style) {
        style.flex_basis = self.0;
    }
}

impl StyleModifier for Size {
    fn modify(self, style: &mut Style) {
        style.size = self;
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct StyleSize(pub Size);

impl StyleModifier for StyleSize {
    fn modify(self, style: &mut Style) {
        style.size = self.0;
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct StyleMinSize(pub Size);

impl StyleModifier for StyleMinSize {
    fn modify(self, style: &mut Style) {
        style.min_size = self.0;
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct StyleMaxSize(pub Size);

impl StyleModifier for StyleMaxSize {
    fn modify(self, style: &mut Style) {
        style.max_size = self.0;
    }
}

macro_rules! impl_style_size {
    ($class: ident) => {
        impl $class {
            pub fn new(width: Val, height: Val) -> Self {
                Self(Size::new(width, height))
            }

            pub fn px(width: f32, height: f32) -> Self {
                Self(Size::new(Val::Px(width), Val::Px(height)))
            }

            pub fn percent(with: f32, height: f32) -> Self {
                Self(Size::new(Val::Percent(with), Val::Percent(height)))
            }

            pub const AUTO: Self = Self(Size {
                width: Val::Auto,
                height: Val::Auto,
            });

            pub const UNDEFINED: Self = Self(Size {
                width: Val::Undefined,
                height: Val::Undefined,
            });

            pub const FULL: Self = Self(Size {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
            });
        }

        impl From<Size> for $class {
            fn from(size: Size) -> Self {
                Self(size)
            }
        }
    };
}

impl_style_size!(StyleSize);
impl_style_size!(StyleMinSize);
impl_style_size!(StyleMaxSize);

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct StyleAspectRatio(pub Option<f32>);

impl StyleModifier for StyleAspectRatio {
    fn modify(self, style: &mut Style) {
        style.aspect_ratio = self.0;
    }
}

impl StyleModifier for Overflow {
    fn modify(self, style: &mut Style) {
        style.overflow = self;
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct StyleCenterChildren;

impl StyleModifier for StyleCenterChildren {
    fn modify(self, style: &mut Style) {
        style.align_items = AlignItems::Center;
        style.justify_content = JustifyContent::Center;
    }
}

pub trait TextModifier {
    fn modify(self, text: &mut Text);
}

impl TextModifier for Text {
    fn modify(self, text: &mut Text) {
        *text = self;
    }
}

impl TextModifier for Vec<TextSection> {
    fn modify(self, text: &mut Text) {
        text.sections = self;
    }
}

#[derive(Clone, Debug, Default)]
pub struct TextPushSection(pub TextSection);

impl TextModifier for TextPushSection {
    fn modify(self, text: &mut Text) {
        text.sections.push(self.0);
    }
}

impl TextModifier for TextAlignment {
    fn modify(self, text: &mut Text) {
        text.alignment = self;
    }
}

impl TextModifier for TextSection {
    fn modify(self, text: &mut Text) {
        assert!(
            text.sections.len() > 0,
            "Text must have at least one section to modify"
        );
        text.sections[0] = self;
    }
}

impl TextModifier for String {
    fn modify(self, text: &mut Text) {
        assert!(
            text.sections.len() > 0,
            "Text must have at least one section to modify"
        );
        text.sections[0].value = self;
    }
}

impl TextModifier for TextStyle {
    fn modify(self, text: &mut Text) {
        assert!(
            text.sections.len() > 0,
            "Text must have at least one section to modify"
        );
        text.sections[0].style = self;
    }
}

impl TextModifier for Handle<Font> {
    fn modify(self, text: &mut Text) {
        assert!(
            text.sections.len() > 0,
            "Text must have at least one section to modify"
        );
        text.sections[0].style.font = self;
    }
}

impl TextModifier for f32 {
    fn modify(self, text: &mut Text) {
        assert!(
            text.sections.len() > 0,
            "Text must have at least one section to modify"
        );
        text.sections[0].style.font_size = self;
    }
}

impl TextModifier for Color {
    fn modify(self, text: &mut Text) {
        assert!(
            text.sections.len() > 0,
            "Text must have at least one section to modify"
        );
        text.sections[0].style.color = self;
    }
}

impl TextModifier for VerticalAlign {
    fn modify(self, text: &mut Text) {
        text.alignment.vertical = self;
    }
}

impl TextModifier for HorizontalAlign {
    fn modify(self, text: &mut Text) {
        text.alignment.horizontal = self;
    }
}

macro_rules! impl_for_tuple {
    ($trait: ident, $component: ident) => {
        impl<T1> $trait for (T1,)
        where
            T1: $trait,
        {
            fn modify(self, c: &mut $component) {
                self.0.modify(c);
            }
        }

        impl<T1, T2> $trait for (T1, T2)
        where
            T1: $trait,
            T2: $trait,
        {
            fn modify(self, c: &mut $component) {
                self.0.modify(c);
                self.1.modify(c);
            }
        }

        impl<T1, T2, T3> $trait for (T1, T2, T3)
        where
            T1: $trait,
            T2: $trait,
            T3: $trait,
        {
            fn modify(self, c: &mut $component) {
                self.0.modify(c);
                self.1.modify(c);
                self.2.modify(c);
            }
        }

        impl<T1, T2, T3, T4> $trait for (T1, T2, T3, T4)
        where
            T1: $trait,
            T2: $trait,
            T3: $trait,
            T4: $trait,
        {
            fn modify(self, c: &mut $component) {
                self.0.modify(c);
                self.1.modify(c);
                self.2.modify(c);
                self.3.modify(c);
            }
        }

        impl<T1, T2, T3, T4, T5> $trait for (T1, T2, T3, T4, T5)
        where
            T1: $trait,
            T2: $trait,
            T3: $trait,
            T4: $trait,
            T5: $trait,
        {
            fn modify(self, c: &mut $component) {
                self.0.modify(c);
                self.1.modify(c);
                self.2.modify(c);
                self.3.modify(c);
                self.4.modify(c);
            }
        }

        impl<T1, T2, T3, T4, T5, T6> $trait for (T1, T2, T3, T4, T5, T6)
        where
            T1: $trait,
            T2: $trait,
            T3: $trait,
            T4: $trait,
            T5: $trait,
            T6: $trait,
        {
            fn modify(self, c: &mut $component) {
                self.0.modify(c);
                self.1.modify(c);
                self.2.modify(c);
                self.3.modify(c);
                self.4.modify(c);
                self.5.modify(c);
            }
        }

        impl<T1, T2, T3, T4, T5, T6, T7> $trait for (T1, T2, T3, T4, T5, T6, T7)
        where
            T1: $trait,
            T2: $trait,
            T3: $trait,
            T4: $trait,
            T5: $trait,
            T6: $trait,
            T7: $trait,
        {
            fn modify(self, c: &mut $component) {
                self.0.modify(c);
                self.1.modify(c);
                self.2.modify(c);
                self.3.modify(c);
                self.4.modify(c);
                self.5.modify(c);
                self.6.modify(c);
            }
        }

        impl<T1, T2, T3, T4, T5, T6, T7, T8> $trait for (T1, T2, T3, T4, T5, T6, T7, T8)
        where
            T1: $trait,
            T2: $trait,
            T3: $trait,
            T4: $trait,
            T5: $trait,
            T6: $trait,
            T7: $trait,
            T8: $trait,
        {
            fn modify(self, c: &mut $component) {
                self.0.modify(c);
                self.1.modify(c);
                self.2.modify(c);
                self.3.modify(c);
                self.4.modify(c);
                self.5.modify(c);
                self.6.modify(c);
                self.7.modify(c);
            }
        }
    };
}

impl_for_tuple!(StyleModifier, Style);
impl_for_tuple!(TextModifier, Text);
