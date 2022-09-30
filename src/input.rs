use bevy::prelude::*;

pub struct KeyboardInputState(pub super::snake::Direction);

impl Default for KeyboardInputState {
  fn default() -> Self {
    Self (
      super::snake::Direction::Up,
    )
  }
}

pub fn keyboard_input(mut state: ResMut<KeyboardInputState>, keyboard_input: Res<Input<KeyCode>>) {
    state.0 = if keyboard_input.just_pressed(KeyCode::Left) {
        super::snake::Direction::Left
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        super::snake::Direction::Right
    } else if keyboard_input.just_pressed(KeyCode::Up) {
        super::snake::Direction::Up
    } else if keyboard_input.just_pressed(KeyCode::Down) {
        super::snake::Direction::Down
    } else {
        state.0
    }
}

