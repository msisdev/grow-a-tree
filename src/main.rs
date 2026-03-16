use bevy::prelude::*;
use grow_a_tree::camera::system::*;
use grow_a_tree::entity::pixel_perfect_layer::setup_pixel_perfect_layer;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_pixel_perfect_layer))
        // .add_systems(Update, fit_canvas)
        .run();
}
