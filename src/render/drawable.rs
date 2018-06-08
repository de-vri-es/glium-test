use glium;

/// Result type for draw operations.
pub type DrawResult = Result<(), glium::DrawError>;

/// A drawable geometry consiting of a vertex buffer and an index buffer.
#[derive(Clone, Copy, Debug)]
pub struct Geometry<'a, V: 'a + glium::Vertex, I: 'a + glium::index::Index> {
	pub vertices: &'a glium::VertexBuffer<V>,
	pub indices:  &'a glium::IndexBuffer<I>,
}

/// Trait to indicate that a shader program can work with vertices and uniforms of the given type.
pub unsafe trait ShaderProgram<V: glium::Vertex, Uniforms: glium::uniforms::Uniforms> {
	fn program(&self) -> &glium::Program;

	fn draw<'a, I: glium::index::Index>(
		&self,
		surface     : &mut impl glium::Surface,
		geometry    : Geometry<'a, V, I>,
		uniforms    : &Uniforms,
		draw_params : &glium::DrawParameters,
	) -> DrawResult {
		surface.draw(geometry.vertices, geometry.indices, self.program(), uniforms, draw_params)
	}
}
