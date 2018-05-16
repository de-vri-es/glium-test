use ::glium;
use ::image;
use ::std;

#[derive(Debug,Error)]
pub enum TextureLoadError {
	Image(image::ImageError),
	Io(std::io::Error),
	Load(glium::texture::TextureCreationError),
}

pub fn triangle_texture(display: &glium::Display) -> Result<glium::texture::Texture2d, TextureLoadError> {
	let image = image::load(std::io::Cursor::new(&include_bytes!("textures/triangle.png")[..]), image::PNG).map_err(|x| TextureLoadError::Image(x))?.to_rgb();
	let size  = image.dimensions();
	let image = glium::texture::RawImage2d::from_raw_rgb_reversed(&image.into_raw(), size);
	glium::texture::Texture2d::new(display, image).map_err(|x| TextureLoadError::Load(x))
}
