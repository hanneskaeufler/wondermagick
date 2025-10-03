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
