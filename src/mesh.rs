use std::ops::Deref;
use std;

use glium;

use glium::index::{Index};

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
pub struct Polygon<T: Index>(pub Vec<Triangle<T>>);

impl<T: Index> Deref for Polygon<T> {
	type Target = Vec<Triangle<T>>;
	fn deref(&self) -> &Self::Target { &self.0 }
}

/// Allow extracting of vertices from a buffer by an index.
/**
 * The buffer an index may be arbitrarily complex types,
 * like tuples of buffers and indices.
 */
pub trait ExtractVertex<Buffer, Index>: Sized {
	fn extract(buffer: &Buffer, index: &Index) -> Result<Self, String>;
}

pub trait VertexBuffer<V: glium::Vertex> {
	fn vertex_buffer(self, display: &glium::Display) -> Result<glium::vertex::VertexBufferAny, glium::vertex::BufferCreationError>;
}

pub trait IndexBuffer {
	fn index_buffer(&self, facade: &glium::Display) -> Result<glium::index::IndexBufferAny, glium::index::BufferCreationError>;
}

impl<I: Index> IndexBuffer for Polygon<I> {
	fn index_buffer(&self, facade: &glium::Display) -> Result<glium::index::IndexBufferAny, glium::index::BufferCreationError> {
		let data: &[I] = unsafe { std::slice::from_raw_parts(self.as_slice().as_ptr() as *const I, self.len() * 3) };
		Ok(glium::IndexBuffer::new(facade, glium::index::PrimitiveType::TrianglesList, data)?.into())
	}
}

pub struct PositionBuffer<'a> {
	pub positions: &'a Vec<[f32;3]>,
}

pub struct PositionNormalBuffer<'a> {
	pub positions: &'a Vec<[f32;3]>,
	pub normals:   &'a Vec<[f32;3]>,
}

pub struct PositionNormalTextureBuffer<'a> {
	pub positions:         &'a Vec<[f32;3]>,
	pub normals:           &'a Vec<[f32;3]>,
	pub texture_positions: &'a Vec<[f32;2]>,
}

pub trait GetPositions {
	fn get_positions(&self) -> &[[f32; 3]];
}

pub trait GetNormals {
	fn get_normals(&self) -> &[[f32; 3]];
}

pub trait GetTexturePositions {
	fn get_texture_positions(&self) -> &[[f32; 2]];
}

impl<'a> GetPositions for PositionBuffer<'a> {
	fn get_positions(&self) -> &[[f32;3]] { self.positions }
}
impl<'a> GetPositions for PositionNormalBuffer<'a> {
	fn get_positions(&self) -> &[[f32;3]] { self.positions }
}
impl<'a> GetPositions for PositionNormalTextureBuffer<'a> {
	fn get_positions(&self) -> &[[f32;3]] { self.positions }
}

impl<'a> GetNormals for PositionNormalBuffer<'a> {
	fn get_normals(&self) -> &[[f32;3]] { self.normals }
}
impl<'a> GetNormals for PositionNormalTextureBuffer<'a> {
	fn get_normals(&self) -> &[[f32;3]] { self.normals }
}

impl<'a> GetTexturePositions for PositionNormalTextureBuffer<'a> {
	fn get_texture_positions(&self) -> &[[f32;2]] { self.texture_positions }
}
