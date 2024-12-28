use super::{despawn_screen, GameState};
use bevy::{asset::embedded_asset, prelude::*};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "../assets/icon.png");
        app.add_systems(OnEnter(GameState::Splash), splash_setup)
            .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
            .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
    }
}

fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).just_finished() {
        game_state.set(GameState::Menu);
    }
}

#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("embedded://flappyboi/../assets/icon.png");

    // Spawn 2 entities.
    // First entity is a Node that is aligned center, and has the component OnSplashScreen
    // for grouping
    // Second entity is a child of the center Node that contains a width 100 ImageNode + Node
    commands
        .spawn((
            Node {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            BackgroundColor(Color::BLACK),
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageNode::new(icon),
                Node {
                    width: Val::Px(200.),
                    ..default()
                },
            ));
        });
    // Spawn timer
    commands.insert_resource(SplashTimer(Timer::from_seconds(1.5, TimerMode::Once)));
}
