use bevy::color::palettes::css;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy_simple_stat_bars::prelude::*;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct PlayerCharacter;

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct Hp {
    current: i32,
    max: i32,
}

#[derive(Component)]
struct Mp {
    current: i32,
    max: i32,
}

fn spawn_player(mut commands: Commands) {
    let player = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(vec2(32.0, 64.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
            Speed(250.0),
            PlayerCharacter,
            Hp {
                current: 30,
                max: 30,
            },
            Mp {
                current: 12,
                max: 15,
            },
        ))
        .id();

    commands.spawn((
        StatBarColor(css::GREEN.into()),
        StatBarEmptyColor(css::BLACK.into()),
        StatBarBorder {
            color: css::DARK_GRAY.into(),
            thickness: 3.0,
        },
        StatBarValue(1.0),
        StatBarSize {
            full_length: 50.0,
            thickness: 6.0,
        },
        StatBarSubject(player),
        StatBarPosition(40.0 * Vec2::Y),
        component_observer(|hp: &Hp| hp.current as f32 / hp.max as f32),
    ));

    commands.spawn((
        StatBarColor(css::PURPLE.into()),
        StatBarEmptyColor(css::BLACK.into()),
        StatBarBorder {
            color: css::DARK_GRAY.into(),
            thickness: 3.0,
        },
        StatBarValue(12.0 / 15.0),
        StatBarSize {
            full_length: 50.0,
            thickness: 6.0,
        },
        StatBarSubject(player),
        StatBarPosition(50.0 * Vec2::Y),
        component_observer(|mp: &Mp| mp.current as f32 / mp.max as f32),
    ));
}

fn move_player(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Speed), With<PlayerCharacter>>,
) {
    for (mut transform, player_speed) in query.iter_mut() {
        let mut m = Vec3::ZERO;
        if keyboard.pressed(KeyCode::KeyA) {
            m -= Vec3::X
        }
        if keyboard.pressed(KeyCode::KeyD) {
            m += Vec3::X
        }
        if keyboard.pressed(KeyCode::KeyS) {
            m -= Vec3::Y
        }
        if keyboard.pressed(KeyCode::KeyW) {
            m += Vec3::Y
        }
        transform.translation += time.delta_seconds() * player_speed.0 * m.normalize_or_zero();
    }
}

fn update_stats(
    mut cooldown: Local<f32>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut players: Query<(&mut Hp, &mut Mp), With<PlayerCharacter>>,
) {
    *cooldown -= time.delta_seconds();
    if 0.0 < *cooldown {
        return;
    } else {
        *cooldown = 0.1;
    }
    for (mut hp, mut mp) in players.iter_mut() {
        if keyboard.pressed(KeyCode::ArrowDown) {
            hp.current = (hp.current - 1).clamp(0, hp.max);
        }
        if keyboard.pressed(KeyCode::ArrowUp) {
            hp.current = (hp.current + 1).clamp(0, hp.max);
        }
        if keyboard.pressed(KeyCode::ArrowLeft) {
            mp.current = (mp.current - 1).clamp(0, mp.max);
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            mp.current = (mp.current + 1).clamp(0, mp.max);
        }
    }
}

fn death(mut commands: Commands, query: Query<(Entity, &Hp)>) {
    for (entity, hp) in query.iter() {
        if hp.current <= 0 {
            commands.entity(entity).despawn();
        }
    }
}

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(css::NAVY.into()))
        .add_plugins(DefaultPlugins)
        .add_plugins(StatBarsPlugin)
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2dBundle::default());
        })
        .add_systems(Startup, spawn_player)
        .add_systems(Update, (move_player, death, update_stats).chain())
        .run();
}
