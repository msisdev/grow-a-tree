use bevy::prelude::*;

use super::*;

pub fn load(
	commands: &mut Commands,
	asset_server: &AssetServer,
	texture_atlas_layouts: &mut Assets<TextureAtlasLayout>,
) {
	let texture: Handle<Image> = asset_server.load(config::asset::DEFAULT);

	let sprite = Sprite::from_atlas_image(
		texture,
		TextureAtlas {
			layout: texture_atlas_layouts.add(config::asset::default_layout()),
			index: config::sprite::DEFAULT,
		},
	);

	commands.spawn(component::PotBundle {
		pot: component::Pot,
		transform: Transform::default(),
		global_transform: GlobalTransform::default(),
		visibility: Visibility::Visible,
		sprite,
		animation: config::animate::default_animation(),
	});
}
