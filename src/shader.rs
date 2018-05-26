use glium;
use glium::uniforms::AsUniformValue;

use nalgebra as na;
use std::mem::transmute;

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

pub struct Uniforms<'a> {
	light_direction: &'a na::Unit<na::Vector3<f32>>,
	transform:       &'a na::Matrix4<f32>,
	material:        &'a Material,
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

trait AsUniform<'a> {
	fn as_uniform_value(self) -> glium::uniforms::UniformValue<'a>;
}


impl<'a> AsUniform<'a> for &'a na::Matrix4<f32> {
	fn as_uniform_value(self) -> glium::uniforms::UniformValue<'a> {
		glium::uniforms::UniformValue::Mat4(unsafe { transmute(*self) })
	}
}

impl<'a> AsUniform<'a> for &'a na::Vector3<f32> {
	fn as_uniform_value(self) -> glium::uniforms::UniformValue<'a> {
		glium::uniforms::UniformValue::Vec3(unsafe { transmute(*self) })
	}
}

impl<'a> AsUniform<'a> for &'a na::Unit<na::Vector3<f32>> {
	fn as_uniform_value(self) -> glium::uniforms::UniformValue<'a> {
		self.as_ref().as_uniform_value()
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

pub struct DefaultProgram(glium::Program);

impl DefaultProgram {
	pub fn into_inner(self) -> glium::Program { self.0 }
	pub fn inner(&self) -> &glium::Program { &self.0 }
	pub fn inner_mut(&mut self) -> &mut glium::Program { &mut self.0 }
}

pub fn default_program(display: &glium::Display) -> Result<DefaultProgram, glium::program::ProgramCreationError> {
	glium::Program::from_source(display, &VERTEX_SHADER, &FRAGMENT_SHADER, None).map(DefaultProgram)
}
