use bevy::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, character_movement)
            .add_systems(Startup, create_character)
            .insert_resource(Money(100.0));
    }
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

// TODO: I would have just put this on the player...!
#[derive(Resource)]
pub struct Money(pub f32);

fn create_character(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("character.png");

    commands.spawn((
        SpriteBundle {
            // TODO: use 16x16 sprite & remove this!
            sprite: Sprite {
                custom_size: Some(Vec2::new(16.0, 16.0)),
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
