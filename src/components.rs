use bevy::color::palettes::css;

use crate::*;

#[derive(Component)]
pub struct StatBarBorder {
    pub color: Color,
    pub thickness: f32,
}

impl Default for StatBarBorder {
    fn default() -> Self {
        Self {
            color: css::DARK_GRAY.into(),
            thickness: 2.0,
        }
    }
}

/// not used for now
#[derive(Component)]
pub struct StatBarOrientation {
    pub quat: Quat,
}

impl Default for StatBarOrientation {
    fn default() -> Self {
        Self {
            quat: Quat::IDENTITY,
        }
    }
}

#[derive(Component)]
pub struct StatBarValue(pub f32);

impl Default for StatBarValue {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Component)]
pub struct StatBarColor(pub Color);

impl Default for StatBarColor {
    fn default() -> Self {
        Self(Color::WHITE)
    }
}

/// not used for now
#[derive(Component)]
pub struct StatBarAlignment(pub Vec2);

impl Default for StatBarAlignment {
    fn default() -> Self {
        Self(0.5 * Vec2::ONE)
    }
}

#[derive(Clone, Copy, Component)]
pub struct StatBarSize {
    pub full_length: f32,
    pub thickness: f32,
}

impl Default for StatBarSize {
    fn default() -> Self {
        Self {
            full_length: 100.0,
            thickness: 10.0,
        }
    }
}

#[derive(Component)]
pub struct StatBarZDepth(pub f32);

impl Default for StatBarZDepth {
    fn default() -> Self {
        Self(950.0)
    }
}

#[derive(Component)]
pub struct StatBarEmptyColor(pub Color);

impl Default for StatBarEmptyColor {
    fn default() -> Self {
        Self(Color::BLACK)
    }
}

#[derive(Component)]
pub struct StatBarSubject(pub Entity);

#[derive(Component)]
pub struct StatBarPosition(pub Vec2);
