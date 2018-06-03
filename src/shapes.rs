//use std::ops::Mul;
//use num::{NumCast,ToPrimitive};

use glium;
use nalgebra as na;

use render::{
	DrawResult,
	Drawable,
	Mesh,
	VertexPositionNormal,
	//Triangle,
	TriangleList,
	TransformedObject,
	wavefront,
};

use geometry::{
	translate,
	//rotate_z,
};

//#[derive(Clone, Copy, Debug)]
//struct Id;

//impl From<Id> for na::Transform3<f32> {
	//fn from(_: Id) -> Self {
		//Self::identity()
	//}
//}

//impl<T> Mul<T> for Id {
	//type Output = T;
	//fn mul(self, other: T) -> T { other }
//}

//fn index<Index: NumCast, In: ToPrimitive>(input: In) -> Index {
	//Index::from(input).unwrap()
//}

//fn vec3<T: na::Scalar>(x: T, y: T, z: T) -> na::Vector3<T> {
	//na::Vector3::new(x, y, z)
//}

//fn point3<T: na::Scalar>(x: T, y: T, z: T) -> na::Point3<T> {
	//na::Point3::new(x, y, z)
//}

//fn cylinder<Index, Material>(
	//segments:  usize,
	//radius:    f32,
	//height:    f32,
	//material:  Material,
	//transform: impl Into<na::Transform3<f32>>
//) -> Mesh<VertexPositionNormal, Index, Material> where
	//Index: glium::index::Index + NumCast + ::std::fmt::Debug,
	//Material: Clone,
//{
	//const TAU: f32 = 2.0 * ::std::f32::consts::PI;

	//let segments = ::std::cmp::max(2, segments);

	//let mut vertices  = Vec::with_capacity(segments * 4);
	//let mut triangles = Vec::with_capacity(segments * 4 - 4);

	//let transform = transform.into();
	//let rotation = |i| rotate_z(i as f32 * TAU / segments as f32);

	//// Bottom cap vertices
	//for i in 0..segments {
		//let position = transform * rotation(i) * point3(radius, 0.0, -0.5 * height);
		//let normal   = transform * -vec3(0., 0., 1.);
		//vertices.push(VertexPositionNormal{position, normal});
	//}

	//// Top cap vertices
	//for i in 0..segments {
		//let position = transform * rotation(i) * point3(radius, 0.0, 0.5 * height);
		//let normal   = transform * vec3(0., 0., 1.);
		//vertices.push(VertexPositionNormal{position, normal});
	//}

	//// Side vertices
	//for i in 0..segments {
		//let transform       = transform * rotation(i);
		//let position_bottom = transform * point3(radius, 0.0, -0.5 * height);
		//let position_top    = transform * point3(radius, 0.0,  0.5 * height);
		//let normal          = transform * vec3(0., 1., 0.);
		//vertices.push(VertexPositionNormal{position: position_bottom, normal});
		//vertices.push(VertexPositionNormal{position: position_top,    normal});
	//}

	//// Bottom cap indices
	//for i in 2..segments {
		//triangles.push(Triangle([0, i - 1, i]).map(index))
	//}
	//// Top cap indices
	//for i in 2..segments {
		//triangles.push(Triangle([segments + i, segments + i - 1, segments + 0]).map(index))
	//}

	//// Side indices
	//for i in 0..segments {
		//triangles.push(Triangle([0, 1, 2]).map(|x| index(2 * segments + (2 * i + x) % (2 * segments))));
		//triangles.push(Triangle([3, 2, 1]).map(|x| index(2 * segments + (2 * i + x) % (2 * segments))));
	//}

	//Mesh{vertices, polygons: vec![(TriangleList(triangles), material)]}
//}

//fn beam<Index, Material>(size: [f32; 3], material:  Material, transform: impl Into<na::Transform3<f32>>)
	//-> Mesh<VertexPositionNormal, Index, Material>
//where
	//Index: glium::index::Index + NumCast,
	//Material: Clone,
//{
	//let transform = transform.into();
	//let vertices  = vec![
		//// Bottom plane.
		//VertexPositionNormal{position: transform * point3(-size[0], -size[1], -size[2]), normal: transform * vec3(0., 0., -1.)},
		//VertexPositionNormal{position: transform * point3(-size[0],  size[1], -size[2]), normal: transform * vec3(0., 0., -1.)},
		//VertexPositionNormal{position: transform * point3( size[0], -size[1], -size[2]), normal: transform * vec3(0., 0., -1.)},
		//VertexPositionNormal{position: transform * point3( size[0],  size[1], -size[2]), normal: transform * vec3(0., 0., -1.)},
		//// Top plane.
		//VertexPositionNormal{position: transform * point3(-size[0], -size[1],  size[2]), normal: transform * vec3(0., 0.,  1.)},
		//VertexPositionNormal{position: transform * point3( size[0], -size[1],  size[2]), normal: transform * vec3(0., 0.,  1.)},
		//VertexPositionNormal{position: transform * point3(-size[0],  size[1],  size[2]), normal: transform * vec3(0., 0.,  1.)},
		//VertexPositionNormal{position: transform * point3( size[0],  size[1],  size[2]), normal: transform * vec3(0., 0.,  1.)},
		//// Left plane.
		//VertexPositionNormal{position: transform * point3(-size[0], -size[1], -size[2]), normal: transform * vec3(-1., 0., 0.)},
		//VertexPositionNormal{position: transform * point3(-size[0], -size[1],  size[2]), normal: transform * vec3(-1., 0., 0.)},
		//VertexPositionNormal{position: transform * point3(-size[0],  size[1], -size[2]), normal: transform * vec3(-1., 0., 0.)},
		//VertexPositionNormal{position: transform * point3(-size[0],  size[1],  size[2]), normal: transform * vec3(-1., 0., 0.)},
		//// Right plane.
		//VertexPositionNormal{position: transform * point3( size[0], -size[1], -size[2]), normal: transform * vec3( 1., 0., 0.)},
		//VertexPositionNormal{position: transform * point3( size[0],  size[1], -size[2]), normal: transform * vec3( 1., 0., 0.)},
		//VertexPositionNormal{position: transform * point3( size[0], -size[1],  size[2]), normal: transform * vec3( 1., 0., 0.)},
		//VertexPositionNormal{position: transform * point3( size[0],  size[1],  size[2]), normal: transform * vec3( 1., 0., 0.)},
		//// Front plane.
		//VertexPositionNormal{position: transform * point3(-size[0], -size[1], -size[2]), normal: transform * vec3(1., -1., 0.)},
		//VertexPositionNormal{position: transform * point3( size[0], -size[1], -size[2]), normal: transform * vec3(1., -1., 0.)},
		//VertexPositionNormal{position: transform * point3(-size[0], -size[1],  size[2]), normal: transform * vec3(1., -1., 0.)},
		//VertexPositionNormal{position: transform * point3( size[0], -size[1],  size[2]), normal: transform * vec3(1., -1., 0.)},
		//// Back plane.
		//VertexPositionNormal{position: transform * point3(-size[0],  size[1], -size[2]), normal: transform * vec3(1.,  1., 0.)},
		//VertexPositionNormal{position: transform * point3(-size[0],  size[1],  size[2]), normal: transform * vec3(1.,  1., 0.)},
		//VertexPositionNormal{position: transform * point3( size[0],  size[1], -size[2]), normal: transform * vec3(1.,  1., 0.)},
		//VertexPositionNormal{position: transform * point3( size[0],  size[1],  size[2]), normal: transform * vec3(1.,  1., 0.)},
	//];

	//let triangles = vec![
		//Triangle([ 2,  1,  0]).map(index), Triangle([ 2,  3,  1]).map(index),
		//Triangle([ 6,  5,  4]).map(index), Triangle([ 6,  7,  5]).map(index),
		//Triangle([10,  9,  8]).map(index), Triangle([10, 11,  9]).map(index),
		//Triangle([14, 13, 12]).map(index), Triangle([14, 15, 13]).map(index),
		//Triangle([18, 17, 16]).map(index), Triangle([18, 19, 17]).map(index),
		//Triangle([22, 21, 20]).map(index), Triangle([22, 23, 21]).map(index),
	//];

	//Mesh{vertices, polygons: vec![(TriangleList(triangles), material)]}
//}

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
	pub body:   TransformedObject<Part>,
	pub rotors: Vec<TransformedObject<Part>>,
}

impl<Part: Copy> DroneParts<Part> {
	pub fn build(body_part: Part, rotor_part: Part, rotor_positions: &[na::Vector3<f32>]) -> Self {
		Self{
			body: TransformedObject{
				object: body_part,
				transform: na::Transform3::identity()
			},
			rotors: rotor_positions.iter().map(|&pos| TransformedObject{
				object: rotor_part,
				transform: translate(pos) * na::Transform3::identity()
			}).collect(),
		}
	}
}

impl<'a, Part, Extra> Drawable<Extra> for &'a DroneParts<Part>
where
	Part: Copy,
	Extra: Copy,
	for<'b> &'b TransformedObject<Part>: Drawable<Extra>,
{
	fn draw(self, surface: &mut impl glium::Surface, draw_params: &glium::DrawParameters, extra: Extra) -> DrawResult {
		self.body.draw(surface, draw_params, extra)?;
		for rotor in &self.rotors {
			rotor.draw(surface, draw_params, extra)?;
		}
		Ok(())
	}
}
