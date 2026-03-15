use bevy::prelude::*;
use std::time::Duration;

#[derive(Component, Debug, Clone)]
pub struct Animation {
    pub layers: Vec<AnimationLayer>,
    pub state: AnimationLayerState,
}

#[derive(Debug, Clone)]
pub enum AnimationLayer {
    Still(AnimationStillLayer),
    Linear(AnimationLinearLayer),
    Sequence(AnimationSequenceLayer),
}

#[derive(Clone, Debug)]
pub struct AnimationStillLayer {
    pub index: usize,
}

#[derive(Clone, Debug)]
pub struct AnimationLinearLayer {
    pub indices: Vec<usize>,
    pub interval: Duration,
    pub is_repeating: bool,
}

#[derive(Clone, Debug)]
pub struct AnimationSequenceLayer {
    pub sequences: Vec<(usize, Duration)>,
    pub is_repeating: bool,
}

#[derive(Debug, Clone)]
pub enum AnimationLayerState {
    Still,
    Linear {
        current_index: usize,
        timer: Timer,
    },
    Sequence {
        current_sequence_index: usize,
        timer: Timer,
    },
}

impl AnimationLayerState {
    pub fn new_from_layer(layer: &AnimationLayer) -> Self {
        match layer {
            AnimationLayer::Still(_) => Self::Still,
            AnimationLayer::Linear(l) => Self::Linear {
                current_index: 0,
                timer: Timer::new(l.interval, TimerMode::Repeating),
            },
            AnimationLayer::Sequence(s) => Self::Sequence {
                current_sequence_index: 0,
                timer: Timer::new(
                    s.sequences.first().map(|(_, d)| *d).unwrap_or(Duration::ZERO),
                    TimerMode::Once, // Sequence timers usually need repeating/updating dynamically
                ),
            },
        }
    }
}
