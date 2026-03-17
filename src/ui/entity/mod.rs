mod pot;
mod root;
mod tree;

pub use pot::*;
pub use root::*;
use bevy::prelude::*;

pub fn get_tile_node(x: usize, y: usize, w: usize, h: usize) -> Node {
    let tile_size = 32.0; // 256 width / 8 tiles = 32, 224 height / 7 tiles = 32
    Node {
        position_type: PositionType::Absolute,
        left: Val::Px(x as f32 * tile_size),
        bottom: Val::Px(y as f32 * tile_size),
        width: Val::Px(w as f32 * tile_size),
        height: Val::Px(h as f32 * tile_size),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}
