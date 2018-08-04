use sdl2::rect::Point;

use specs::{Join, Read, ReadStorage, System, Write};

use super::components::Sprite;
use super::resources::{Images, RenderCommand, RenderCommands};
use core::components::Transform;

pub struct RenderSystem;

impl<'s> System<'s> for RenderSystem {
	type SystemData = (
		ReadStorage<'s, Transform>,
		ReadStorage<'s, Sprite>,
		Read<'s, Images>,
		Write<'s, RenderCommands>,
	);

	fn run(&mut self, (transforms, sprites, images, mut commands): Self::SystemData) {
		commands.clear();

		for (transform, sprite) in (&transforms, &sprites).join() {
			let image = images.get(sprite.image);

			commands.push(RenderCommand {
				layer: sprite.layer,
				position: Point::new(transform.position.x as i32, transform.position.y as i32)
					- image.center,
				texture_index: image.texture_index,
			});
		}

		commands.sort_by_key(|c| c.layer);
	}
}
