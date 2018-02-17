use std::cmp::Ordering;

use sdl2::rect::Rect;

use super::super::{EngineState,EngineEvents};
use super::System;

use components::Sprite;

pub struct RenderSystem {}

impl System for RenderSystem {
	fn update(state: &mut EngineState, events: &EngineEvents) {
		state.sprites.all.sort_unstable();

		let mut canvas = state.sdl_context.canvas.borrow_mut();

		for sprite in &state.sprites.all {
			let texture = &state.image_resources.get(sprite.image_resource).texture;
			let query = texture.query();

			let transform = state.transforms.get(sprite.transform);

			canvas
				.copy(
					texture,
					None,
					Rect::new(
						transform.position.x as i32,
						transform.position.y as i32,
						query.width,
						query.height,
					),
				)
				.unwrap();
		}
	}
}

impl<'a> PartialEq for Sprite<'a> {
	fn eq(&self, other: &Self) -> bool {
		return self.depth == other.depth;
	}
}

impl<'a> Eq for Sprite<'a> {}

impl<'a> PartialOrd for Sprite<'a> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl<'a> Ord for Sprite<'a> {
	fn cmp(&self, other: &Self) -> Ordering {
		other.depth.cmp(&self.depth)
	}
}
