extern crate alga;
extern crate genmesh;
extern crate glium;
extern crate image;
extern crate nalgebra;
extern crate num;
extern crate obj;

#[macro_use]
extern crate derive_error;

pub mod render;
pub mod geometry;
pub mod shapes;

use geometry::{rotate, VectorEx};

use nalgebra as na;

fn main() {
	use glium::Surface;
	use render::{
		Drawable,
		simple_shader::{
			Material,
			Uniforms,
		}
	};
	use geometry::rotate_z;

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
		backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
		multisampling: true,
		smooth: Some(glium::draw_parameters::Smooth::Nicest),
		.. Default::default()
	};

	let program = render::simple_shader::program(&display).unwrap();
	//let monkey : RenderableMesh<VertexPositionNormal, u16, Material> = render::wavefront::load_file("src/assets/monkey.obj", true).unwrap().upload(&display).unwrap();

	let red   = Material{diffuse: [1., 0., 0.], ..Default::default()};
	let green = Material{diffuse: [0., 1., 0.], ..Default::default()};

	let drone_body = {
		let (vertices, indices) = shapes::drone_body();
		(&glium::VertexBuffer::new(&display, &vertices).unwrap(), &indices.make_index_buffer(&display).unwrap(), &red)
	};

	let drone_rotor = {
		let (vertices, indices) = shapes::drone_rotor();
		(&glium::VertexBuffer::new(&display, &vertices).unwrap(), &indices.make_index_buffer(&display).unwrap(), &green)
	};

	const PI : f32 = std::f32::consts::PI;
	let rotors = [
		rotate_z(0.0 * PI / 3.0) * na::Vector3::new(0.65, 0., 0.),
		rotate_z(1.0 * PI / 3.0) * na::Vector3::new(0.65, 0., 0.),
		rotate_z(2.0 * PI / 3.0) * na::Vector3::new(0.65, 0., 0.),
		rotate_z(3.0 * PI / 3.0) * na::Vector3::new(0.65, 0., 0.),
		rotate_z(4.0 * PI / 3.0) * na::Vector3::new(0.65, 0., 0.),
		rotate_z(5.0 * PI / 3.0) * na::Vector3::new(0.65, 0., 0.),
	];

	let drone = shapes::DroneParts::build(drone_body, drone_rotor, &rotors);

	let mut closed = false;
	let mut time: f32 = 0.0;

	while !closed {
		let transform = na::Transform3::identity() * rotate(na::Vector3::new(1., 1., 1.).as_unit(), time);

		let mut frame = display.draw();
		let uniforms = Uniforms{
			transform: &transform.into(),
			light_direction: &na::Vector3::new(-1.0, 0.4, 0.9).as_unit(),
			material: &Default::default(),
		};

		frame.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

		//drone.body.object.draw(&mut frame, &params, (&program, &uniforms));
		drone.draw(&mut frame, &params, (&program, &uniforms)).unwrap();
		frame.finish().unwrap();

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
