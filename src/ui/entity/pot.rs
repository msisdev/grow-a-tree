use bevy::prelude::*;

use crate::{global, ui::component};

pub fn spawn_pot(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> Entity {
    let texture: Handle<Image> = asset_server.load("sprite/pot.png");

    let ui_image = ImageNode::from_atlas_image(
        texture,
        TextureAtlas {
            layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                UVec2::splat(32),
                2,
                1,
                None,
                None,
            )),
            index: 0,
        },
    );

    commands
        .spawn((
            component::DraggableButton,
            ui_image,
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(global::TILE_UNIT as f32),
                left: Val::Px(global::TILE_UNIT as f32),
                width: Val::Px(global::TILE_UNIT as f32),
                height: Val::Px(global::TILE_UNIT as f32),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .id()
}
