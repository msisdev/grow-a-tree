pub mod asset {
	use bevy::prelude::*;

	pub const DEFAULT: &str = "sprite/pot.png";

	pub fn default_layout() -> TextureAtlasLayout {
		TextureAtlasLayout::from_grid(
			UVec2::splat(32),
			2,
			1,
			None,
			None,
		)
	}
}

pub mod sprite {
	pub const DEFAULT: usize = 0;
	pub const WATERING: usize = 1;
}

pub mod animate {
	use bevy::prelude::*;
	use crate::animate::component::*;

	pub fn default_animation() -> Animation {
		let layers = vec![
			AnimationLayer::Still(AnimationStillLayer {
				index: 0,
			}),
			AnimationLayer::Linear(AnimationLinearLayer {
				indices: vec![0, 1],
				interval: std::time::Duration::from_millis(500),
				is_repeating: true,
			}),
		];

		let state = AnimationLayerState::new_from_layer(&layers[0]);

		Animation { layers, state }
	}
}
