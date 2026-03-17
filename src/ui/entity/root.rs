use bevy::prelude::*;

use crate::{global, ui::component};

pub fn spawn_root(commands: &mut Commands) -> Entity {
    let root = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .id();

    let container = commands
        .spawn((
            Node {
                width: Val::Px(global::RES_WIDTH as f32),
                height: Val::Px(global::RES_HEIGHT as f32),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                ..default()
            },
        ))
        .id();

    commands.entity(root).add_child(container);

    container
}
