use std::ops::Deref;
use std::slice;

use glium;
use glium::index::Index;

use shader;

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct VertexPosition {
	pub position: [f32;3],
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct VertexPositionNormal {
	pub position: [f32;3],
	pub normal:   [f32;3],
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub struct VertexPositionNormalTexture {
	pub position: [f32;3],
	pub normal:   [f32;3],
	pub texture:  [f32;2],
}

implement_vertex!(VertexPosition,              position);
implement_vertex!(VertexPositionNormal,        position, normal);
implement_vertex!(VertexPositionNormalTexture, position, normal, texture);

pub struct Triangle<I: Index>(pub [I; 3]);
impl<I: Index> Clone for Triangle<I> {
	fn clone(&self) -> Self {
		Triangle(self.0)
	}
}

impl<I: Index> Deref for Triangle<I> {
	type Target = [I; 3];
	fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Clone)]
pub struct TriangleList<T: Index>(pub Vec<Triangle<T>>);

impl<T: Index> Deref for TriangleList<T> {
	type Target = Vec<Triangle<T>>;
	fn deref(&self) -> &Self::Target { &self.0 }
}

pub trait MakeIndexBuffer<I: Index> {
	fn make_index_buffer(&self, facade: &glium::Display) -> Result<glium::index::IndexBuffer<I>, glium::index::BufferCreationError>;
}

impl<I: Index> MakeIndexBuffer<I> for TriangleList<I> {
	fn make_index_buffer(&self, facade: &glium::Display) -> Result<glium::index::IndexBuffer<I>, glium::index::BufferCreationError> {
		let data: &[I] = unsafe { slice::from_raw_parts(self.as_slice().as_ptr() as *const I, self.len() * 3) };
		glium::IndexBuffer::new(facade, glium::index::PrimitiveType::TrianglesList, data)
	}
}

pub struct FacesWithMaterial<I: Index> {
	pub faces: TriangleList<I>,
	pub material: shader::Material,
}

pub struct Mesh<V: glium::Vertex, I: Index> {
	pub vertices: Vec<V>,
	pub polygons: Vec<FacesWithMaterial<I>>,
	//_phantom: PhantomData<&'a bool>,
}
