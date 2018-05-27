use std::io::Cursor;
use vertex::VertexPositionNormal;
use mesh::Mesh;
use wavefront;

const TEAPOT_BYTES: &[u8] = include_bytes!("assets/monkey.obj");

pub fn object() -> Mesh<VertexPositionNormal, u16> {
	wavefront::load(&mut Cursor::new(&TEAPOT_BYTES)).unwrap()
}
