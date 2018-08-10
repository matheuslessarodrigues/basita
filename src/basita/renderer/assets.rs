use sdl2::rect::Point;

use core::assets::{Asset, AssetLoadError, AssetLoader};
use sdl::{SdlLoader, SdlStorage};

pub struct Image {
	pub index: usize,
	pub center: Point,
}

impl Asset for Image {
	type Id = String;
}

impl<'a> AssetLoader<'a, Image> for SdlLoader {
	type Storage = SdlStorage<'a>;

	fn load(
		&'a self,
		path: &<Image as Asset>::Id,
		storage: &mut Self::Storage,
	) -> Result<Image, AssetLoadError> {
		self.load_texture(path, storage).map(|index| {
			let texture = storage.texture_storage.at(index);
			let query = texture.query();

			Image {
				index: index,
				center: Point::new(query.width as i32 / 2, query.height as i32 / 2),
			}
		})
	}
}

pub struct Font {
	pub index: usize,
	pub size: usize,
}

impl Asset for Font {
	type Id = String;
}

impl<'a> AssetLoader<'a, Font> for SdlLoader {
	type Storage = SdlStorage<'a>;

	fn load(
		&'a self,
		path: &<Font as Asset>::Id,
		storage: &mut Self::Storage,
	) -> Result<Font, AssetLoadError> {
		self.load_font(path, 32, storage).map(|index| Font {
			index: index,
			size: 32,
		})
	}
}
