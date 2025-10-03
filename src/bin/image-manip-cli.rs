use clap::Parser;
use image::ImageFormat;
use std::ffi::OsString;
use std::path::PathBuf;
use wondermagick::{
    arg_parsers::Location, decode::decode, encode::encode, error::MagickError,
    operations::auto_orient::auto_orient, plan::Modifiers, plan::Strip, wm_try,
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
