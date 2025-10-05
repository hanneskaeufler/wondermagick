use crate::image::Image;
//use image::{ColorType, DynamicImage, Pixel, Rgba};
use image_text::{AxisAlign, TextBlock, TextBlockPosition};

pub fn label(image: &mut Image, watermark_text: &std::ffi::OsStr) {
    image_text::draw_text(
        &mut image.pixels,
        TextBlock::string(watermark_text.to_string_lossy()).with_alignment(TextBlockPosition {
            x: AxisAlign::CenterAtCanvasCenter,
            y: AxisAlign::CenterAtCanvasCenter,
        }),
    );
}
