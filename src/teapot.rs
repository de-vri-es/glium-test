use std::io::Cursor;
use mesh;
use mesh_obj;

const TEAPOT_BYTES: &[u8] = include_bytes!("assets/teapot.obj");

pub fn object() -> mesh_obj::ObjMesh<'static, mesh::VertexPositionNormal, u16> {
	mesh_obj::load(&mut Cursor::new(&TEAPOT_BYTES)).unwrap()
}
