use crate::{arg_parsers::Rotation, error::MagickError, image::Image, wm_err};

pub fn rotate(image: &mut Image, rotation: &Rotation) -> Result<(), MagickError> {
    match rotation {
        Rotation::Clockwise(num) => {
            if *num == 90 {
                image.pixels = image.pixels.rotate90();
                Ok(())
            } else if *num == 180 {
                image.pixels = image.pixels.rotate180();
                Ok(())
            } else if *num == 270 {
                image.pixels = image.pixels.rotate270();
                Ok(())
            } else {
                Err(wm_err!("Unsupported rotation value"))
            }
        }
        Rotation::CounterClockwise(_) => Err(wm_err!("Counterclockwise rotation is not supported")),
    }
}
