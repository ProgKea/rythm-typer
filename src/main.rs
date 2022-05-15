use bevy::app::AppExit;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .insert_resource(WindowDescriptor {
            present_mode: bevy::window::PresentMode::Mailbox,
            ..default()
        })
        .add_system(write_text)
        .add_system(check_quit)
        .run();
}

#[derive(Component)]
struct TargetText;

#[derive(Component)]
struct HighlightCharacter;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut target_string =
        reqwest::blocking::get("https://random-word-api.herokuapp.com/word?number=10")
            .unwrap()
            .text()
            .unwrap();
    let removed_chars = vec!['[', ']', '"'];

    for char in removed_chars {
        target_string = target_string.replace(char, "");
    }
    target_string = target_string.replace(',', " ");

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
                target_string.clone(),
                TextStyle {
                    font: asset_server.load("fonts/Ubuntu Mono.ttf"),
                    font_size: 50.0,
                    color: Color::WHITE,
                },
                Default::default(),
            ),
            ..default()
        })
        .insert(TargetText);
}

fn write_text(
    mut target_text: Query<&mut Text, With<TargetText>>,
    mut char_evr: EventReader<ReceivedCharacter>,
) {
    for ev in char_evr.iter() {
        for mut text in target_text.iter_mut() {
            if text.sections[0].value.clone().len() > 0 {
                if text.sections[0].value.clone().chars().next().unwrap() == ev.char {
                    text.sections[0].value =
                        format!("{}", text.sections[0].value.clone().get(1..).unwrap());
                }
            }
        }
    }
}

fn check_quit(keyboard_input: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
