use bevy::{prelude::*, window::WindowResized};
use crate::{global, ui::entity};

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let parent = entity::spawn_root(&mut commands);
    let button = entity::spawn_pot(
        &mut commands,
        &asset_server,
        &mut texture_atlas_layouts,
    );
    commands.entity(parent).add_child(button);
}

pub fn fit_ui_to_window(
    mut resize_messages: MessageReader<WindowResized>,
    mut ui_scale: ResMut<UiScale>,
) {
    for window_resized in resize_messages.read() {
        let quantized_width = (window_resized.width / 32.0).floor() * 32.0;
        let quantized_height = (window_resized.height / 32.0).floor() * 32.0;

        let h_scale = quantized_width / global::RES_WIDTH as f32;
        let v_scale = quantized_height / global::RES_HEIGHT as f32;
        
        ui_scale.0 = h_scale.min(v_scale).max(1.0);
    }
}
