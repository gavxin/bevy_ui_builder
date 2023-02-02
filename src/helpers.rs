use bevy::prelude::*;

pub trait SizeTrait {
    const FULL: Self;
    fn px(width: f32, height: f32) -> Self;
    fn percent(width: f32, height: f32) -> Self;
    fn from_vec2(vec: Vec2) -> Self;
    fn width_px_auto(val: f32) -> Self;
    fn height_px_auto(val: f32) -> Self;
}

impl SizeTrait for Size {
    const FULL: Self = Size {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
    };
    fn px(width: f32, height: f32) -> Self {
        Size::new(Val::Px(width), Val::Px(height))
    }
    fn percent(width: f32, height: f32) -> Self {
        Size::new(Val::Percent(width), Val::Percent(height))
    }
    fn from_vec2(vec: Vec2) -> Self {
        Size::new(Val::Px(vec.x), Val::Px(vec.y))
    }
    fn width_px_auto(val: f32) -> Self {
        Size::new(Val::Px(val), Val::Auto)
    }
    fn height_px_auto(val: f32) -> Self {
        Size::new(Val::Auto, Val::Px(val))
    }
}
