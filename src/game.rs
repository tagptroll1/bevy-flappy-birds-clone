use super::GameState;
use crate::{pipes, player};
use bevy::{asset::embedded_asset, prelude::*};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "../assets/audio/flop.ogg");
        embedded_asset!(app, "../assets/audio/death.ogg");
        embedded_asset!(app, "../assets/audio/woho.ogg");
        embedded_asset!(app, "../assets/audio/themesong.ogg");
        embedded_asset!(app, "../assets/bg.png");

        app.add_systems(Startup, game_setup)
            .add_systems(Update, move_background.run_if(in_state(GameState::Game)))
            // These should only really work if State is Game
            .add_plugins(player::PlayerPlugin)
            .add_plugins(pipes::PipesPlugin);
    }
}

#[derive(Resource, Deref)]
pub struct FlopSound(Handle<AudioSource>);
#[derive(Resource, Deref)]
pub struct DeathSound(Handle<AudioSource>);
#[derive(Resource, Deref)]
pub struct WohoSound(Handle<AudioSource>);

#[derive(Component)]
struct BackgroundTile;

const BACKGROUND_WIDTH: f32 = 400.; //1024.;
const BACKGROUND_SPEED: f32 = 100.;

fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let flop_sound =
        asset_server.load::<AudioSource>("embedded://flappyboi/../assets/audio/flop.ogg");
    let death_sound =
        asset_server.load::<AudioSource>("embedded://flappyboi/../assets/audio/death.ogg");
    let woho_sound =
        asset_server.load::<AudioSource>("embedded://flappyboi/../assets/audio/woho.ogg");

    commands.insert_resource(FlopSound(flop_sound));
    commands.insert_resource(DeathSound(death_sound));
    commands.insert_resource(WohoSound(woho_sound));

    commands.spawn((
        BackgroundTile,
        Sprite{
            image: asset_server.load("embedded://flappyboi/../assets/bg.png"),
            custom_size: Some(Vec2::new(400., 400.)),
            anchor: bevy::sprite::Anchor::BottomLeft,
            ..default()
        },
        Transform::default(),
    ));

    commands.spawn((
        BackgroundTile,
        Sprite {
            image: asset_server.load("embedded://flappyboi/../assets/bg.png"),
            custom_size: Some(Vec2::new(400., 400.)),
            anchor: bevy::sprite::Anchor::BottomLeft,
            ..default()
        },
        Transform {
            translation: Vec3::new(BACKGROUND_WIDTH, 0., 0.),
            ..default()
        }
    ));
}

fn move_background(
    mut background_q: Query<&mut Transform, With<BackgroundTile>>,
    time: Res<Time>,
) {
    for mut transform in background_q.iter_mut() {
        transform.translation.x -= BACKGROUND_SPEED * time.delta_secs();
        if transform.translation.x < -BACKGROUND_WIDTH {
            transform.translation.x = BACKGROUND_WIDTH;
        }
    }
}