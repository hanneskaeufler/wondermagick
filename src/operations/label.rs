use crate::{
    error::MagickError, image::Image, operations::alpha::Alpha, operations::composite::composite,
    operations::gravity::Gravity,
};
use image::ImageFormat;
use image_text::{AxisAlign, Text, TextBlock, TextBlockPosition};

fn gravity_into(gravity: Gravity, width: u32, height: u32) -> TextBlockPosition {
    match gravity {
        Gravity::Center => TextBlockPosition {
            x: AxisAlign::CenterAtCanvasCenter,
            y: AxisAlign::CenterAtCanvasCenter,
        },
        Gravity::North => TextBlockPosition {
            x: AxisAlign::CenterAtCanvasCenter,
            y: AxisAlign::StartAt(0 as f32),
        },
        Gravity::South => TextBlockPosition {
            x: AxisAlign::CenterAtCanvasCenter,
            y: AxisAlign::EndAt(height as f32),
        },
        Gravity::East => TextBlockPosition {
            x: AxisAlign::EndAt(width as f32),
            y: AxisAlign::CenterAtCanvasCenter,
        },
        Gravity::West => TextBlockPosition {
            x: AxisAlign::StartAt(0 as f32),
            y: AxisAlign::CenterAtCanvasCenter,
        },
        Gravity::Northeast => TextBlockPosition {
            x: AxisAlign::EndAt(width as f32),
            y: AxisAlign::StartAt(0 as f32),
        },
        Gravity::Northwest => TextBlockPosition {
            x: AxisAlign::StartAt(0 as f32),
            y: AxisAlign::StartAt(0 as f32),
        },
        Gravity::Southeast => TextBlockPosition {
            x: AxisAlign::EndAt(width as f32),
            y: AxisAlign::EndAt(height as f32),
        },
        Gravity::Southwest => TextBlockPosition {
            x: AxisAlign::StartAt(0 as f32),
            y: AxisAlign::EndAt(height as f32),
        },
    }
}

pub fn label(
    image: &mut Image,
    text: &std::ffi::OsStr,
    color: (u8, u8, u8, u8),
    gravity: Gravity,
) -> Result<(), MagickError> {
    let w = image.pixels.width();
    let h = image.pixels.height();

    // First, create a temporary image to draw the text with full opacity
    let mut text_image = image::DynamicImage::new_rgba8(w, h);

    // Draw the text with full alpha on the temporary image
    image_text::draw_text(
        &mut text_image,
        TextBlock {
            alignment: gravity_into(gravity, w, h),
            max_width: Some(w as f32),
            max_height: Some(h as f32),
            text_align: Default::default(),
            text_spans: vec![Text {
                text: String::from(text.to_string_lossy()),
                font_size: 200.0,
                font_weight: 1200,
                color: (color.0, color.1, color.2, 255), // Force full alpha for rendering
                font: None,
                line_height: None,
            }],
            font: None,
        },
    );

    // Now composite the temporary image onto the original with the desired alpha
    let text_alpha = color.3;
    composite(
        image,
        &mut Image {
            format: Some(ImageFormat::Png),
            pixels: text_image,
            exif: None,
            icc: None,
            properties: crate::image::InputProperties {
                filename: std::ffi::OsString::new(),
                color_type: image::ExtendedColorType::Rgba8,
            },
        },
        Gravity::Center,
        Alpha(text_alpha as f32 / 255.0),
    )
}
