use std;
use nalgebra as na;
use glutin;
use glutin::dpi::LogicalSize;

use super::geometry::{rotate_x, rotate_y};

pub struct Viewer {
	camera_orbit_    : na::Vector2<f64>,
	camera_zoom_     : f64,
	viewport_size_   : LogicalSize,
	field_of_view_   : f64,
	clip_near_       : f64,
	clip_far_        : f64,

	close_requested_ : bool,
	mouse_down_      : bool,
	mouse_position_  : na::Vector2<f64>,

	perspective_     : na::Projective3<f32>,
	camera_          : na::Similarity3<f32>,
}

impl Viewer {
	pub fn new(viewport_size: LogicalSize, field_of_view: f64, clip_near: f64, clip_far: f64) -> Self {
		let mut viewer = Self {
			camera_orbit_   : na::Vector2::zeros(),
			camera_zoom_    : 1.0,
			viewport_size_  : viewport_size,
			field_of_view_  : field_of_view,
			clip_near_      : clip_near,
			clip_far_       : clip_far,

			close_requested_ : false,
			mouse_down_      : false,
			mouse_position_  : na::Vector2::zeros(),

			perspective_ : na::Projective3::identity(),
			camera_      : na::Similarity3::identity(),
		};

		viewer.update_perspective();
		viewer.update_camera();
		viewer
	}

	pub fn perspective(&self) -> &na::Projective3<f32> {
		&self.perspective_
	}

	pub fn camera(&self) -> &na::Similarity3<f32> {
		&self.camera_
	}

	pub fn close_requested(&self) -> bool {
		return self.close_requested_;
	}

	pub fn process_event(&mut self, event: &glutin::Event) {
		match event {
			glutin::Event::WindowEvent{event, ..} => self.process_window_event(event),
			glutin::Event::DeviceEvent{..}        => (),
			glutin::Event::Awakened               => (),
			glutin::Event::Suspended(_suspended)  => (),
		}
	}

	pub fn process_window_event(&mut self, event: &glutin::WindowEvent) {
		match event {
			glutin::WindowEvent::Resized(size) => {
				self.viewport_size_ = *size;
				self.update_perspective();
			},
			glutin::WindowEvent::CloseRequested => {
				self.close_requested_ = true;
			},
			glutin::WindowEvent::Focused(focused) => {
				self.mouse_down_ = self.mouse_down_ && *focused;
			},
			glutin::WindowEvent::CursorMoved{position, ..} => {
				let new_position = na::Vector2::new(position.x, position.y);
				let difference   = new_position - self.mouse_position_;
				if self.mouse_down_ {
					self.camera_orbit_ += difference;
					self.update_camera();
				}
				self.mouse_position_ = new_position;
			},
			glutin::WindowEvent::MouseWheel{delta, ..} => {
				self.camera_zoom_ += delta.pixel_delta().y;
				self.update_camera();
			},
			glutin::WindowEvent::MouseInput{button, state, ..} => {
				if *button == glutin::MouseButton::Left {
					self.mouse_down_ = *state == glutin::ElementState::Pressed;
				}
			},
			_ => (),
		}
	}

	fn update_perspective(&mut self) {
		let ratio = self.viewport_size_.width / self.viewport_size_.height;
		let perspective = na::Perspective3::new(ratio, self.field_of_view_, self.clip_near_, self.clip_far_);
		self.perspective_ = na::convert(na::Projective3::from_matrix_unchecked(*perspective.as_matrix()));
	}

	fn update_camera(&mut self) {
		let zoom     = 1.01f64.powf(self.camera_zoom_);
		let orbit    = self.camera_orbit_ * std::f64::consts::PI / 1000.0;
		let isometry = na::Translation3::new(0., 0., -1.) * rotate_x(orbit.y) * rotate_y(orbit.x);
		let camera   = na::Similarity::from_isometry(isometry, zoom);
		self.camera_ = na::convert(camera);
	}
}

trait PixelDelta {
	fn pixel_delta(&self) -> glutin::dpi::LogicalPosition;
}

impl PixelDelta for glutin::MouseScrollDelta {
	fn pixel_delta(&self) -> glutin::dpi::LogicalPosition {
		match self {
			glutin::MouseScrollDelta::PixelDelta(delta) => *delta,
			glutin::MouseScrollDelta::LineDelta(x, y)   => glutin::dpi::LogicalPosition::new(*x as f64 * 10.0, *y as f64 * 10.0),
		}
	}
}
