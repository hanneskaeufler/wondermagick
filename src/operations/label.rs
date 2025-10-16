use crate::{image::Image, operations::gravity::Gravity};
//use image::{ColorType, DynamicImage, Pixel, Rgba};
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

pub fn label(image: &mut Image, text: &std::ffi::OsStr, color: (u8, u8, u8, u8), gravity: Gravity) {
    let w = image.pixels.width();
    let h = image.pixels.height();
    image_text::draw_text(
        &mut image.pixels,
        TextBlock {
            alignment: gravity_into(gravity, w, h),
            max_width: Some(w as f32),
            max_height: Some(h as f32),
            text_align: Default::default(),
            text_spans: vec![Text {
                text: String::from(text.to_string_lossy()),
                font_size: 42.0,
                font_weight: 400,
                color: color,
                font: "Arial-Black".into(),
                line_height: None,
            }],
            font: "Arial-Black".into(),
        },
    );
}
