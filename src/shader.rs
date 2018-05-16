use ::glium;

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

	void main() {
		float brightness = dot(normalize(v_normal), normalize(light_direction));
		vec3 dark_color = vec3(0.6, 0.0, 0.0);
		vec3 regular_color = vec3(1.0, 0.0, 0.0);
		color = vec4(mix(dark_color, regular_color, brightness), 1.0);
	}
"#;

pub fn program_pnt(display: &glium::Display) -> Result<glium::Program, glium::program::ProgramCreationError> {
	glium::Program::from_source(display, &VERTEX_SHADER, &FRAGMENT_SHADER, None)
}
