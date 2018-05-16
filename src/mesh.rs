use std::borrow::Cow::Borrowed;
use std::ops::Deref;
use std;

use glium;

use glium::index::{Index};

#[repr(C, packed)]
pub struct Triangle<T: Index>(pub [T; 3]);
impl<T: Index> Clone for Triangle<T> {
	fn clone(&self) -> Self {
		Triangle(self.0)
	}
}

#[derive(Clone)]
pub struct Polygon<T: Index>(pub Vec<Triangle<T>>);

impl<T: Index> Deref for Polygon<T> {
	type Target = Vec<Triangle<T>>;
	fn deref(&self) -> &Self::Target { &self.0 }
}

pub trait PositionVertices {
	fn position_vertices(&self, display: &glium::Display) -> Result<glium::vertex::VertexBufferAny, glium::vertex::BufferCreationError>;
}

pub trait NormalVertices {
	fn normal_vertices(&self, display: &glium::Display) -> Result<glium::vertex::VertexBufferAny, glium::vertex::BufferCreationError>;
}

pub trait TextureVertices {
	fn texture_vertices(&self, display: &glium::Display) -> Result<glium::vertex::VertexBufferAny, glium::vertex::BufferCreationError>;
}

pub const POSITION_VERTEX_FORMAT: glium::VertexFormat = Borrowed(&[(Borrowed("position"), 0, glium::vertex::AttributeType::F32F32F32, false)]);
pub const NORMAL_VERTEX_FORMAT:   glium::VertexFormat = Borrowed(&[(Borrowed("normal"),   0, glium::vertex::AttributeType::F32F32F32, false)]);
pub const TEXTURE_VERTEX_FORMAT:  glium::VertexFormat = Borrowed(&[(Borrowed("texture"),  0, glium::vertex::AttributeType::F32F32,    false)]);

pub trait IndexBuffer {
	fn to_index_buffer(&self, facade: &glium::Display) -> Result<glium::index::IndexBufferAny, glium::index::BufferCreationError>;
}

impl<I: Index> IndexBuffer for Polygon<I> {
	fn to_index_buffer(&self, facade: &glium::Display) -> Result<glium::index::IndexBufferAny, glium::index::BufferCreationError> {
		let data: &[I] = unsafe { std::slice::from_raw_parts(self.as_slice().as_ptr() as *const I, self.len() * 3) };
		Ok(glium::IndexBuffer::new(facade, glium::index::PrimitiveType::TrianglesList, data)?.into())
	}
}
