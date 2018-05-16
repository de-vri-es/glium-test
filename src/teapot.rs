use std::io::Cursor;
use obj::{Obj,SimplePolygon};

const TEAPOT_BYTES: &[u8] = include_bytes!("assets/teapot.obj");

pub fn object() -> Obj<'static, SimplePolygon> {
	Obj::<SimplePolygon>::load_buf(&mut Cursor::new(&TEAPOT_BYTES)).unwrap()
}
