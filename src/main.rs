use bevy::prelude::*;
use grow_a_tree::camera::system::*;
use grow_a_tree::entity::pixel_perfect_layer::setup_pixel_perfect_layer;
use grow_a_tree::ui::system::{fit_ui_to_window, setup_ui};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_systems(Startup, (setup_camera, setup_pixel_perfect_layer))
        .add_systems(Startup, (setup_camera, setup_ui,))
        .add_systems(Update, fit_ui_to_window)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
