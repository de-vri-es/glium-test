extern crate alga;
extern crate genmesh;
extern crate glium;
extern crate glutin;
extern crate image;
extern crate nalgebra;
extern crate num;
extern crate obj;

pub mod geometry;
pub mod render;
pub mod shapes;
pub mod viewer;

use geometry::{rotate_y, VectorEx};

use nalgebra as na;

fn main() {
	use glium::Surface;
	use render::{
		Geometry,
		SetMaterial,
		simple_shader::{
			Material,
			Uniforms,
		}
	};

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
		backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
		multisampling: true,
		smooth: Some(glium::draw_parameters::Smooth::Nicest),
		.. Default::default()
	};

	let program = render::simple_shader::program(&display).unwrap();

	let red   = Material{diffuse: [1., 0., 0.], ..Default::default()};
	let green = Material{diffuse: [0., 1., 0.], ..Default::default()};

	let body = {
		let (vertices, indices) = shapes::drone_body();
		render::Adjusted{data: Geometry{
			vertices: &glium::VertexBuffer::new(&display, &vertices).unwrap(),
			indices:  &indices.make_index_buffer(&display).unwrap(),
		}, adjust: SetMaterial(red)}
	};

	let rotor = {
		let (vertices, indices) = shapes::drone_rotor();
		render::Adjusted{data: Geometry{
			vertices: &glium::VertexBuffer::new(&display, &vertices).unwrap(),
			indices:  &indices.make_index_buffer(&display).unwrap(),
		}, adjust: SetMaterial(green)}
	};

	const PI : f32 = std::f32::consts::PI;
	let rotors = [
		rotate_y(0.0 * PI / 3.0) * na::Vector3::new(0.65, 0., 0.),
		rotate_y(1.0 * PI / 3.0) * na::Vector3::new(0.65, 0., 0.),
		rotate_y(2.0 * PI / 3.0) * na::Vector3::new(0.65, 0., 0.),
		rotate_y(3.0 * PI / 3.0) * na::Vector3::new(0.65, 0., 0.),
		rotate_y(4.0 * PI / 3.0) * na::Vector3::new(0.65, 0., 0.),
		rotate_y(5.0 * PI / 3.0) * na::Vector3::new(0.65, 0., 0.),
	];

	let drone = shapes::DroneParts::build(body, rotor, &rotors);

	let mut viewer = viewer::Viewer::new(
		display.gl_window().get_inner_size().unwrap_or(glutin::dpi::LogicalSize::new(1.0, 1.0)),
		0.5 * std::f64::consts::PI,
		0.01,
		1024.0,
	);

	while !viewer.close_requested() {
		let mut frame = display.draw();
		let uniforms = Uniforms{
			time: 0.0,
			perspective: na::convert(*viewer.perspective()),
			transform:   na::convert(*viewer.camera()),
			light_direction: na::Vector3::new(0., 0., 1.).as_unit(),
			material: Default::default(),
		};

		frame.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

		drone.draw(&mut frame, &program, &uniforms, &params).unwrap();
		frame.finish().unwrap();

		event_loop.poll_events(|event| {
			viewer.process_event(&event);
		});
	}
}
