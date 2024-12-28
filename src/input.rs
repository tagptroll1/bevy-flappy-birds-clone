use crate::GameState;
use bevy::prelude::*;

#[derive(Default, Event)]
pub struct JumpEvent;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<JumpEvent>()
            .add_systems(Update, (handle_input).run_if(in_state(GameState::Game)));
    }
}

fn handle_input(keys: Res<ButtonInput<KeyCode>>, mut jump_event: EventWriter<JumpEvent>) {
    if keys.just_pressed(KeyCode::Space) {
        jump_event.send_default();
    }
}
