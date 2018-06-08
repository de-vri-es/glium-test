//use std::ops::Mul;
//use num::{NumCast,ToPrimitive};

use glium;
use nalgebra as na;

use render::{
	DrawResult,
	Adjusted,
	Geometry,
	Chain,
	ShaderProgram,
	Adjust,
	Mesh,
	TriangleList,
	VertexPositionNormal,
	RightTransformGeometry,
	wavefront,
};

use geometry::{
	translate,
};

fn load_merge(bytes: &'static [u8]) -> (Vec<VertexPositionNormal>, TriangleList<u16>) {
	// Load the mesh.
	let mut bytes = bytes;
	let Mesh{vertices, polygons} = wavefront::load(&mut bytes).unwrap();

	// Flatten everything into a single polygon.
	let polygons = TriangleList(polygons.into_iter().flat_map(|(triangles, _)| triangles.0).collect());

	(vertices, polygons)
}

const DRONE_BODY_BYTES:  &[u8] = include_bytes!("./assets/drone_body.obj");
const DRONE_ROTOR_BYTES: &[u8] = include_bytes!("./assets/drone_rotor.obj");

pub fn drone_body() -> (Vec<VertexPositionNormal>, TriangleList<u16>) {
	load_merge(DRONE_BODY_BYTES)
}

pub fn drone_rotor() -> (Vec<VertexPositionNormal>, TriangleList<u16>) {
	load_merge(DRONE_ROTOR_BYTES)
}

#[derive(Clone, Debug)]
pub struct DroneParts<Part> where
{
	pub body:   Part,
	pub rotors: Vec<Part>,
}

impl<'a, V, I, A> DroneParts<Adjusted<Geometry<'a, V, I>, Chain<RightTransformGeometry, A>>>
where
	A: Copy,
	V: glium::Vertex,
	I: glium::index::Index,
{
	pub fn build(
		body:  Adjusted<Geometry<'a, V, I>, A>,
		rotor: Adjusted<Geometry<'a, V, I>, A>,
		rotor_positions: &[na::Vector3<f32>]
	) -> Self {
		Self{
			body:   body.chain_left(RightTransformGeometry(na::Transform3::identity())),
			rotors: rotor_positions.iter().map(|&pos| rotor.chain_left(RightTransformGeometry(translate(pos) * na::Transform3::identity()))).collect(),
		}
	}
}

impl<'a, V, I, A> DroneParts<Adjusted<Geometry<'a, V, I>, A>>
where
	V: glium::Vertex,
	I: glium::index::Index,
{
	pub fn draw<P, U1>(&self, surface: &mut impl glium::Surface, program: &P, uniforms: U1, draw_params: &glium::DrawParameters) -> DrawResult
	where
		U1: Copy,
		A: Adjust<U1>,
		A::Output: glium::uniforms::Uniforms,
		P: ShaderProgram<V, A::Output>,
	{
		program.draw(surface, self.body.data, &self.body.adjust.adjust(uniforms), draw_params)?;
		for rotor in &self.rotors {
			program.draw(surface, rotor.data, &rotor.adjust.adjust(uniforms), draw_params)?;
		}
		Ok(())
	}
}
