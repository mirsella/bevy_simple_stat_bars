use crate::prelude::*;
use bevy::prelude::*;

/// basic components
/// required by all stat bars
#[derive(Bundle)]
pub struct StatBarRequiredBundle {
    pub color: StatBarColor,
    pub value: StatBarValue,
    pub size: StatBarSize,
    pub subject: StatBarSubject,
}

#[derive(Bundle)]
pub struct StatBarBundle {
    pub color: StatBarColor,
    pub empty_color: StatBarEmptyColor,
    pub border: StatBarBorder,
    pub value: StatBarValue,
    pub size: StatBarSize,
    pub subject: StatBarSubject,
    pub position: StatBarPosition,
}
