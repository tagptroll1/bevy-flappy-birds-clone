use super::{despawn_screen, GameState};
use bevy::asset::embedded_asset;
use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;
use bevy::sprite::Anchor::TopCenter;
use rand::Rng;

pub struct PipesPlugin;

#[derive(Component)]
#[require(Sprite)]
pub struct Pipe {
    pub flipped: bool,
    pub passed: bool,
}

impl Plugin for PipesPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "../assets/pipe.png");
        app.add_systems(OnEnter(GameState::Game), spawn_pipes)
            .add_systems(OnExit(GameState::Game), despawn_screen::<Pipe>)
            .add_systems(Update, move_pipes.run_if(in_state(GameState::Game)));
    }
}

const PIPE_OPENING: f32 = 120.;
const PIPE_GAP: f32 = 250.;
const PIPE_SPEED: f32 = 150.;
const PIPE_Y_RANGE_MIN: i32 = 70;
const PIPE_Y_RANGE_MAX: i32 = 300;
const PIPE_WIDTH: f32 = 52.0; // Width of the pipe sprite
const PIPE_HEIGHT: f32 = 320.0; // Height of the pipe sprite

fn spawn_pipes(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut random = rand::thread_rng();

    for i in 0..5 {
        let rand_y = random.gen_range(PIPE_Y_RANGE_MIN..=PIPE_Y_RANGE_MAX) as f32;
        let (top_y, bot_y) = get_pipe_y(rand_y);

        commands.spawn((
            Pipe {
                flipped: false,
                passed: false,
            },
            Sprite {
                image: asset_server.load("embedded://flappyboi/../assets/pipe.png"),
                anchor: TopCenter,
                ..default()
            },
            Transform {
                translation: Vec3::new(400. + (i as f32 * PIPE_GAP), bot_y, 0.),
                ..default()
            },
        ));
        commands.spawn((
            Pipe {
                flipped: true,
                passed: false,
            },
            Sprite {
                image: asset_server.load("embedded://flappyboi/../assets/pipe.png"),
                anchor: TopCenter,
                ..default()
            },
            Transform {
                translation: Vec3::new(400. + (i as f32 * PIPE_GAP), top_y, 0.),
                scale: Vec3::new(1., -1., 1.),
                ..default()
            },
        ));
    }
}

fn move_pipes(
    mut pipe_q: Query<(&mut Transform, &mut Pipe)>,
    time: Res<Time>,
) {
    let mut random = rand::thread_rng();
    let this_loops_random_y = random.gen_range(PIPE_Y_RANGE_MIN..PIPE_Y_RANGE_MAX) as f32;
    let mut end_spawn = 0.;

    if let Some(last) = pipe_q
        .iter()
        .max_by(|&x, &y| x.0.translation.x.total_cmp(&y.0.translation.x))
    {
        let transform = last.0;
        end_spawn = transform.translation.x + 250.;
    }
    for (mut transform, mut pipe) in pipe_q.iter_mut() {
        transform.translation.x -= PIPE_SPEED * time.delta_secs();
        if transform.translation.x < -52. {
            let (top, bottom) = get_pipe_y(this_loops_random_y);
            transform.translation.x = end_spawn;

            if pipe.flipped {
                transform.translation.y = top;
            } else {
                transform.translation.y = bottom;
            }
            pipe.passed = false;
        }
    }
}

fn get_pipe_y(y: f32) -> (f32, f32) {
    // y 0 is bottom
    // The top pipe is flipped and has it's anchor at bottom left
    // The bot pipe has its anchor at top left
    let top = y + PIPE_OPENING / 2.; //600. + y + PIPE_OPENING / 2.;
    let bottom = y - PIPE_OPENING / 2.; //- y + PIPE_OPENING / 2.;

    (top, bottom)
}

pub fn pipe_to_aabb2d(transform: &Transform, flipped: bool) -> Aabb2d {
    // Extract the center position from the transform
    let center = transform.translation.xy();

    // Half-size for the AABB is derived from the pipe dimensions
    let half_size = Vec2::new(PIPE_WIDTH / 2.0, PIPE_HEIGHT / 2.0);

    // Adjust the center based on whether the pipe is flipped
    let adjusted_center = if flipped {
        center + Vec2::new(0.0, PIPE_HEIGHT / 2.0) // Move center down
    } else {
        center - Vec2::new(0.0, PIPE_HEIGHT / 2.0) // Move center up
    };

    // Create the AABB
    Aabb2d::new(adjusted_center, half_size)
}
