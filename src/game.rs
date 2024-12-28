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

        app.add_systems(OnEnter(GameState::Game), game_setup)
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
}
