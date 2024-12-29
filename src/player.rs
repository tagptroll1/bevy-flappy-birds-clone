use super::Score;
use crate::game::{DeathSound, FlopSound, WohoSound};
use crate::input::JumpEvent;
use crate::pipes::{pipe_to_aabb2d, Pipe};
use crate::{despawn_screen, input, GameState, Highscore};
use bevy::asset::embedded_asset;
use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};
use bevy::prelude::*;

const GRAVITY: f32 = -600.;
const MAX_FALL_SPEED: f32 = -800.;
const PLAYER_JUMP_SPEED: f32 = 300.;
const PLAYER_SIZE: (f32, f32) = (34., 24.);

const SCREEN_HEIGHT: f32 = 400.;
#[derive(Component, Default)]
#[require(Sprite)]
pub struct Bird {
    speed: f32,
    angle: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "../assets/bird.png");
        app.add_plugins(input::InputPlugin)
            .add_systems(
                Update,
                (
                    jump,
                    check_bounds,
                    give_score_for_passing,
                    check_pipe_collision,
                )
                    .run_if(in_state(GameState::Game)),
            )
            // This would need to check on GameState?
            .add_systems(OnEnter(GameState::Game), spawn_player)
            .add_systems(OnExit(GameState::Game), despawn_screen::<Bird>);
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Bird::default(),
        Sprite {
            image: asset_server.load("embedded://flappyboi/../assets/bird.png"),
            ..default()
        },
        Transform {
            translation: Vec3::new(200., SCREEN_HEIGHT / 2., 5.),
            ..default()
        },
    ));
}

fn check_bounds(
    mut bird_q: Query<(&mut Transform, &mut Bird)>,
    mut game_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    death_sound: Res<DeathSound>,
) {
    let (mut transform, mut bird) = bird_q.single_mut();
    if (transform.translation.y - transform.scale.y / 2.) <= 0. {
        game_state.set(GameState::DeathScreen);
        commands.spawn((AudioPlayer(death_sound.clone()), PlaybackSettings::DESPAWN));
        reset_bird(&mut bird, &mut transform);
    }
}

fn jump(
    time: Res<Time>,
    mut jump_events: EventReader<JumpEvent>,
    bird_q: Single<(&mut Bird, &mut Transform)>,
    mut commands: Commands,
    flop_sound: Res<FlopSound>,
) {
    let dt = time.delta_secs();
    let jumped = !jump_events.is_empty();
    jump_events.clear();
    let (mut bird, mut transform) = bird_q.into_inner();

    if jumped {
        bird.speed = PLAYER_JUMP_SPEED;
        commands.spawn((AudioPlayer(flop_sound.clone()), PlaybackSettings::DESPAWN));
    } else {
        bird.speed += GRAVITY * dt;
        bird.speed = bird.speed.max(MAX_FALL_SPEED);
    }
    transform.translation.y += bird.speed * dt;

    if (transform.translation.y - transform.scale.y / 2.) > SCREEN_HEIGHT {
        bird.speed = 0.0;
    }
    transform.translation.y = transform.translation.y.clamp(0., SCREEN_HEIGHT);

    // Set bird rotation based on speed.
    if bird.speed > 0.0 {
        // Rotate left.
        bird.angle += 600.0 * dt;
    } else if bird.speed < -110.0 {
        // Rotate right.
        bird.angle -= 480.0 * dt;
    }
    bird.angle = bird.angle.clamp(-90.0, 30.0);
    transform.rotation = Quat::from_rotation_z(bird.angle.to_radians());

    // // DEBUG
    // gizmos.rect_2d(
    //     Isometry2d::new(
    //         transform.translation.truncate(),
    //         Rot2::from(bird.angle),
    //     ),
    //     Vec2::from(PLAYER_SIZE),
    //     Color::srgb(1., 1., 1.));
}

fn give_score_for_passing(
    mut score: ResMut<Score>,
    mut highscore: ResMut<Highscore>,
    bird_q: Single<&Transform, (With<Bird>, Without<Pipe>)>,
    mut pipes_q: Query<(&mut Pipe, &mut Transform)>,
    mut commands: Commands,
    woho_sound: Res<WohoSound>,
) {
    let bird_transform = bird_q.into_inner();

    for (mut pipe, transform) in pipes_q.iter_mut() {
        // Prevent giving score every tick once we pass a pipe
        if pipe.passed {
            continue;
        }
        if transform.translation.x < bird_transform.translation.x - 30. {
            pipe.passed = true;

            // Only give score for one of the 2 pipes it passes, flipped is unique in the pair
            if pipe.flipped {
                **score += 1;
                if score.gt(&highscore) {
                    **highscore = score.clone();
                }

                commands.spawn((AudioPlayer(woho_sound.clone()), PlaybackSettings::DESPAWN));
            }
        }
    }
}

fn check_pipe_collision(
    bird_q: Single<(&mut Transform, &mut Bird), Without<Pipe>>,
    pipes_q: Query<(&Transform, &Pipe), Without<Bird>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    death_sound: Res<DeathSound>,
) {
    let (mut bird_transform, mut bird) = bird_q.into_inner();
    for (pipe_transform, pipe) in pipes_q.iter() {
        let b_translate = bird_transform.translation.truncate();
        let bird_circle = BoundingCircle::new(b_translate, (PLAYER_SIZE.1 / 2.) - 1.);
        let collides = bird_collides(
            bird_circle,
            pipe_to_aabb2d(pipe_transform, pipe.flipped),
        );

        if collides {
            game_state.set(GameState::DeathScreen);
            commands.spawn((AudioPlayer(death_sound.clone()), PlaybackSettings::DESPAWN));
            reset_bird(&mut bird, &mut bird_transform);
        }
    }
}

// Returns `Some` if `ball` collides with `bounding_box`.
// The returned `Collision` is the side of `bounding_box` that `ball` hit.
fn bird_collides(bird: BoundingCircle, bounding_box: Aabb2d) -> bool {
    bird.intersects(&bounding_box)
}

fn reset_bird(bird: &mut Bird, transform: &mut Transform) {
    transform.translation.y = SCREEN_HEIGHT / 2.;
    transform.rotation = Quat::from_rotation_z(0.);
    bird.speed = 0.;
    bird.angle = 0.;
}