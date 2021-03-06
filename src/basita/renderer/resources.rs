use sdl2::pixels::Color;
use sdl2::rect::Point;

use super::assets::{Font, Image};
use crate::core::assets::AssetCollection;

pub type Images = AssetCollection<Image>;
pub type Fonts = AssetCollection<Font>;

pub enum RenderVariant {
	Texture(usize),
	TextureEx(usize, bool, bool),
	Rect(u32, u32),
	RectFill(u32, u32),
	Line(Point),
	Point,
}

pub struct RenderCommand {
	pub layer: usize,
	pub color: Color,
	pub position: Point,
	pub variant: RenderVariant,
	_internal: (),
}

#[derive(Default)]
pub struct RenderCommands {
	pub commands: Vec<RenderCommand>,
}

impl RenderCommands {
	pub fn add_texture(
		&mut self,
		layer: usize,
		color: Color,
		position: Point,
		texture_index: usize,
	) {
		self.commands.push(RenderCommand {
			layer: layer,
			color: color,
			position: position,
			variant: RenderVariant::Texture(texture_index),
			_internal: (),
		})
	}

	pub fn add_texture_ex(
		&mut self,
		layer: usize,
		color: Color,
		position: Point,
		texture_index: usize,
		flip_horizontal: bool,
		flip_vertical: bool,
	) {
		self.commands.push(RenderCommand {
			layer: layer,
			color: color,
			position: position,
			variant: RenderVariant::TextureEx(texture_index, flip_horizontal, flip_vertical),
			_internal: (),
		})
	}

	pub fn add_rect(
		&mut self,
		layer: usize,
		color: Color,
		position: Point,
		width: u32,
		height: u32,
	) {
		self.commands.push(RenderCommand {
			layer: layer,
			color: color,
			position: position,
			variant: RenderVariant::Rect(width, height),
			_internal: (),
		})
	}

	pub fn add_rect_fill(
		&mut self,
		layer: usize,
		color: Color,
		position: Point,
		width: u32,
		height: u32,
	) {
		self.commands.push(RenderCommand {
			layer: layer,
			color: color,
			position: position,
			variant: RenderVariant::RectFill(width, height),
			_internal: (),
		})
	}

	pub fn add_line(&mut self, layer: usize, color: Color, position: Point, to: Point) {
		self.commands.push(RenderCommand {
			layer: layer,
			color: color,
			position: position,
			variant: RenderVariant::Line(to),
			_internal: (),
		})
	}

	pub fn add_point(&mut self, layer: usize, color: Color, position: Point) {
		self.commands.push(RenderCommand {
			layer: layer,
			color: color,
			position: position,
			variant: RenderVariant::Point,
			_internal: (),
		})
	}
}
