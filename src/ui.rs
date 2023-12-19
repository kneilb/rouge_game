use crate::character::Money;
use bevy::prelude::*;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui)
            .add_systems(Update, update_money);
    }
}

#[derive(Component)]
pub struct MoneyText;

fn spawn_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::BLUE.into(),
                ..default()
            },
            Name::new("UI Root"),
        ))
        .with_children(|commands| {
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Money!",
                        TextStyle {
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    ..default()
                },
                MoneyText,
            ));
        });
}

fn update_money(mut query: Query<&mut Text, With<MoneyText>>, money: Res<Money>) {
    let mut money_text = query.single_mut();

    money_text.sections[0].value = format!("Money: £{:?}", money.0);
}
