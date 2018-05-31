use glium;

/// Result type for draw operations.
pub type DrawResult = Result<(), glium::DrawError>;

/// Trait to indicate that a shader program can work with vertices and uniforms of the given type.
pub unsafe trait ShaderProgram<V: glium::Vertex, Uniforms: glium::uniforms::Uniforms> {
	fn program(&self) -> &glium::Program;

	fn draw<I: glium::index::Index>(
		&self,
		surface     : &mut impl glium::Surface,
		vertices    : &glium::VertexBuffer<V>,
		indices     : &glium::IndexBuffer<I>,
		uniforms    : &Uniforms,
		draw_params : &glium::DrawParameters,
	) -> DrawResult {
		surface.draw(vertices, indices, self.program(), uniforms, draw_params)
	}
}

/// Trait for things that can be drawn to a surface.
pub trait Drawable<ExtraArgs> {
	fn draw(&self, surface: &mut impl glium::Surface, draw_params: &glium::DrawParameters, args: ExtraArgs) -> DrawResult;
}
