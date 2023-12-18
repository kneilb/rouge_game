use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "kneilb Rougelike".into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, (character_movement, spawn_pig, pig_lifetime))
        .insert_resource(Money(100.0))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };
    commands.spawn(camera);

    let texture = asset_server.load("character.png");

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(16.0, 16.0)), // TODO: use 16x16 sprite & remove this!
                ..default()
            },
            texture,
            ..default()
        },
        Player { speed: 100.0 },
    ));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let movement_distance = player.speed * time.delta_seconds();

        if input.pressed(KeyCode::W) {
            transform.translation.y += movement_distance;
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= movement_distance;
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= movement_distance;
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += movement_distance;
        }
    }
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Component)]
pub struct Pig {
    pub lifetime: Timer,
}

// TODO: I would have just put this on the player...!
#[derive(Resource)]
pub struct Money(pub f32);

fn spawn_pig(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut money: ResMut<Money>,
    query: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let player_transform = query.single();

    if money.0 < 10.0 {
        info!("Not enough cash!");
        return;
    }

    money.0 -= 10.0;
    info!("Spent £10 on a pig, remaining money £{:?}", money.0);

    let texture = asset_server.load("pig.png");

    commands.spawn((
        SpriteBundle {
            texture,
            transform: *player_transform,
            ..default()
        },
        Pig {
            lifetime: Timer::from_seconds(2.0, TimerMode::Once),
        },
    ));
}

fn pig_lifetime(
    mut commands: Commands,
    mut money: ResMut<Money>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Pig)>,
) {
    for (pig_entity, mut pig) in &mut query {
        pig.lifetime.tick(time.delta());

        if pig.lifetime.finished() {
            money.0 += 15.0;
            info!("Earnt £15; money is now £{:?}", money.0);
        }

        commands.entity(pig_entity).despawn();
    }
}
