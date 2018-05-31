use glium;
use glium::uniforms::AsUniformValue;

use nalgebra;
use std::mem::transmute;

use super::{
	DrawResult,
	Drawable,
	RenderableMesh,
	ShaderProgram,
	TransformedObject,
	VertexPositionNormal,
	VertexPositionNormalTexture,
};

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Material {
	pub diffuse:  [f32; 3],
	pub specular: [f32; 3],
	pub opacity:  f32,
}

impl Default for Material {
	fn default() -> Self {
		Self {
			diffuse:  [1.0, 1.0, 1.0],
			specular: [0.0, 0.0, 0.0],
			opacity:  1.0,
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Uniforms<'a> {
	pub light_direction: &'a nalgebra::Unit<nalgebra::Vector3<f32>>,
	pub transform:       &'a nalgebra::Matrix4<f32>,
	pub material:        &'a Material,
}

trait AsUniform<'a> {
	fn as_uniform_value(self) -> glium::uniforms::UniformValue<'a>;
}

impl<'a> AsUniform<'a> for &'a nalgebra::Matrix4<f32> {
	fn as_uniform_value(self) -> glium::uniforms::UniformValue<'a> {
		glium::uniforms::UniformValue::Mat4(unsafe { transmute(*self) })
	}
}

impl<'a> AsUniform<'a> for &'a nalgebra::Vector3<f32> {
	fn as_uniform_value(self) -> glium::uniforms::UniformValue<'a> {
		glium::uniforms::UniformValue::Vec3(unsafe { transmute(*self) })
	}
}

impl<'a> AsUniform<'a> for &'a nalgebra::Unit<nalgebra::Vector3<f32>> {
	fn as_uniform_value(self) -> glium::uniforms::UniformValue<'a> {
		self.as_ref().as_uniform_value()
	}
}

impl<'a> glium::uniforms::Uniforms for Uniforms<'a> {
	fn visit_values<'b, F>(&'b self, mut visit: F) where
		F: FnMut(&str, glium::uniforms::UniformValue<'b>)
	{
		visit("light_direction", self.light_direction.as_uniform_value());
		visit("transform",       self.transform.as_uniform_value());
		visit("mat_diffuse",     self.material.diffuse.as_uniform_value());
		visit("mat_specular",    self.material.specular.as_uniform_value());
		visit("mat_opacity",     self.material.opacity.as_uniform_value());
	}
}

pub const VERTEX_SHADER: &str = r#"
	#version 150
	uniform mat4 transform;

	in vec3 position;
	in vec3 normal;
	in vec2 texture;

	out vec3 v_normal;
	out vec2 v_texture;

	void main() {
		v_normal = transpose(inverse(mat3(transform))) * normal;
		v_texture = texture;
		gl_Position = transform * vec4(position, 1.0);
	}
"#;

pub const FRAGMENT_SHADER: &str = r#"
	#version 140
	in vec3 v_normal;
	out vec4 color;

	uniform vec3 light_direction;
	uniform vec3 mat_diffuse;
	uniform vec3 mat_specular;
	uniform float mat_opacity;

	float dot_normal(vec3 a, vec3 b) {
		return dot(a, b) / length(a) / length(b);
	}

	void main() {
		float brightness = dot_normal(v_normal, light_direction);
		vec3 diffuse = mat_diffuse * smoothstep(-0.75, 0.75, brightness);
		color = vec4(diffuse, mat_opacity);
	}
"#;

pub struct SimpleShaderProgram(glium::Program);

pub fn program(display: &impl glium::backend::Facade) -> Result<SimpleShaderProgram, glium::program::ProgramCreationError> {
	glium::Program::from_source(display, &VERTEX_SHADER, &FRAGMENT_SHADER, None).map(SimpleShaderProgram)
}

impl SimpleShaderProgram {
	pub fn into_inner (     self) ->      glium::Program {      self.0 }
	pub fn inner      (    &self) ->     &glium::Program {     &self.0 }
	pub fn inner_mut  (&mut self) -> &mut glium::Program { &mut self.0 }
}

impl AsRef<glium::Program> for SimpleShaderProgram {
	fn as_ref(&self) -> &glium::Program { self.inner() }
}

unsafe impl<'a> ShaderProgram<VertexPositionNormal, Uniforms<'a>> for SimpleShaderProgram {
	fn program(&self) -> &glium::Program { self.inner() }
}

unsafe impl<'a> ShaderProgram<VertexPositionNormalTexture, Uniforms<'a>> for SimpleShaderProgram {
	fn program(&self) -> &glium::Program { self.inner() }
}


impl<'a, V: glium::Vertex, I: glium::index::Index, P> Drawable<(&'a P, &'a Uniforms<'a>)> for RenderableMesh<V, I, Material> where
	P: ShaderProgram<V, Uniforms<'a>> + AsRef<glium::Program>,
{
	fn draw(&self, surface: &mut impl glium::Surface, draw_params: &glium::DrawParameters, extra: (&'a P, &'a Uniforms<'a>)) -> DrawResult {
		let (program, uniforms) = extra;
		for (indices, material) in &self.polygons {
			let uniforms = Uniforms{material: material, ..*uniforms};
			surface.draw(&self.vertices, indices, program.as_ref(), &uniforms, draw_params)?;
		}
		Ok(())
	}
}

impl<'a, P, O> Drawable<(&'a P, &'a Uniforms<'a>)> for TransformedObject<O> where
	for <'b> O: Drawable<(&'b P, &'b Uniforms<'b>)>
{
	fn draw(&self, surface: &mut impl glium::Surface, draw_params: &glium::DrawParameters, extra: (&'a P, &'a Uniforms<'a>)) -> DrawResult {
		let (program, uniforms) = extra;
		let transform = uniforms.transform * self.transform;;
		let uniforms  = Uniforms{transform: &transform, ..*uniforms};
		self.object.draw(surface, draw_params, (program, &uniforms))
	}
}
