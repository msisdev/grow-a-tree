use std::time::Duration;

use bevy::prelude::*;

use super::*;

pub fn animate_system(
	time: Res<Time>,
	mut query: Query<(
		&mut Animation,
		&mut Sprite,
	)>,
) {
	for (mut animation, mut sprite) in query.iter_mut() {
		let active_layer = match animation.layers.first() {
			Some(layer) => layer.clone(),
			None => continue,
		};

		animate(time.delta(), &mut animation.state, &active_layer, &mut sprite);
	}
}

fn animate(
	delta: Duration,
	state: &mut AnimationLayerState,
	active_layer: &AnimationLayer,
	sprite: &mut Sprite,
) {
	match (state, active_layer) {
		(
			AnimationLayerState::Still,
			AnimationLayer::Still(layer),
		) => {
			animate_still(layer, sprite);
		}
		(
			AnimationLayerState::Linear { current_index, timer },
			AnimationLayer::Linear(layer),
		) => {
			animate_linear(delta, current_index, timer, layer, sprite);
		}
		(
			AnimationLayerState::Sequence { current_sequence_index, timer },
			AnimationLayer::Sequence(layer),
		) => {
			animate_sequence(delta, current_sequence_index, timer, layer, sprite);
		}
		(state_ref, _) => {
			*state_ref = AnimationLayerState::new_from_layer(active_layer);
		}
	}
}

fn animate_still(layer: &AnimationStillLayer, sprite: &mut Sprite) {
	if let Some(atlas) = &mut sprite.texture_atlas {
		atlas.index = layer.index;
	}
}

fn animate_linear(
	delta: Duration,
	current_index: &mut usize,
	timer: &mut Timer,
	layer: &AnimationLinearLayer,
	sprite: &mut Sprite,
) {
	timer.tick(delta);

	if !timer.just_finished() {
		return;
	}
	if layer.indices.is_empty() {
		return;
	}
	if !layer.is_repeating && *current_index == layer.indices.len() - 1 {
		return;
	}

	// Increase & rotate index
	*current_index = (*current_index + 1) % layer.indices.len();

	// Update sprite
	if let Some(atlas) = &mut sprite.texture_atlas {
		atlas.index = layer.indices[*current_index];
	}
}

fn animate_sequence(
	delta: Duration,
	current_sequence_index: &mut usize,
	timer: &mut Timer,
	layer: &AnimationSequenceLayer,
	sprite: &mut Sprite,
) {
	timer.tick(delta);
	if !timer.just_finished() {
		return;
	}
	if layer.sequences.is_empty() {
		return;
	}
	if !layer.is_repeating && *current_sequence_index == layer.sequences.len() - 1 {
		return;
	}

	// Increase & rotate index
	*current_sequence_index = (*current_sequence_index + 1) % layer.sequences.len();
	let (next_index, next_duration) = layer.sequences[*current_sequence_index];
	
	// Update sprite
	if let Some(atlas) = &mut sprite.texture_atlas {
		atlas.index = next_index;
	}
	
	// Update timer
	timer.set_duration(next_duration);
	timer.reset();
}
