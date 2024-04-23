use bevy::{prelude::*, sprite::Anchor};

#[derive(Resource, Deref, DerefMut, Default)]
pub struct Score(u32);

#[derive(Component, Default)]
pub struct ScoreDisplay;

pub fn spawn_score_display(mut commands: Commands) {
    commands.spawn((
        ScoreDisplay,
        Text2dBundle {
            text: Text::from_section(
                "Score: 0",
                TextStyle {
                    font_size: 8.0,
                    color: Color::WHITE,
                    ..default()
                },
            )
            .with_no_wrap(),
            transform: Transform::from_xyz(-62.0, 63.0, 0.0),
            text_anchor: Anchor::TopLeft,
            ..default()
        },
    ));
}

pub fn update_score_display(mut query: Query<&mut Text, With<ScoreDisplay>>, score: Res<Score>) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("Score: {}", **score);
}
