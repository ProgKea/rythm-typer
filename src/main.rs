#![allow(unused)]

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(change_text)
        .run();
}

#[derive(Component)]
struct TargetText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Percent(50.0),
                    left: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
            text: Text::with_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/PixelSmall.ttf"),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
                Default::default(),
            ),
            ..default()
        })
        .insert(TargetText);
}

fn change_text(mut query: Query<&mut Text, With<TargetText>>, keyboard_input: Res<Input<KeyCode>>) {
    for mut text in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::A) {
            text.sections[0].value = format!("This is the text you have to write");
        }
    }
}
