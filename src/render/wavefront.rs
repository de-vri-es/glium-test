use std;
use glium;
use obj;
use num::NumCast;

use std::io;
use std::collections::{
	BTreeMap,
	btree_map::Entry::{Occupied, Vacant},
};

use super::{
	Mesh,
	Triangle,
	TriangleList,
	VertexPosition,
	VertexPositionNormal,
	VertexPositionNormalTexture,
	simple_shader,
};

pub trait GetPositions {
	fn get_positions(&self) -> &[[f32; 3]];
}

pub trait GetNormals {
	fn get_normals(&self) -> &[[f32; 3]];
}

pub trait GetTexturePositions {
	fn get_texture_positions(&self) -> &[[f32; 2]];
}

/// Allow extracting vertices from a buffer by an index.
/**
 * The buffer an index may be arbitrarily complex types,
 * like tuples of buffers and indices.
 */
pub trait ExtractVertex<Buffer, Index>: Sized {
	fn extract(buffer: &Buffer, index: &Index) -> Result<Self, String>;
}

impl<'a, P: obj::GenPolygon> GetPositions for obj::Obj<'a, P> {
	fn get_positions(&self) -> &[[f32; 3]] { &self.position }
}

impl<'a, P: obj::GenPolygon> GetNormals for obj::Obj<'a, P> {
	fn get_normals(&self) -> &[[f32; 3]] { &self.normal }
}

impl<'a, P: obj::GenPolygon> GetTexturePositions for obj::Obj<'a, P> {
	fn get_texture_positions(&self) -> &[[f32; 2]] { &self.texture }
}

fn get_checked<'a, T>(slice: &'a [T], index: usize, name: &str) -> Result<&'a T, String> {
	slice.get(index).ok_or_else(|| format!("{} index out of range, total count: {}, index: {}", name, slice.len(), index))
}

/// Allow extracting position vertices from an Obj.
impl<B> ExtractVertex<B, obj::IndexTuple> for VertexPosition where
	B: GetPositions
{
	fn extract(buffer: &B, index: &obj::IndexTuple) -> Result<VertexPosition, String> {
		Ok(VertexPosition{
			position: *get_checked(buffer.get_positions(), index.0, "position")?,
		})
	}
}

/// Allow extracting position+normal vertices from an Obj.
impl<B> ExtractVertex<B, obj::IndexTuple> for VertexPositionNormal where
	B: GetPositions + GetNormals
{
	fn extract(buffer: &B, index: &obj::IndexTuple) -> Result<VertexPositionNormal, String> {
		let (position, normal) = match *index {
			obj::IndexTuple(p, None, None) => (p, p),
			obj::IndexTuple(p, _, Some(n)) => (p, n),
			_ => return Err(format!("no normal index in vertex")),
		};

		Ok(VertexPositionNormal{
			position: *get_checked(buffer.get_positions(), position, "position")?,
			normal:   *get_checked(buffer.get_normals(),   normal,   "normal")?,
		})
	}
}

/// Allow extracting position+normal+texture vertices from an Obj.
impl<B> ExtractVertex<B, obj::IndexTuple> for VertexPositionNormalTexture where
	B: GetPositions + GetNormals + GetTexturePositions
{
	fn extract(buffer: &B, index: &obj::IndexTuple) -> Result<VertexPositionNormalTexture, String> {
		let (position, normal, texture) = match *index {
			obj::IndexTuple(p, None,    None   ) => (p, p, p),
			obj::IndexTuple(p, Some(t), Some(n)) => (p, n, t),
			obj::IndexTuple(_, Some(_), None   ) => return Err(format!("no normal index in vertex")),
			obj::IndexTuple(_, None,    Some(_)) => return Err(format!("no texture index in vertex")),
		};

		Ok(VertexPositionNormalTexture{
			position: *get_checked(buffer.get_positions(),         position, "position")?,
			normal:   *get_checked(buffer.get_normals(),           normal,   "normal")?,
			texture:  *get_checked(buffer.get_texture_positions(), texture,  "texture position")?
		})
	}
}

impl<'a> From<&'a obj::Material> for simple_shader::Material {
	fn from(obj_mat: &'a obj::Material) -> Self {
		Self{
			diffuse:  obj_mat.kd.unwrap_or([1.0, 1.0, 1.0]),
			specular: obj_mat.ks.unwrap_or([0.0, 0.0, 0.0]),
			opacity:  obj_mat.d.unwrap_or(1.0),
		}
	}
}

//pub fn convert_material(obj_mat: &obj::Material) -> Material {
	//Material {
		//diffuse:  obj_mat.kd.unwrap_or([1.0, 1.0, 1.0]),
		//specular: obj_mat.ks.unwrap_or([0.0, 0.0, 0.0]),
		//opacity:  obj_mat.d.unwrap_or(1.0),
	//}
//}

struct ObjectReindexer<'obj, 'mat, V, I> where
	V: glium::Vertex + ExtractVertex<obj::Obj<'mat, obj::SimplePolygon>, obj::IndexTuple>,
	I: glium::index::Index + NumCast,
	'mat: 'obj,
{
	object: &'obj obj::Obj<'mat, obj::SimplePolygon>,
	vertices: Vec<V>,
	cache: BTreeMap<obj::IndexTuple, I>,
}

impl<'obj, 'mat, V, I> ObjectReindexer<'obj, 'mat, V, I> where 
	V: glium::Vertex + ExtractVertex<obj::Obj<'mat, obj::SimplePolygon>, obj::IndexTuple>,
	I: glium::index::Index + NumCast,
	'mat: 'obj,
{
	fn reindex<M>(object: &'obj obj::Obj<'mat, obj::SimplePolygon>) -> Result<Mesh<V, I, M>, String> where 
		M: for <'b> From<&'b obj::Material> + Clone + Default,
	{
		Self{
			object,
			vertices: Vec::with_capacity(object.position.len()),
			cache:    BTreeMap::new(),
		}.process()
	}

	fn process_vertex(&mut self, index_tuple: obj::IndexTuple) -> Result<I, String> {
		match self.cache.entry(index_tuple) {
			Occupied(entry) => Ok(*entry.get()),
			Vacant(entry) => {
				let index = self.vertices.len();
				let index = I::from(index).ok_or_else(|| format!("index out of bounds: {}", index))?;
				self.vertices.push(V::extract(self.object, &index_tuple)?);
				entry.insert(index);
				Ok(index)
			}
		}
	}

	fn process_indices(&mut self, faces: &mut Vec<Triangle<I>>, indices: &obj::SimplePolygon) -> Result<(), String> {
		if indices.len() % 3 != 0 { return Err(format!("polygon indices length is not a multiple of 3: {}", indices.len())); }
		for i in 0..(indices.len() / 3) {
			faces.push(Triangle([
				self.process_vertex(indices[i * 3 + 0])?,
				self.process_vertex(indices[i * 3 + 1])?,
				self.process_vertex(indices[i * 3 + 2])?,
			]));
		}
		Ok(())
	}

	fn process_group<M>(&mut self, group: &obj::Group<'mat, obj::SimplePolygon>) -> Result<(TriangleList<I>, M), String> where
		M: for <'b> From<&'b obj::Material> + Clone + Default,
	{
		let total_faces = group.polys.iter().map(|x| x.len() / 3).sum();
		let mut faces: Vec<Triangle<I>> = Vec::with_capacity(total_faces);
		for indices in &group.polys {
			self.process_indices(&mut faces, indices)?
		}

		let material = group.material.as_ref().map(|x| x.as_ref().into()).unwrap_or_default();

		Ok((TriangleList(faces), material))
	}

	fn process<M>(mut self) -> Result<Mesh<V, I, M>, String> where
		M: for <'b> From<&'b obj::Material> + Clone + Default,
	{
		let total_polygons = self.object.objects.iter().map(|x| x.groups.iter().count()).sum();
		let mut polygons = Vec::with_capacity(total_polygons);
		for object in &self.object.objects {
			for group in &object.groups {
				polygons.push(self.process_group(group)?);
			}
		}

		Ok(Mesh{
			vertices: self.vertices,
			polygons,
		})
	}
}

pub fn load<'a, V, I, M>(buffer: &mut impl io::BufRead) -> io::Result<Mesh<V, I, M>> where
	V: glium::Vertex + ExtractVertex<obj::Obj<'a, obj::SimplePolygon>, obj::IndexTuple>,
	I: glium::index::Index + NumCast,
	M: for <'b> From<&'b obj::Material> + Clone + Default,
{
	ObjectReindexer::reindex(&obj::Obj::load_buf(buffer)?).map_err(|x| io::Error::new(io::ErrorKind::Other, x))
}

pub fn load_file<'a, V, I, M>(path: &'a (impl AsRef<std::path::Path> + ?Sized), load_materials: bool) -> io::Result<Mesh<V, I, M>> where
	V: glium::Vertex + ExtractVertex<obj::Obj<'a, obj::SimplePolygon>, obj::IndexTuple>,
	I: glium::index::Index + NumCast,
	M: for <'b> From<&'b obj::Material> + Clone + Default,
{
	let mut object = obj::Obj::load(path.as_ref())?;
	if load_materials {
		object.load_mtls().map_err(|x| {
			if x.is_empty() {
				io::Error::new(io::ErrorKind::Other, String::from("unknown error occured while loading materials"))
			} else {
				let (file, error) = &x[0];
				io::Error::new(io::ErrorKind::Other, format!("error parsing {}: {}", file, error))
			}
		})?;
	}

	ObjectReindexer::reindex(&object).map_err(|x| io::Error::new(io::ErrorKind::Other, x))
}
