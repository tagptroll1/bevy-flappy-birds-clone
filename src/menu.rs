use crate::Highscore;

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
        app.add_systems(OnEnter(GameState::Menu), (menu_setup, hide_score))
            .add_systems(OnExit(GameState::Menu),(despawn_screen::<OnMenuScreen>, show_score))
            .add_systems(OnEnter(GameState::DeathScreen), (show_score, show_highscore, death_menu_setup))
            .add_systems(OnExit(GameState::DeathScreen), (despawn_screen::<OnDeathScreen>, hide_highscore))
            .add_systems(Startup, setup_score_ui)
            .add_systems(Update, update_scoreboard.run_if(in_state(GameState::Game)))
            .add_systems(
                Update,
                (close_menu_action).run_if(in_state(GameState::Menu).or(in_state(GameState::DeathScreen))),
            );
    }
}

const SCOREBOARD_FONT_SIZE: f32 = 33.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const TEXT_COLOR: Color = Color::BLACK;
const SCORE_COLOR: Color = Color::srgb(1.0, 0.7, 0.1);
const RETRY_TEXT_COLOR: Color = Color::srgb(0.1, 0., 0.);

#[derive(Component)]
struct OnMenuScreen;

#[derive(Component)]
struct OnDeathScreen;

#[derive(Component)]
struct ScoreboardUi;

#[derive(Component)]
struct HighscoreboardUi;

fn setup_score_ui(mut commands: Commands) {
    // Scoreboard
    commands
        .spawn((
            Text::new("Score: "),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(TEXT_COLOR),
            ScoreboardUi,
            Node {
                display: Display::None,
                position_type: PositionType::Absolute,
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            },
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(SCORE_COLOR),
        ));

    commands
        .spawn((
            Text::new("Highscore: "),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(TEXT_COLOR),
            HighscoreboardUi,
            Node {
                display: Display::None,
                position_type: PositionType::Absolute,
                top: add_to_px(SCOREBOARD_TEXT_PADDING, 50.),
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            },
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font_size: SCOREBOARD_FONT_SIZE,
                ..default()
            },
            TextColor(SCORE_COLOR),
        ));
}

fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            OnMenuScreen,
            Node {
                align_items: AlignItems::End,
                justify_content: JustifyContent::Center,
                height: Val::Percent(100.),
                width: Val::Percent(100.),
                bottom: Val::Percent(10.),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text("Press space to play".to_string()),
                TextColor(RETRY_TEXT_COLOR),
                TextFont {
                    font: asset_server
                        .load("embedded://flappyboi/../assets/fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    ..default()
                },
            ));
        });
}

fn death_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            OnDeathScreen,
            Node {
                align_items: AlignItems::End,
                justify_content: JustifyContent::Center,
                height: Val::Percent(100.),
                width: Val::Percent(100.),
                bottom: Val::Percent(10.),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text("Press space to retry".to_string()),
                TextColor(RETRY_TEXT_COLOR),
                TextFont {
                    font: asset_server
                        .load("embedded://flappyboi/../assets/fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    ..default()
                },
            ));
        });
}

fn show_score(
    mut score_ui: Single<(&mut ScoreboardUi, &mut Node), Without<HighscoreboardUi>>,
) {
    score_ui.1.as_mut().display = Display::Block;
}

fn hide_score(
    mut score_ui: Single<(&mut ScoreboardUi, &mut Node), Without<HighscoreboardUi>>,
){
    score_ui.1.as_mut().display = Display::None;
}

fn show_highscore(
    highscore: Res<Highscore>,
    mut highscore_ui: Single<(&mut HighscoreboardUi, &mut Node), Without<ScoreboardUi>>,
) {
    write_highscore(**highscore);
    highscore_ui.1.as_mut().display = Display::Block;
}

fn hide_highscore(
    mut highscore_ui: Single<(&mut HighscoreboardUi, &mut Node), Without<ScoreboardUi>>,
){
    highscore_ui.1.as_mut().display = Display::None;
}

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

fn update_scoreboard(
    score: Res<Score>,
    highscore: Res<Highscore>,
    score_root: Single<Entity, (With<ScoreboardUi>, With<Text>)>,
    highscore_root: Single<Entity, (With<HighscoreboardUi>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    *writer.text(*score_root, 1) = score.to_string();
    *writer.text(*highscore_root, 1) = highscore.to_string();
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


fn add_to_px(val: Val, amount: f32) -> Val {
    match val {
        Val::Px(px) => Val::Px(px + amount),
        _ => val,
    }
}
