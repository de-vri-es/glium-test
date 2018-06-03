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
	fn draw(self, surface: &mut impl glium::Surface, draw_params: &glium::DrawParameters, args: ExtraArgs) -> DrawResult;
}

impl<'a, P, V, I, U> Drawable<()> for (&'a P, &'a glium::VertexBuffer<V>, &'a glium::IndexBuffer<I>, &'a U)
where
	P: ShaderProgram<V, U>,
	V: glium::Vertex,
	I: glium::index::Index,
	U: glium::uniforms::Uniforms,
{
	fn draw(self, surface: &mut impl glium::Surface, draw_params: &glium::DrawParameters, _extra: ()) -> DrawResult {
		let (program, vertices, indices, uniforms) = self;
		program.draw(surface, vertices, indices, uniforms, draw_params)
	}
}

impl<'a, P, V, I, U> Drawable<&'a U> for (&'a P, &'a glium::VertexBuffer<V>, &'a glium::IndexBuffer<I>)
where
	P: ShaderProgram<V, U>,
	V: glium::Vertex,
	I: glium::index::Index,
	U: glium::uniforms::Uniforms,
{
	fn draw(self, surface: &mut impl glium::Surface, draw_params: &glium::DrawParameters, uniforms: &'a U) -> DrawResult {
		let (program, vertices, indices) = self;
		program.draw(surface, vertices, indices, uniforms, draw_params)
	}
}

//impl<ExtraArgs, T> Drawable<ExtraArgs> for T
//where
	//ExtraArgs: Copy,
	//for <'a> &'a T: IntoIterator,
	//for <'a> <&'a T as IntoIterator>::Item: Drawable<ExtraArgs>,
//{
	//fn draw(&self, surface: &mut impl glium::Surface, draw_params: &glium::DrawParameters, extra: ExtraArgs) -> DrawResult {
		//for drawable in self.into_iter() {
			//drawable.draw(surface, draw_params, extra)?;
		//}
		//Ok(())
	//}
//}
