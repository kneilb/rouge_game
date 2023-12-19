use crate::character::{Money, Player};
use bevy::prelude::*;

pub struct PigPlugin;

impl Plugin for PigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_pig, pig_lifetime))
            .register_type::<Pig>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Pig {
    pub lifetime: Timer,
}

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
            // TODO: use 16x16 sprite & remove this!
            sprite: Sprite {
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
            texture,
            transform: *player_transform,
            ..default()
        },
        Pig {
            lifetime: Timer::from_seconds(2.0, TimerMode::Once),
        },
        Name::new("Pig"),
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
