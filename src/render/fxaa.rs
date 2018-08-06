use glium;
use glium::framebuffer::SimpleFrameBuffer;

pub struct Fxaa<'a, Facade: 'a> {
	facade:        &'a Facade,
	vertex_buffer: glium::VertexBuffer<SpriteVertex>,
	index_buffer:  glium::IndexBuffer<u16>,
	program:       glium::Program,
	target_color:  Option<glium::texture::Texture2d>,
	target_depth:  Option<glium::framebuffer::DepthRenderBuffer>,
}

#[derive(Copy, Clone)]
struct SpriteVertex {
	position: [f32; 2],
	texture:  [f32; 2],
}

implement_vertex!(SpriteVertex, position, texture);

const VERTEX_SHADER : &str = r"
	#version 100
	attribute vec2 position;
	attribute vec2 texture;
	varying vec2 v_texture;
	void main() {
		gl_Position = vec4(position, 0.0, 1.0);
		v_texture   = texture;
	}
";


const FRAGMENT_SHADER : &str = r"
	#version 100
	precision mediump float;

	uniform vec2 resolution;
	uniform sampler2D tex;
	varying vec2 v_texture;

	#define FXAA_REDUCE_MIN   (1.0 / 128.0)
	#define FXAA_REDUCE_MUL   (1.0 / 8.0)
	#define FXAA_SPAN_MAX     8.0

	vec4 fxaa(sampler2D tex, vec2 fragCoord, vec2 resolution, vec2 v_rgbNW, vec2 v_rgbNE, vec2 v_rgbSW, vec2 v_rgbSE, vec2 v_rgbM) {
		mediump vec2 inverseVP = vec2(1.0 / resolution.x, 1.0 / resolution.y);
		vec3 rgbNW    = texture2D(tex, v_rgbNW).xyz;
		vec3 rgbNE    = texture2D(tex, v_rgbNE).xyz;
		vec3 rgbSW    = texture2D(tex, v_rgbSW).xyz;
		vec3 rgbSE    = texture2D(tex, v_rgbSE).xyz;
		vec4 texColor = texture2D(tex, v_rgbM);
		vec3 rgbM     = texColor.xyz;
		vec3 luma     = vec3(0.299, 0.587, 0.114);
		float lumaNW  = dot(rgbNW, luma);
		float lumaNE  = dot(rgbNE, luma);
		float lumaSW  = dot(rgbSW, luma);
		float lumaSE  = dot(rgbSE, luma);
		float lumaM   = dot(rgbM,  luma);
		float lumaMin = min(lumaM, min(min(lumaNW, lumaNE), min(lumaSW, lumaSE)));
		float lumaMax = max(lumaM, max(max(lumaNW, lumaNE), max(lumaSW, lumaSE)));

		mediump vec2 dir;
		dir.x = -((lumaNW + lumaNE) - (lumaSW + lumaSE));
		dir.y =  ((lumaNW + lumaSW) - (lumaNE + lumaSE));

		float dirReduce = max((lumaNW + lumaNE + lumaSW + lumaSE) * (0.25 * FXAA_REDUCE_MUL), FXAA_REDUCE_MIN);

		float rcpDirMin = 1.0 / (min(abs(dir.x), abs(dir.y)) + dirReduce);
		dir = min(vec2(FXAA_SPAN_MAX, FXAA_SPAN_MAX), max(vec2(-FXAA_SPAN_MAX, -FXAA_SPAN_MAX), dir * rcpDirMin)) * inverseVP;

		vec3 rgbA = 0.5 * (
			  texture2D(tex, fragCoord * inverseVP + dir * (1.0 / 3.0 - 0.5)).xyz
			+ texture2D(tex, fragCoord * inverseVP + dir * (2.0 / 3.0 - 0.5)).xyz
		);
		vec3 rgbB = rgbA * 0.5 + 0.25 * (
			  texture2D(tex, fragCoord * inverseVP + dir * -0.5).xyz
			+ texture2D(tex, fragCoord * inverseVP + dir * 0.5).xyz
		);
		float lumaB = dot(rgbB, luma);

		if ((lumaB < lumaMin) || (lumaB > lumaMax)) {
			return vec4(rgbA, texColor.a);
		} else {
			return vec4(rgbB, texColor.a);
		}
	}

	void main() {
		vec2 fragCoord = v_texture * resolution;

		vec2 inverseVP = 1.0 / resolution.xy;
		mediump vec2 v_rgbNW = (fragCoord + vec2(-1.0, -1.0)) * inverseVP;
		mediump vec2 v_rgbNE = (fragCoord + vec2(1.0, -1.0)) * inverseVP;
		mediump vec2 v_rgbSW = (fragCoord + vec2(-1.0, 1.0)) * inverseVP;
		mediump vec2 v_rgbSE = (fragCoord + vec2(1.0, 1.0)) * inverseVP;
		mediump vec2 v_rgbM = vec2(fragCoord * inverseVP);
		gl_FragColor = fxaa(tex, fragCoord, resolution, v_rgbNW, v_rgbNE, v_rgbSW, v_rgbSE, v_rgbM);
	}
";

impl<'a, Facade: 'a + glium::backend::Facade> Fxaa<'a, Facade> {
	pub fn new(facade: &'a Facade) -> Self {
		Self {
			facade: facade,

			vertex_buffer: glium::VertexBuffer::new(facade, &[
				SpriteVertex { position: [-1.0, -1.0], texture: [0.0, 0.0] },
				SpriteVertex { position: [-1.0,  1.0], texture: [0.0, 1.0] },
				SpriteVertex { position: [ 1.0,  1.0], texture: [1.0, 1.0] },
				SpriteVertex { position: [ 1.0, -1.0], texture: [1.0, 0.0] }
			]).unwrap(),

			index_buffer: glium::index::IndexBuffer::new(facade, glium::index::PrimitiveType::TriangleStrip, &[1 as u16, 2, 0, 3]).unwrap(),
			program:      glium::Program::from_source(facade, &VERTEX_SHADER, &FRAGMENT_SHADER, None).unwrap(),
			target_color: None,
			target_depth: None,
		}
	}

	pub fn draw<Target, DrawFunc, ResultT, ResultE>(&mut self, target: &mut Target, mut draw: DrawFunc) -> Result<ResultT, ResultE> where
		Target: glium::Surface,
		DrawFunc: FnMut(&mut SimpleFrameBuffer) -> Result<ResultT, ResultE>
	{
		let target_dimensions = target.get_dimensions();

		{
			let clear = if let &Some(ref tex) = &self.target_color {
				tex.get_width() != target_dimensions.0 || tex.get_height().unwrap() != target_dimensions.1
			} else {
				false
			};
			if clear { self.target_color = None; }
		}

		{
			let clear = if let &Some(ref tex) = &self.target_depth {
				tex.get_dimensions() != target_dimensions
			} else {
				false
			};
			if clear { self.target_depth = None; }
		}

		if self.target_color.is_none() {
			let texture = glium::texture::Texture2d::empty(self.facade, target_dimensions.0 as u32, target_dimensions.1 as u32).unwrap();
			self.target_color = Some(texture);
		}
		let target_color = self.target_color.as_ref().unwrap();

		if self.target_depth.is_none() {
			let texture = glium::framebuffer::DepthRenderBuffer::new(self.facade, glium::texture::DepthFormat::I24, target_dimensions.0 as u32, target_dimensions.1 as u32).unwrap();
			self.target_depth = Some(texture);
		}
		let target_depth = self.target_depth.as_ref().unwrap();

		let output = draw(&mut SimpleFrameBuffer::with_depth_buffer(self.facade, target_color, target_depth).unwrap())?;

		let uniforms = uniform! {
			tex: target_color,
			resolution: (target_dimensions.0 as f32, target_dimensions.1 as f32)
		};

		target.draw(&self.vertex_buffer, &self.index_buffer, &self.program, &uniforms, &Default::default()).unwrap();
		Ok(output)
	}
}
