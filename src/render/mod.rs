use crate::prelude::*;
use bevy::prelude::*;
use bevy::render::Extract;
use bevy::render::RenderApp;
use bevy::sprite::ExtractedSprite;
use bevy::sprite::ExtractedSprites;
use bevy::sprite::SpriteSystem;

const DEFAULT_BAR_Z_DEPTH: f32 = 950.0;

#[allow(clippy::type_complexity)]
fn extract_status_bars(
    mut commands: Commands,
    mut extracted_sprites: ResMut<ExtractedSprites>,
    subject_query: Extract<Query<&GlobalTransform>>,
    status_bar_query: Extract<
        Query<(
            &StatBarColor,
            Option<&StatBarEmptyColor>,
            Option<&StatBarBorder>,
            &StatBarValue,
            &StatBarSize,
            &StatBarSubject,
            Option<&StatBarPosition>,
            Option<&StatBarZDepth>,
            Option<&StatBarAlignment>,
            Option<&StatBarOrientation>,
            Option<&Visibility>,
        )>,
    >,
) {
    for (
        &StatBarColor(color),
        empty_color_option,
        border_option,
        &StatBarValue(value),
        size,
        &StatBarSubject(subject),
        position_option,
        z_option,
        _alignment_option,
        _orientation_option,
        visiblity,
    ) in status_bar_query.iter()
    {
        if let Some(Visibility::Hidden) = visiblity {
            continue;
        }
        let position = position_option
            .map(|&StatBarPosition(p)| p)
            .unwrap_or(Vec2::ZERO);
        let z_depth = z_option
            .map(|&StatBarZDepth(z)| z)
            .unwrap_or(DEFAULT_BAR_Z_DEPTH);
        if let Ok(translation) = subject_query.get(subject).map(|subject_transform| {
            (subject_transform.translation().truncate() + position).extend(z_depth)
        }) {
            let inner_size = size.full_length * Vec2::X + size.thickness * Vec2::Y;

            if let Some(border) = border_option {
                let border_size = inner_size + border.thickness * Vec2::ONE;

                let new_entity = commands.spawn_empty().id();
                extracted_sprites.sprites.push(ExtractedSprite {
                    main_entity: subject,
                    render_entity: new_entity,
                    transform: GlobalTransform::from_translation(
                        translation.with_z(translation.z - 2.),
                    ),
                    color: border.color.into(),
                    image_handle_id: AssetId::default(),
                    flip_x: false,
                    flip_y: false,
                    kind: bevy::sprite::ExtractedSpriteKind::Single {
                        anchor: default(),
                        rect: default(),
                        scaling_mode: default(),
                        custom_size: Some(border_size),
                    },
                });
            }

            if let Some(empty_color) = empty_color_option {
                let new_entity = commands.spawn_empty().id();
                extracted_sprites.sprites.push(ExtractedSprite {
                    main_entity: subject,
                    render_entity: new_entity,
                    transform: GlobalTransform::from_translation(
                        translation.with_z(translation.z - 1.),
                    ),
                    color: empty_color.0.into(),
                    image_handle_id: AssetId::default(),
                    flip_x: false,
                    flip_y: false,
                    kind: bevy::sprite::ExtractedSpriteKind::Single {
                        anchor: default(),
                        rect: default(),
                        scaling_mode: default(),
                        custom_size: Some(inner_size),
                    },
                });
            }

            if 0.0 < value {
                let clamped_value = value.clamp(0.0, 1.0);
                let bar_size = clamped_value * inner_size.x * Vec2::X + inner_size.y * Vec2::Y;
                let bar_translation =
                    0.5 * size.full_length * (value - 1.0) * Vec3::X + translation;

                let new_entity = commands.spawn_empty().id();
                extracted_sprites.sprites.push(ExtractedSprite {
                    main_entity: subject,
                    render_entity: new_entity,
                    transform: GlobalTransform::from_translation(bar_translation),
                    color: color.into(),
                    image_handle_id: AssetId::default(),
                    flip_x: false,
                    flip_y: false,
                    kind: bevy::sprite::ExtractedSpriteKind::Single {
                        anchor: default(),
                        rect: default(),
                        scaling_mode: default(),
                        custom_size: Some(bar_size),
                    },
                });
            }
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum StatBarSystem {
    ExtractStatBars,
}

pub struct RenderStatBarsPlugin;

impl Plugin for RenderStatBarsPlugin {
    fn build(&self, app: &mut App) {
        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.add_systems(
                ExtractSchedule,
                extract_status_bars
                    .in_set(StatBarSystem::ExtractStatBars)
                    .after(SpriteSystem::ExtractSprites),
            );
        }
    }
}
