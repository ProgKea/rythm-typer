use bevy::app::AppExit;
use bevy::prelude::*;
use rand::prelude::*;
use std::env;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Typing Test".to_string(),
            resizable: false,
            width: 800.0,
            height: 500.0,
            present_mode: bevy::window::PresentMode::Fifo,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .insert_resource(TargetString(get_random_words(env::args().next_back().unwrap().parse::<u32>().unwrap_or(10))))
        .add_system(write_text)
        .add_system(highlight_character.before(write_text))
        .add_system(check_quit)
        .run();
}

#[derive(Component)]
struct TargetText;

#[derive(Component)]
struct HighlightCharacter;

struct TargetString(String);

fn get_random_words(count: u32) -> String {
    let mut words: Vec<String> = include_str!("1-1000.txt")
        .split_whitespace()
        .map(|s| s.to_string() + " ")
        .collect();

    let mut rng = rand::thread_rng();
    words.shuffle(&mut rng);

    if count > 999 {
        words.join("")
    } else {
        words[0..=count as usize].join("")
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut target_string: ResMut<TargetString>,
) {
    let removed_chars = vec!['[', ']', '"'];

    for char in removed_chars {
        target_string.0 = target_string.0.replace(char, "");
    }
    target_string.0 = target_string.0.replace(',', " ");

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
                target_string.0.clone(),
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
                target_string.0.get(0..1).unwrap(),
                TextStyle {
                    font: asset_server.load("fonts/Ubuntu Mono.ttf"),
                    font_size: 50.0,
                    color: Color::GOLD,
                },
                Default::default(),
            ),
            ..default()
        })
        .insert(HighlightCharacter);
}

fn write_text(
    mut target_text: Query<&mut Text, With<TargetText>>,
    mut target_string: ResMut<TargetString>,
    mut char_evr: EventReader<ReceivedCharacter>,
    mut exit: EventWriter<AppExit>,
) {
    for ev in char_evr.iter() {
        if target_string.0.chars().next().unwrap() == ev.char {
            if target_string.0.len() > 1 {
                target_string.0 = target_string.0.clone().get(1..).unwrap().to_string();
                target_text.single_mut().sections[0].value = target_string.0.clone();
            } else {
                exit.send(AppExit);
            }
        }
    }
}

fn highlight_character(
    target_string: ResMut<TargetString>,
    mut highlight_character: Query<&mut Text, With<HighlightCharacter>>,
) {
    if target_string.is_changed() {
        for mut char in highlight_character.iter_mut() {
            char.sections[0].value = target_string.0.get(0..1).unwrap().to_string();
        }
    }
}

fn check_quit(keyboard_input: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
