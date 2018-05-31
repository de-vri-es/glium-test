use std;
use glium;
use nalgebra;

use std::slice;

/// A vertex type with only 3D position information.
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct VertexPosition {
	pub position: [f32;3],
}

/// A vertex type with 3D position and normal information.
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct VertexPositionNormal {
	pub position: [f32;3],
	pub normal:   [f32;3],
}

/// A vertex type with 3D position and normal information, and 2D texture coordinates.
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct VertexPositionNormalTexture {
	pub position: [f32;3],
	pub normal:   [f32;3],
	pub texture:  [f32;2],
}

implement_vertex!(VertexPosition,              position);
implement_vertex!(VertexPositionNormal,        position, normal);
implement_vertex!(VertexPositionNormalTexture, position, normal, texture);

/// An index type for triangles.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Triangle<I: glium::index::Index>(pub [I; 3]);

/// Deref Triangle<I> into [I; 3].
impl<I: glium::index::Index> std::ops::Deref for Triangle<I> {
	type Target = [I; 3];
	fn deref(&self) -> &Self::Target { &self.0 }
}

/// A list of triangles that can be turned into an index buffer.
#[derive(Clone, Debug)]
pub struct TriangleList<I: glium::index::Index>(pub Vec<Triangle<I>>);

/// Deref TriangleList<I> into Vec<Triangle<I>>
impl<I: glium::index::Index> std::ops::Deref for TriangleList<I> {
	type Target = Vec<Triangle<I>>;
	fn deref(&self) -> &Self::Target { &self.0 }
}

impl<I: glium::index::Index> TriangleList<I> {
	/// Upload the triangle list to the GPU as an index buffer.
	fn make_index_buffer(&self, facade: &impl glium::backend::Facade) -> Result<glium::index::IndexBuffer<I>, glium::index::BufferCreationError> {
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

impl<V: glium::Vertex, I: glium::index::Index, P: Clone> Mesh<V, I, P> {
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
}

/// An object with a transformation applied.
pub struct TransformedObject<Object> {
	pub object    : Object,
	pub transform : nalgebra::Matrix4<f32>,
}

impl<O> TransformedObject<O> {
	/// Create a transformed object from an object and a transformation.
	pub fn new(object: O, transform: nalgebra::Matrix4<f32>) -> Self { Self{object, transform} }
}
