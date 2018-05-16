use mesh::{
	NormalVertices,
	Polygon,
	PositionVertices,
	TextureVertices,
	Triangle,
	NORMAL_VERTEX_FORMAT,
	POSITION_VERTEX_FORMAT,
	TEXTURE_VERTEX_FORMAT,
};

use ::glium;
use ::glium::index::Index;
use ::obj;
use ::num;

use ::std::borrow::Cow::Borrowed;

impl<'a, P: obj::GenPolygon> PositionVertices for obj::Obj<'a, P> {
	fn position_vertices(&self, display: &glium::Display) -> Result<glium::vertex::VertexBufferAny, glium::vertex::BufferCreationError> {
		Ok(unsafe { glium::VertexBuffer::new_raw(display, &self.position, Borrowed(&POSITION_VERTEX_FORMAT), 12) }?.into())
	}
}

impl<'a, P: obj::GenPolygon> NormalVertices for obj::Obj<'a, P> {
	fn normal_vertices(&self, display: &glium::Display) -> Result<glium::vertex::VertexBufferAny, glium::vertex::BufferCreationError> {
		Ok(unsafe { glium::VertexBuffer::new_raw(display, &self.normal, Borrowed(&NORMAL_VERTEX_FORMAT), 12) }?.into())
	}
}

impl<'a, P: obj::GenPolygon> TextureVertices for obj::Obj<'a, P> {
	fn texture_vertices(&self, display: &glium::Display) -> Result<glium::vertex::VertexBufferAny, glium::vertex::BufferCreationError> {
		Ok(unsafe { glium::VertexBuffer::new_raw(display, &self.texture, Borrowed(&TEXTURE_VERTEX_FORMAT), 8) }?.into())
	}
}

fn extract_index<I: num::NumCast>(tuple: &obj::IndexTuple) -> Result<I, String> {
	if let obj::IndexTuple(i, None, None) = tuple {
		I::from(*i).ok_or_else(|| format!("index value out of bounds: {}", i))
	} else {
		Err(String::from("separate normal or texture indices are not supported"))
	}
}

pub fn group_to_polygon<I: Index + num::NumCast>(group: &obj::Group<obj::SimplePolygon>) -> Result<Polygon<I>, String> {
	let mut total_size = 0;
	for polygon in &group.polys {
		if polygon.len() % 3 != 0 { return Err(format!("polygon indices length is not a multiple of 3: {}", polygon.len())); }
		total_size += polygon.len();
	}

	let mut result = Vec::with_capacity(total_size);
	for polygon in &group.polys {
		//for tuple in polygon {
		//	result.push(extract_index(tuple)?)
		//}
		for x in 0..(polygon.len() / 3) {
			result.push(Triangle([
				extract_index(&polygon[x * 3 + 0])?,
				extract_index(&polygon[x * 3 + 1])?,
				extract_index(&polygon[x * 3 + 2])?,
			]));
		}
	}
	Ok(Polygon(result))
	//Ok(result)
}
