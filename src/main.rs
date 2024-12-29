mod debug;
mod game;
mod input;
mod menu;
mod pipes;
mod player;
mod splash;

use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::{env, fs};

use bevy::audio::Volume;
use bevy::input::common_conditions::input_pressed;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    DeathScreen,
    Game,
}

#[derive(Resource, Deref, DerefMut)]
pub struct Score(pub usize);

#[derive(Resource, Deref, DerefMut)]
pub struct Highscore(pub usize);

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Flappy Birb".into(),
                    name: Some("bevy.app".into()),
                    resolution: (400., 400.).into(),
                    window_theme: Some(WindowTheme::Dark),
                    present_mode: PresentMode::AutoNoVsync,
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..default()
                    },
                    visible: true,
                    ..default()
                }),
                ..default()
            }),
    )
    .add_systems(Update, exit_game.run_if(input_pressed(KeyCode::Escape)))
    .add_systems(Startup, setup)
    //.add_plugins(debug::DebugPlugin)
    .init_state::<GameState>()
    .add_plugins((game::GamePlugin, splash::SplashPlugin, menu::MenuPlugin))
    .run();
}

fn exit_game(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit::Success);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let file_score = read_highscore_from_file();

    let theme_song =
        asset_server.load::<AudioSource>("embedded://flappyboi/../assets/audio/themesong.ogg");
    commands.insert_resource(Score(0));
    commands.insert_resource(Highscore(file_score));

    commands.spawn((
        Camera2d::default(),
        OrthographicProjection {
            viewport_origin: Vec2::new(0., 0.),
            ..OrthographicProjection::default_2d()
        },
    ));

    commands.spawn((
        AudioPlayer(theme_song.clone()),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            volume: Volume::new(0.5),
            ..default()
        },
    ));
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn read_highscore_from_file() -> usize {
    // Get the LOCALAPPDATA path
    let local_app_data = match env::var("LOCALAPPDATA") {
        Ok(path) => path,
        Err(_) => return 0,
    };

    // Construct the path to the highscore file
    let mut highscore_path = PathBuf::from(local_app_data);
    highscore_path.push("flappyboi");
    highscore_path.push("highscore.txt");

    // Ensure the directory exists
    if let Some(parent) = highscore_path.parent() {
        if fs::create_dir_all(parent).is_err() {
            error!("Failed to create directories in localappdata");
            return 0;
        }
    }

    // Create the file if it doesn't exist and set default highscore to 0
    if !highscore_path.exists() {
        if let Ok(mut file) = File::create(&highscore_path) {
            if file.write_all(b"0").is_err() {
                return 0;
            } else {
                error!("Failed to write default 0 to file")
            }
        } else {
            error!("Failed to create highscore file");
            return 0;
        }
    }

    // Read the highscore into an i32
    let mut file = match File::open(&highscore_path) {
        Ok(file) => file,
        Err(_) => return 0,
    };

    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        error!("Failed to read files content");
        return 0;
    }

    // Parse the highscore
    contents.trim().parse().unwrap_or(0)
}
