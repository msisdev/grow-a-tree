use bevy::prelude::*;

use crate::ui::component;

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
            super::get_tile_node(0, 0, 1, 1), // Use x, y coordinates here (example: 0, 0)
        ))
        .id()
}
