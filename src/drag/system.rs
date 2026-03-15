use bevy::{prelude::*, window::PrimaryWindow};

use crate::drag::*;

pub fn drag_system(
    q_window: Single<&Window, With<PrimaryWindow>>,
    mut q_draggable: Query<&mut Transform, With<Draggable>>,
) {
    let window = q_window.into_inner();
    if let Some(cursor_pos) = window.cursor_position() {
        // Note: If you are using a 2D camera with a centered projection,
        // you may need to map screen coordinates to world coordinates.
        for mut transform in q_draggable.iter_mut() {
            transform.translation.x = cursor_pos.x;
            transform.translation.y = cursor_pos.y;
        }
    }
}
