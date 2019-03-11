use basita::{math::Vector2, renderer::components::Camera};

use crate::MyGame;

pub struct CameraEntity {
	camera: Camera,
}

pub fn new(lazy: &mut LazyEvaluations<MyGame>, position: Vector2) {
	lazy.add(move |context, game| {
		game.cameras.push(CameraEntity {
			camera: Camera { position: position },
		});
	});
}
