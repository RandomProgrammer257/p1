use crate::{Rec, Transform, Window};
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct MouseStates {
    pub window_position: Vec2,
    pub world_position: Vec2,
    pub is_in_window: bool,
}

pub fn mouse_position_handler(
    mut mouse_state: ResMut<MouseStates>,
    window_query: Query<&Window>,
    player_query: Query<&Transform, With<Rec>>,
) {
    let window = window_query.single();

    if let Some(cursor_position) = window.cursor_position() {
        mouse_state.is_in_window = true;

        mouse_state.window_position.x = cursor_position.x;
        mouse_state.window_position.y = cursor_position.y;

        if let Ok(player_transform) = player_query.get_single() {
            mouse_state.world_position.x = cursor_position.x + player_transform.translation.x;
            mouse_state.world_position.y = cursor_position.y + player_transform.translation.y;
        }
    } else {
        mouse_state.is_in_window = false;
    }
}
