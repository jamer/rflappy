// vi: ts=4 sw=4

use rsfml::graphics::{IntRect, Texture, RenderWindow};
use rsfml::graphics::rc::Sprite;
use rsfml::system::vector2::{ToVec, Vector2f, Vector2u};

pub struct Ground {
	window_size: Vector2f,
	image_size: Vector2f,
	frequency: f32,
	phase: f32,
	sprite: ~Sprite,
}

impl Ground {
	pub fn new(window_size: Vector2u, mut sprite: ~Sprite, frequency: f32) -> Ground {
		let texture_ref = sprite.get_texture()
			.expect("Ground's Sprite didn't have a texture");
		let mut texture = texture_ref.deref().borrow_mut();
		let image_size: Vector2u = texture.get_size();

		Ground::horizontally_repeat_sprites_texture(window_size, sprite, texture.deref_mut(), image_size);

		let mut ground = Ground {
			window_size: window_size.to_vector2f(),
			image_size: image_size.to_vector2f(),
			frequency: frequency,
			phase: 0.,
			sprite: sprite,
		};
		ground.update(0.);

		ground
	}

	fn horizontally_repeat_sprites_texture(window_size: Vector2u, sprite: &mut Sprite, texture: &mut Texture, image_size: Vector2u) {
		let rect: IntRect = sprite.get_texture_rect();
		let repeats: i32 = (window_size.x as f32 / image_size.x as f32 + 1.).ceil() as i32;
		sprite.set_texture_rect(&IntRect {
			left: rect.left,
			top: rect.top,
			width: rect.width * repeats,
			height: rect.height,
		});
		texture.set_repeated(true);
	}

	pub fn update(&mut self, seconds: f32) {
		self.phase += seconds;

		let wavelength = 1. / self.frequency;
		while self.phase >= wavelength {
			self.phase -= wavelength;
		}

		let new_position: Vector2f = Vector2f {
			x: -1. * self.image_size.x * self.phase * self.frequency,
			y: self.get_top(),
		};
		self.sprite.set_position(&new_position);
	}

	pub fn draw(&self, window: &mut RenderWindow) {
		window.draw(self.sprite);
	}

	pub fn get_top(&self) -> f32 {
		(self.window_size.y - self.image_size.y) as f32
	}
}
