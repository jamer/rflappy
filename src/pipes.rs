// vi: ts=4 sw=4

use entity::Entity;
use rand::{Rng, task_rng};
use rsfml::graphics::{Image, RenderWindow, Texture};
use rsfml::graphics::rc::Sprite;
use rsfml::system::vector2::{ToVec, Vector2f};
use std::cell::RefCell;
use std::clone::Clone;
use std::rc::Rc;
use std::vec::Vec;

pub struct Pipes {
	image_size: Vector2f,
	top_pipe_texture: Rc<RefCell<Texture>>,
	bottom_pipe_texture: Rc<RefCell<Texture>>,

	/**
	 * Minimum x-position at which new pipes spawn by.
	 */
	min_spawn_x: f32,
	/**
	 * Width of a pipe's bounding box for collision detection purposes.  A
	 * pipe's bounding box is horizontally centered at the center of the pipe's
	 * graphic.
	 */
	collision_width: f32,
	/**
	 * Horizontal space between pipes.
	 */
	pipe_x_gap: f32,
	/**
	 * Vertical space between pipes.
	 */
	pipe_y_gap: f32,
	/**
	 * Minimum height the top pipe must be.
	 */
	top_height_min: f32,
	/**
	 * Maximum height the top pipe can be.
	 */
	top_height_max: f32,
	x_velocity: f32,

	/**
	 * The pipes themselves.
	 */
	sprites: Vec<Sprite>,
}

impl Entity for Pipes {
	fn reset(&mut self) {
		self.sprites = Vec::new();
	}

	fn update(&mut self, seconds: f32) {
		let x_movement: f32 = self.x_velocity * seconds;
		if !self.sprites.last().is_some() {
			let x: f32 = self.min_spawn_x;
			self.insert_pipe_pair(x);
		}
		for s in self.sprites.mut_iter() {
			s.move(&Vector2f {
				x: x_movement,
				y: 0.,
			});
		}
		loop {
			let last_pipe_x = self.sprites.last().expect("Couldn't get last Sprite").get_position().x;
			if last_pipe_x > self.min_spawn_x { break; }
			let x: f32 = last_pipe_x + self.pipe_x_gap;
			self.insert_pipe_pair(x);
		}
	}

	fn draw(&self, window: &mut RenderWindow) {
		for s in self.sprites.iter() {
			window.draw(s);
		}
	}
}

impl Pipes {
	pub fn new(top_pipe_image: ~Image, min_spawn_x: f32,
			collision_width: f32, pipe_x_gap: f32, pipe_y_gap: f32,
			top_height_min: f32, top_height_max: f32, x_velocity: f32) -> Pipes {
		Pipes {
			image_size: top_pipe_image.get_size().to_vector2f(),
			top_pipe_texture: Rc::new(RefCell::new(Pipes::texture_from_image(top_pipe_image))),
			bottom_pipe_texture: Rc::new(RefCell::new(Pipes::texture_from_image_flipped(top_pipe_image))),

			min_spawn_x: min_spawn_x,
			collision_width: collision_width,
			pipe_x_gap: pipe_x_gap,
			pipe_y_gap: pipe_y_gap,
			top_height_min: top_height_min,
			top_height_max: top_height_max,
			x_velocity: x_velocity,

			sprites: Vec::new(),
		}
	}

	fn texture_from_image(image: &Image) -> Texture {
		Texture::new_from_image(image).expect("Couldn't create Texture from Image")
	}

	fn texture_from_image_flipped(image: &Image) -> Texture {
		let mut image_clone = image.clone().expect("Couldn't clone Image");
		image_clone.flip_vertically();
		Texture::new_from_image(&image_clone).expect("Couldn't create Texture from Image")
	}

	fn insert_pipe_pair(&mut self, x: f32) {
		let mut top: Sprite = self.make_top_pipe();
		let mut bottom: Sprite = self.make_bottom_pipe();
		let gap_height: f32 = task_rng().gen_range(self.top_height_min, self.top_height_max);

		top.set_position(&Vector2f {
			x: x,
			y: gap_height
		});
		bottom.set_position(&Vector2f {
			x: x,
			y: gap_height + self.pipe_y_gap,
		});
		self.sprites.push(top);
		self.sprites.push(bottom);
	}

	fn make_top_pipe(&self) -> Sprite {
		let mut sprite = Sprite::new_with_texture(self.top_pipe_texture.clone()).expect("Couldn't create pipe Sprite");
		sprite.set_origin(&Vector2f {
			x: self.image_size.x / 2.,
			y: self.image_size.y,
		});

		sprite
	}

	fn make_bottom_pipe(&self) -> Sprite {
		let mut sprite = Sprite::new_with_texture(self.bottom_pipe_texture.clone()).expect("Couldn't create pipe Sprite");
		sprite.set_origin(&Vector2f {
			x: self.image_size.x / 2.,
			y: 0.,
		});

		sprite
	}
}
