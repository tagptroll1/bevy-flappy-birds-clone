use crate::{Highscore, HighscoreboardUi};

use super::{despawn_screen, GameState, Score};
use bevy::asset::embedded_asset;
use bevy::prelude::*;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "../assets/fonts/FiraSans-Bold.ttf");
        app.add_systems(OnEnter(GameState::Menu), (menu_setup, show_highscore))
            .add_systems(
                OnExit(GameState::Menu),
                (despawn_screen::<OnMenuScreen>, hide_highscore),
            )
            .add_systems(
                Update,
                (close_menu_action).run_if(in_state(GameState::Menu)),
            );
    }
}

fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            OnMenuScreen,
            Node {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                height: Val::Percent(100.),
                width: Val::Percent(100.),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text("Press space to play".to_string()),
                TextColor(Color::srgb(1., 1., 0.)),
                TextFont {
                    font: asset_server
                        .load("embedded://flappyboi/../assets/fonts/FiraSans-Bold.ttf"),
                    font_size: 50.0,
                    ..default()
                },
            ));
        });
}

fn show_highscore(
    highscore: Res<Highscore>,
    mut highscore_ui: Single<(&mut HighscoreboardUi, &mut Node)>,
) {
    write_highscore(**highscore);
    highscore_ui.1.as_mut().display = Display::Block;
}
fn hide_highscore(mut highscore: Single<(&mut HighscoreboardUi, &mut Node)>) {
    highscore.1.as_mut().display = Display::None;
}

#[derive(Component)]
struct OnMenuScreen;

fn close_menu_action(
    keys: Res<ButtonInput<KeyCode>>,
    mut score: ResMut<Score>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        **score = 0;
        game_state.set(GameState::Game);
    }
}

fn write_highscore(highscore: usize) {
    // Get the LOCALAPPDATA path
    let local_app_data = match env::var("LOCALAPPDATA") {
        Ok(path) => path,
        Err(_) => {
            error!("Could not find LOCALAPPDATA environment variable");
            return;
        }
    };

    // Construct the path to the highscore file
    let mut highscore_path = PathBuf::from(local_app_data);
    highscore_path.push("flappyboi");
    highscore_path.push("highscore.txt");

    // Ensure the directory exists
    if let Some(parent) = highscore_path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            error!("Failed to create directory: {}", e);
            return;
        }
    }

    // Write the highscore to the file
    if let Err(e) = File::create(&highscore_path).and_then(|mut file| write!(file, "{}", highscore))
    {
        error!("Failed to write highscore: {}", e);
    }
}
