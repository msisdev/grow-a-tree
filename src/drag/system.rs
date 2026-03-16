use bevy::{prelude::*, window::PrimaryWindow};

use crate::drag::*;

const DRAG_RADIUS: f32 = 50.0;

pub fn start_drag_system(
    mut commands: Commands,
    q_window: Single<&Window, With<PrimaryWindow>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    q_draggable: Query<(Entity, &GlobalTransform), With<Draggable>>,
) {
    let Some(cursor_pos) = q_window.into_inner().cursor_position() else {
        return;
    };

    if mouse_button.just_pressed(MouseButton::Left) {
        for (entity, global_transform) in q_draggable.iter() {
            if handle_grab(&mut commands, entity, global_transform, cursor_pos) {
                break;
            }
        }
    }
}

pub fn update_drag_system(
    mut commands: Commands,
    q_window: Single<&Window, With<PrimaryWindow>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut q_dragging: Query<(Entity, &mut Transform), With<IsDragging>>,
) {
    let Some(cursor_pos) = q_window.into_inner().cursor_position() else {
        return;
    };

    if mouse_button.pressed(MouseButton::Left) {
        for (_, mut transform) in q_dragging.iter_mut() {
            handle_drag(&mut transform, cursor_pos);
        }
    }
    if mouse_button.just_released(MouseButton::Left) {
        for (entity, _) in q_dragging.iter() {
            handle_release(&mut commands, entity);
        }
    }
}

fn handle_grab(
    commands: &mut Commands,
    entity: Entity,
    global_transform: &GlobalTransform,
    cursor_pos: Vec2,
) -> bool {
    let pos = global_transform.translation().truncate();

    if pos.distance(cursor_pos) < DRAG_RADIUS {
        commands.entity(entity).insert(IsDragging);
        true
    } else {
        false
    }
}

fn handle_drag(transform: &mut Transform, cursor_pos: Vec2) {
    transform.translation.x = cursor_pos.x;
    transform.translation.y = cursor_pos.y;
}

fn handle_release(commands: &mut Commands, entity: Entity) {
    commands.entity(entity).remove::<IsDragging>();
}
