use bevy::{camera::{RenderTarget, Viewport}, prelude::*, window::WindowResized};
use super::{config, component};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 0,
            ..default()
        },
        IsDefaultUiCamera,
        component::PixelPerfectCamera,
        config::PIXEL_PERFECT_LAYER,
    ));

    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        component::OuterCamera,
        config::HIGH_RES_LAYER,
    ));
}

pub fn fit_canvas(
    mut resize_messages: MessageReader<WindowResized>,
    mut projection: Single<&mut Projection, With<component::PixelPerfectCamera>>,
) {
    let Projection::Orthographic(projection) = &mut **projection else {
        return;
    };
    for window_resized in resize_messages.read() {
        let h_scale = window_resized.width / config::RES_WIDTH as f32;
        let v_scale = window_resized.height / config::RES_HEIGHT as f32;
        projection.scale = 1. / h_scale.min(v_scale).round();
    }
}
