use crate::{arg_parsers::Rotation, error::MagickError, image::Image};

pub fn rotate(image: &mut Image, _rotation: &Rotation) -> Result<(), MagickError> {
    image.pixels = image.pixels.rotate90();
    Ok(())
}
