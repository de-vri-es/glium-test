#[macro_use]
extern crate glium;
extern crate image;
extern crate nalgebra;
extern crate obj;
extern crate num;
extern crate alga;

#[macro_use]
extern crate derive_error;

pub mod render;
pub mod geometry;

use geometry::{rotate, VectorEx};

use nalgebra as na;

fn main() {
	use glium::Surface;
	use render::{
		RenderableMesh,
		VertexPositionNormal,
		Drawable,
		simple_shader::{
			Material,
			Uniforms,
		}
	};

	let mut event_loop = glium::glutin::EventsLoop::new();
	let window         = glium::glutin::WindowBuilder::new();
	let context        = glium::glutin::ContextBuilder::new().with_depth_buffer(24);
	let display        = glium::Display::new(window, context, &event_loop).unwrap();

	let params = glium::DrawParameters {
		depth: glium::Depth {
			test: glium::draw_parameters::DepthTest::IfLess,
			write: true,
			.. Default::default()
		},
		.. Default::default()
	};

	let program = render::simple_shader::program(&display).unwrap();
	let monkey : RenderableMesh<VertexPositionNormal, u16, Material> = render::wavefront::load_file("src/assets/monkey.obj", true).unwrap().upload(&display).unwrap();

	let mut closed = false;
	let mut time: f32 = 0.0;

	while !closed {
		let transform = na::Similarity::from_parts(na::Translation::identity(), rotate(na::Vector3::new(1., 1., 1.).as_unit(), time), 1.).to_homogeneous();

		let mut frame = display.draw();
		let uniforms = Uniforms{
			transform: &transform,
			light_direction: &na::Vector3::new(-1.0, 0.4, 0.9).as_unit(),
			material: &Default::default(),
		};

		frame.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

		monkey.draw(&mut frame, &params, (&program, &uniforms)).unwrap();
		frame.finish().unwrap();

		// listing the events produced by application and waiting to be received
		event_loop.poll_events(|ev| {
			match ev {
				glium::glutin::Event::WindowEvent { event, .. } => match event {
					glium::glutin::WindowEvent::Closed => closed = true,
					_ => (),
				},
				_ => (),
			}
		});

		time += 0.008;
	}
}
