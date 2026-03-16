use bevy::prelude::*;

use super::component;
use crate::global;

pub fn setup_pixel_perfect_layer(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let parent = spawn_root(&mut commands);
    let button = spawn_pot(
        &mut commands,
        &asset_server,
        &mut texture_atlas_layouts,
    );
    commands.entity(parent).add_child(button);
}

fn spawn_root(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                ..default()
            },
            Pickable::IGNORE,
            global::PIXEL_PERFECT_LAYER,
        ))
        .id()
}

fn spawn_pot(
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
