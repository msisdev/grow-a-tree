use std::sync::{LazyLock, Mutex};

use bevy::prelude::*;
use crate::ui::component;

static TREE_INDEX: LazyLock<Mutex<Vec<Vec<usize>>>> = LazyLock::new(|| {
    Mutex::new(vec![
        vec![0],
        vec![1],
        vec![2],
        vec![3, 9],
        vec![4, 5, 10, 11, 16, 17],
    ])
});

#[derive(Component)]
pub struct TreeUi;

#[derive(Resource)]
pub struct TreeEntities {
    pub nodes: Vec<Entity>,
}

pub fn spawn_tree_ui(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) -> Entity {
    let root = commands.spawn((
        TreeUi,
        super::get_tile_node(3, 2, 2, 4),
    )).id();

    let texture: Handle<Image> = asset_server.load("sprite/tree.png");

    let tree_sprite_3 = [
        commands.spawn((
            ImageNode::from_atlas_image(
                texture.clone(),
                TextureAtlas {
                    layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                        UVec2::splat(32),
                        2,
                        1,
                        None,
                        None,
                    )),
                    index: 3,
                },
            ),
        )).id(),
    ];

    let tree_node_3 = commands.spawn((
        Node {
            ..default()
        }
    )).id();

    for entity in tree_sprite_entities_3 {
        commands.entity(tree_node_3).add_child(entity);
    }

    commands.entity(root).add_child(tree_node_3);

    root
}

fn create_tree(
    commands: &mut Commands,

)

// pub fn load_tree_entities(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
// ) {
//     let texture_handle = asset_server.load("sprite/tree.png");
    
//     // Insert the loaded handle into our globally accessible Resource
//     commands.insert_resource(TreeSpriteData { texture_handle });
// }

pub fn spawn_tree(
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
    tree_ui: Entity,
) {
    
}
