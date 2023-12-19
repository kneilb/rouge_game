use crate::character::{Money, Player};
use bevy::prelude::*;

pub struct PigPlugin;

const PIG_BUY_PRICE: f32 = 10.0;
const PIG_SELL_PRICE: f32 = 15.0;

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

    if money.0 < PIG_BUY_PRICE {
        info!("Not enough cash!");
        return;
    }

    money.0 -= PIG_BUY_PRICE;
    info!("Spent £{} on a pig, remaining money £{:?}", PIG_BUY_PRICE, money.0);

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
            money.0 += PIG_SELL_PRICE;
            info!("Earnt £{}; money is now £{:?}", PIG_SELL_PRICE, money.0);

            commands.entity(pig_entity).despawn();
        }
    }
}
