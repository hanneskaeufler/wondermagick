use clap::Parser;
use image::ImageFormat;
use std::ffi::OsString;
use std::path::PathBuf;
use wondermagick::{
    arg_parsers::{Location, ResizeConstraint, ResizeGeometry, ResizeTarget},
    decode::decode,
    encode::encode,
    error::MagickError,
    operations::auto_orient::auto_orient,
    operations::composite::{composite, Alpha, Gravity},
    operations::resize::resize,
    plan::Modifiers,
    plan::Strip,
    wm_err, wm_try,
};

#[derive(Parser, Debug)]
struct Args {
    /// Path to the input image
    input: OsString,

    /// Path to the output image
    output: OsString,

    /// Resize instruction
    #[arg(long)]
    resize: Option<OsString>,

    /// Strip for privacy
    #[arg(long)]
    strip_metadata: bool,

    /// Quality (1-100)
    #[arg(long)]
    quality: Option<f64>,

    /// Composite a watermark image onto the output image
    #[arg(long)]
    watermark_image: Option<OsString>,

    /// Make watermark image transparent (0.0-1.0)
    #[arg(long)]
    watermark_image_opacity: Option<f32>,

    /// Position watermark image
    #[arg(long)]
    watermark_image_gravity: Option<String>,
}

fn real_main() -> Result<(), MagickError> {
    let args = Args::parse();
    wondermagick::init::init();
    let mut image = wm_try!(decode(&Location::Path(PathBuf::from(args.input)), None));

    wm_try!(auto_orient(&mut image));

    if let Some(size) = &args.resize {
        let size_str = size.to_string_lossy();
        let dims: Vec<&str> = size_str.split('x').collect();
        if dims.len() != 2 {
            return Err(wm_err!("invalid resize argument"));
        }

        wm_try!(resize(
            &mut image,
            &ResizeGeometry {
                target: ResizeTarget::Size {
                    width: Some(
                        dims[0]
                            .parse::<u32>()
                            .map_err(|_| wm_err!("invalid width in resize argument"))?
                    ),
                    height: Some(
                        dims[1]
                            .parse::<u32>()
                            .map_err(|_| { wm_err!("invalid height in resize argument") })?
                    ),
                    ignore_aspect_ratio: false
                },
                constraint: ResizeConstraint::OnlyShrink
            }
        ));
    }

    if let Some(watermark_image) = &args.watermark_image {
        let mut watermark = wm_try!(decode(&watermark_image, None));
        let gravity = if let Some(gravity_str) = &args.watermark_image_gravity {
            Gravity::try_from(gravity_str).map_err(|_| wm_err!("invalid gravity argument"))?
        } else {
            Gravity::Center
        };

        wm_try!(composite(
            &mut image,
            &mut watermark,
            gravity,
            args.watermark_image_opacity
                .map(|v| Alpha(v))
                .unwrap_or(Alpha(1.0))
        ));
    }

    wm_try!(encode(
        &mut image,
        &Location::Path(PathBuf::from(args.output)),
        Some(ImageFormat::Jpeg),
        &Modifiers {
            quality: args.quality,
            strip: Strip {
                exif: args.strip_metadata,
                icc: args.strip_metadata,
            },
            identify_format: Default::default(),
        },
    ));

    Ok(())
}

fn main() {
    if let Err(e) = real_main() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
