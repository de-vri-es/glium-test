use std::ops::Deref;
use std::slice;

use glium;
use glium::index::Index;

use shader;

/// An index type for triangles.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Triangle<I: Index>(pub [I; 3]);

impl<I: Index> Deref for Triangle<I> {
	type Target = [I; 3];
	fn deref(&self) -> &Self::Target { &self.0 }
}

#[derive(Clone, Debug)]
pub struct TriangleList<T: Index>(pub Vec<Triangle<T>>);

impl<T: Index> Deref for TriangleList<T> {
	type Target = Vec<Triangle<T>>;
	fn deref(&self) -> &Self::Target { &self.0 }
}

pub trait MakeIndexBuffer<I: Index> {
	fn make_index_buffer(&self, facade: &impl glium::backend::Facade) -> Result<glium::index::IndexBuffer<I>, glium::index::BufferCreationError>;
}

impl<I: Index> MakeIndexBuffer<I> for TriangleList<I> {
	fn make_index_buffer(&self, facade: &impl glium::backend::Facade) -> Result<glium::index::IndexBuffer<I>, glium::index::BufferCreationError> {
		let data: &[I] = unsafe { slice::from_raw_parts(self.as_slice().as_ptr() as *const I, self.len() * 3) };
		glium::IndexBuffer::new(facade, glium::index::PrimitiveType::TrianglesList, data)
	}
}

pub struct Mesh<V: glium::Vertex, I: Index> {
	pub vertices: Vec<V>,
	pub polygons: Vec<(TriangleList<I>, shader::Material)>,
}

pub struct RenderableMesh<V: glium::Vertex, I: Index> {
	pub vertices: glium::VertexBuffer<V>,
	pub polygons: Vec<(glium::IndexBuffer<I>, shader::Material)>,
}

#[derive(Clone, Copy, Debug, Error)]
pub enum MeshCreationError {
	Vertex(glium::vertex::BufferCreationError),
	Index(glium::index::BufferCreationError),
}

impl<V: glium::Vertex, I: Index> RenderableMesh<V, I> {
	pub fn from_mesh(facade: &impl glium::backend::Facade, mesh: &Mesh<V, I>) -> Result<Self, MeshCreationError> {
		let vertices = glium::VertexBuffer::new(facade, &mesh.vertices).map_err(MeshCreationError::Vertex)?;

		let mut polygons = Vec::with_capacity(mesh.polygons.len());

		for (indices, material) in &mesh.polygons {
			let indices = indices.make_index_buffer(facade).map_err(MeshCreationError::Index)?;
			polygons.push((indices, *material))
		}

		Ok(RenderableMesh{vertices, polygons})
	}

	pub fn draw_all<'a, Program>(&'a self, surface: &mut impl glium::Surface, params: &glium::DrawParameters, program: &Program, uniforms: &shader::Uniforms<'a>) -> Result<(), glium::DrawError> where
		Program: shader::ShaderProgram<V, I, shader::Uniforms<'a>>,
	{
		for (indices, material) in &self.polygons {
			let uniforms = shader::Uniforms{material: material, ..*uniforms};
			program.draw(surface, params, &self.vertices, indices, &uniforms)?;
		}

		Ok(())
	}
}
