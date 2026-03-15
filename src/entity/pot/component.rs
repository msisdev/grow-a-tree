use bevy::prelude::*;

#[derive(Component)]
pub struct Pot;

#[derive(Bundle)]
pub struct PotBundle {
    pub pot: Pot,
    
	pub transform: Transform,
	pub global_transform: GlobalTransform,

	pub visibility: Visibility,
	pub sprite: Sprite,

}
