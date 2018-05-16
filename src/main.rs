#[macro_use]
extern crate glium;
extern crate image;
extern crate nalgebra;
extern crate obj;
extern crate num;
extern crate alga;

#[macro_use]
extern crate derive_error;

pub mod geometry;
pub mod mesh;
pub mod mesh_obj;
pub mod shader;
pub mod teapot;
pub mod texture;

use mesh::{
	IndexBuffer,
	NormalVertices,
	PositionVertices,
	TextureVertices,
};
use geometry::{rotate, VectorEx};

use nalgebra as na;

struct AsUniform<'a, T: 'a>(pub &'a T);

impl<'a> glium::uniforms::AsUniformValue for AsUniform<'a, nalgebra::Matrix4<f32>> {
	fn as_uniform_value(&self) -> glium::uniforms::UniformValue {
		glium::uniforms::UniformValue::Mat4(unsafe { std::mem::transmute(*self.0) })
	}
}


fn main() {
	use glium::glutin;
	use glium::Surface;

	let mut event_loop = glutin::EventsLoop::new();
	let window         = glutin::WindowBuilder::new();
	let context        = glutin::ContextBuilder::new().with_depth_buffer(24);
	let display        = glium::Display::new(window, context, &event_loop).unwrap();

	let params = glium::DrawParameters {
		depth: glium::Depth {
			test: glium::draw_parameters::DepthTest::IfLess,
			write: true,
			.. Default::default()
		},
		.. Default::default()
	};

	let program = shader::program_pnt(&display).unwrap();

	let teapot   = teapot::object();
	let vertices = (
		&teapot.position_vertices(&display).unwrap(),
		&teapot.normal_vertices(&display).unwrap(),
		&teapot.texture_vertices(&display).unwrap(),
	);
	let polygon  = mesh_obj::group_to_polygon::<u16>(&teapot.objects[0].groups[0]).unwrap();
	let indices  = polygon.to_index_buffer(&display).unwrap();

	let mut closed = false;
	let mut time: f32 = 0.0;

	while !closed {
		let transform = na::Similarity::from_parts(na::Translation::identity(), rotate(na::Vector3::new(1., 1., 1.).as_unit(), time), 0.01).to_homogeneous();

		let mut frame = display.draw();
		let uniforms = uniform!{
			transform: AsUniform(&transform),
			light_direction: [-1.0, 0.4, 0.9f32],
		};

		frame.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
		frame.draw(vertices, &indices, &program, &uniforms, &params).unwrap();
		frame.finish().unwrap();

		// listing the events produced by application and waiting to be received
		event_loop.poll_events(|ev| {
			match ev {
				glutin::Event::WindowEvent { event, .. } => match event {
					glutin::WindowEvent::Closed => closed = true,
					_ => (),
				},
				_ => (),
			}
		});

		time += 0.008;
	}
}
