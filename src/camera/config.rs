use bevy::camera::visibility::RenderLayers;

/// SNES resolution
pub const RES_WIDTH: u32 = 256;
pub const RES_HEIGHT: u32 = 224;

pub const PIXEL_PERFECT_LAYER: RenderLayers = RenderLayers::layer(0);

pub const UI_LAYER: RenderLayers = RenderLayers::layer(1);

pub const HIGH_RES_LAYER: RenderLayers = RenderLayers::layer(2);
