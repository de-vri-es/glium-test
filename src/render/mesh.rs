use std;
use glium;
use nalgebra as na;

use std::slice;
use num::NumCast;

/// A vertex type with only 3D position information.
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
#[repr(C)]
pub struct VertexPosition {
	pub position: na::Point3<f32>,
}

/// A vertex type with 3D position and normal information.
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
#[repr(C)]
pub struct VertexPositionNormal {
	pub position: na::Point3<f32>,
	pub normal:   na::Vector3<f32>,
}

/// A vertex type with 3D position and normal information, and 2D texture coordinates.
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
#[repr(C)]
pub struct VertexPositionNormalTexture {
	pub position: na::Point3<f32>,
	pub normal:   na::Vector3<f32>,
	pub texture:  na::Point2<f32>,
}

impl glium::Vertex for VertexPosition {
	fn build_bindings() -> glium::vertex::VertexFormat {
		use std::borrow::Cow::Borrowed;
		use glium::vertex::AttributeType;
		Borrowed(&[
			(Borrowed("position"), 0, AttributeType::F32F32F32, false),
		])
	}
}

impl glium::Vertex for VertexPositionNormal {
	fn build_bindings() -> glium::vertex::VertexFormat {
		use std::borrow::Cow::Borrowed;
		use glium::vertex::AttributeType;
		Borrowed(&[
			(Borrowed("position"),  0, AttributeType::F32F32F32, false),
			(Borrowed("normal"),   12, AttributeType::F32F32F32, false),
		])
	}
}

impl glium::Vertex for VertexPositionNormalTexture {
	fn build_bindings() -> glium::vertex::VertexFormat {
		use std::borrow::Cow::Borrowed;
		use glium::vertex::AttributeType;
		Borrowed(&[
			(Borrowed("position"),  0, AttributeType::F32F32F32, false),
			(Borrowed("normal"),   12, AttributeType::F32F32F32, false),
			(Borrowed("texture"),  24, AttributeType::F32F32,    false),
		])
	}
}

/// An index type for triangles.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Triangle<I>(pub [I; 3]);

impl<I: Copy> Triangle<I> {
	pub fn map<U>(self, func: impl Fn(I) -> U) -> Triangle<U> {
		Triangle([func(self[0]), func(self[1]), func(self[2])])
	}
}

/// Deref Triangle<I> into [I; 3].
impl<I> std::ops::Deref for Triangle<I> {
	type Target = [I; 3];
	fn deref(&self) -> &Self::Target { &self.0 }
}

impl<I> std::ops::DerefMut for Triangle<I> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

/// A list of triangles that can be turned into an index buffer.
#[derive(Clone, Debug)]
pub struct TriangleList<I: glium::index::Index>(pub Vec<Triangle<I>>);

/// Deref TriangleList<I> into Vec<Triangle<I>>
impl<I: glium::index::Index> std::ops::Deref for TriangleList<I> {
	type Target = Vec<Triangle<I>>;
	fn deref(&self) -> &Self::Target { &self.0 }
}

impl<I: glium::index::Index> std::ops::DerefMut for TriangleList<I> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<I: glium::index::Index> TriangleList<I> {
	/// Upload the triangle list to the GPU as an index buffer.
	pub fn make_index_buffer(&self, facade: &impl glium::backend::Facade) -> Result<glium::index::IndexBuffer<I>, glium::index::BufferCreationError> {
		let data: &[I] = unsafe { slice::from_raw_parts(self.as_slice().as_ptr() as *const I, self.len() * 3) };
		glium::IndexBuffer::new(facade, glium::index::PrimitiveType::TrianglesList, data)
	}
}

/// A simple mesh structure with one set of vertices and a vector of polygons with customizable properties.
#[derive(Clone, Debug)]
pub struct Mesh<V: glium::Vertex, I: glium::index::Index, P: Clone> {
	pub vertices: Vec<V>,
	pub polygons: Vec<(TriangleList<I>, P)>,
}

/// A simple mesh that with one vertex buffers and a vector of index buffers with customizable properties.
#[derive(Debug)]
pub struct RenderableMesh<V: glium::Vertex, I: glium::index::Index, P: Clone> {
	pub vertices: glium::VertexBuffer<V>,
	pub polygons: Vec<(glium::IndexBuffer<I>, P)>,
}

#[derive(Clone, Copy, Debug, Error)]
pub enum MeshCreationError {
	Vertex(glium::vertex::BufferCreationError),
	Index(glium::index::BufferCreationError),
}

impl<V, I, P> Mesh<V, I, P> where
	V: glium::Vertex,
	I: glium::index::Index + NumCast + std::ops::Add<Output=I>,
	P: Clone,
{
	/// Upload the mesh to the GPU.
	pub fn upload(&self, facade: &impl glium::backend::Facade) -> Result<RenderableMesh<V, I, P>, MeshCreationError> {
		let vertices = glium::VertexBuffer::new(facade, &self.vertices).map_err(MeshCreationError::Vertex)?;

		let mut polygons = Vec::with_capacity(self.polygons.len());

		for (indices, material) in &self.polygons {
			let indices = indices.make_index_buffer(facade).map_err(MeshCreationError::Index)?;
			polygons.push((indices, material.clone()))
		}

		Ok(RenderableMesh{vertices, polygons})
	}

	/// Append a single polygon with it's own vertices to the mesh.
	pub fn append_consume_polygon(&mut self, vertices: &Vec<V>, mut polygon: TriangleList<I>, properties: P) {
		let extra = I::from(self.vertices.len()).unwrap();
		for face in polygon.iter_mut() {
			*face = face.map(|x| x + extra);
		}
		self.vertices.extend(vertices);
		self.polygons.push((polygon, properties));
	}

	/// Append another mesh to this one.
	pub fn append(&mut self, other: &Self) {
		let extra = I::from(self.vertices.len()).unwrap();

		self.vertices.extend(&other.vertices);
		self.polygons.reserve(other.polygons.len());

		for (faces, properties) in &other.polygons {
			let faces = faces.iter().map(|face| Triangle([face[0] + extra, face[1] + extra, face[2] + extra])).collect();
			self.polygons.push((TriangleList(faces), properties.clone()));
		}

	}

	/// Append another mesh to this one, destroying the other mesh.
	pub fn append_consume(&mut self, mut other: Self) {
		let extra = I::from(self.vertices.len()).unwrap();

		for (faces, _) in other.polygons.iter_mut() {
			for face in faces.iter_mut() { *face = Triangle([face[0] + extra, face[1] + extra, face[2] + extra]) }
		}

		self.vertices.append(&mut other.vertices);
		self.polygons.append(&mut other.polygons);
	}
}

/// An object with a transformation applied.
#[derive(Copy, Clone, Debug)]
pub struct TransformedObject<Object> {
	pub object    : Object,
	pub transform : na::Transform3<f32>,
}

impl<O> TransformedObject<O> {
	/// Create a transformed object from an object and a transformation.
	pub fn new(object: O, transform: na::Transform3<f32>) -> Self { Self{object, transform} }
}
