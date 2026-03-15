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
