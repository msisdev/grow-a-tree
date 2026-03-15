use bevy::{prelude::*, window::PrimaryWindow};

use crate::drag::*;

pub fn drag_system(
    mut commands: Commands,
    q_window: Single<&Window, With<PrimaryWindow>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    q_draggable: Query<(Entity, &GlobalTransform), With<Draggable>>,
    mut q_dragging: Query<(Entity, &mut Transform), With<IsDragging>>,
) {
    let window = q_window.into_inner();
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    handle_grab(&mut commands, &mouse_button, &q_draggable, cursor_pos);
    handle_drag(&mouse_button, &mut q_dragging, cursor_pos);
    handle_release(&mut commands, &mouse_button, &q_dragging);
}

fn handle_grab(
    commands: &mut Commands,
    mouse_button: &Res<ButtonInput<MouseButton>>,
    q_draggable: &Query<(Entity, &GlobalTransform), With<Draggable>>,
    cursor_pos: Vec2,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        for (entity, global_transform) in q_draggable.iter() {
            let pos = global_transform.translation().truncate();
            if pos.distance(cursor_pos) < 50.0 {
                commands.entity(entity).insert(IsDragging);
                break;
            }
        }
    }
}

fn handle_drag(
    mouse_button: &Res<ButtonInput<MouseButton>>,
    q_dragging: &mut Query<(Entity, &mut Transform), With<IsDragging>>,
    cursor_pos: Vec2,
) {
    if mouse_button.pressed(MouseButton::Left) {
        for (_, mut transform) in q_dragging.iter_mut() {
            transform.translation.x = cursor_pos.x;
            transform.translation.y = cursor_pos.y;
        }
    }
}

fn handle_release(
    commands: &mut Commands,
    mouse_button: &Res<ButtonInput<MouseButton>>,
    q_dragging: &Query<(Entity, &mut Transform), With<IsDragging>>,
) {
    if mouse_button.just_released(MouseButton::Left) {
        for (entity, _) in q_dragging.iter() {
            commands.entity(entity).remove::<IsDragging>();
        }
    }
}
