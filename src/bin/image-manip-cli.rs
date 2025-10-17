use clap::{Args, Parser, Subcommand};
use image::ImageFormat;
use std::path::PathBuf;
use std::str::FromStr;
use std::{
    ffi::{OsStr, OsString},
    num::ParseIntError,
};
use wondermagick::arg_parsers::IdentifyFormat;
use wondermagick::{
    arg_parsers::{FileFormat, Location, ResizeConstraint, ResizeGeometry, ResizeTarget},
    decode::decode,
    encode::encode,
    error::MagickError,
    operations::{
        alpha::Alpha, auto_orient::auto_orient, composite::composite, gravity::Gravity,
        identify::identify, label::label, resize::resize,
    },
    plan::Modifiers,
    plan::Strip,
    wm_err, wm_try,
};

#[derive(Parser, Debug)]
pub struct App {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Manipulate an image
    Manipulate(ManipulateArgs),

    /// Identify an image
    Identify(IdentitfyArgs),
}

#[derive(Args, Debug)]
struct ManipulateArgs {
    /// Path to the input image
    input: OsString,

    /// Path to the output image
    output: OsString,

    /// Resize instruction
    #[clap(long)]
    resize: Option<OsString>,

    /// Strip for privacy
    #[clap(long)]
    strip_metadata: bool,

    /// Quality (1-100)
    #[clap(long)]
    quality: Option<u8>,

    /// Composite a watermark image onto the output image
    #[clap(long)]
    watermark_image: Option<OsString>,

    /// Make watermark image transparent (0.0-1.0)
    #[clap(long)]
    watermark_image_opacity: Option<f32>,

    /// Position watermark image
    #[clap(long)]
    watermark_image_gravity: Option<String>,

    /// Composite a watermark text onto the output image
    #[clap(long)]
    watermark_text: Option<OsString>,

    /// Make watermark text colorful (rgba, comma-separated)
    #[clap(long)]
    watermark_text_color_rgba: Option<String>,

    /// Position watermark text
    #[clap(long)]
    watermark_text_gravity: Option<String>,
}

#[derive(Args, Debug)]
struct IdentitfyArgs {
    input: OsString,
}

fn real_main() -> Result<(), MagickError> {
    let app_args = App::parse();
    wondermagick::init::init();

    match app_args.command {
        Command::Manipulate(args) => {
            let mut image = wm_try!(decode(&Location::Path(PathBuf::from(args.input)), None));

            match (&args.watermark_image, &args.watermark_text) {
                (Some(_), Some(_)) => {
                    return Err(wm_err!(
                        "cannot specify both watermark image and watermark text"
                    ));
                }
                _ => {}
            }

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
                                dims[1].parse::<u32>().map_err(|_| {
                                    wm_err!("invalid height in resize argument")
                                })?
                            ),
                            ignore_aspect_ratio: false
                        },
                        constraint: ResizeConstraint::OnlyShrink
                    }
                ));
            }

            if let Some(watermark_image) = &args.watermark_image {
                let mut watermark = wm_try!(decode(
                    &Location::Path(PathBuf::from(watermark_image)),
                    None
                ));

                wm_try!(composite(
                    &mut image,
                    &mut watermark,
                    args.watermark_image_gravity
                        .map(|gravity_str| Gravity::try_from(&gravity_str)
                            .unwrap_or(Gravity::Center))
                        .unwrap_or(Gravity::Center),
                    args.watermark_image_opacity
                        .map(|v| Alpha(v))
                        .unwrap_or(Alpha(1.0))
                ));
            }

            if let Some(watermark_text) = &args.watermark_text {
                let color = args
                    .watermark_text_color_rgba
                    .map_or(Ok((255, 255, 255, 255)), |color_str| {
                        let comps: Vec<&str> = color_str.split(',').collect();
                        let r = u8::from_str(comps[0])?;
                        let g = u8::from_str(comps[1])?;
                        let b = u8::from_str(comps[2])?;
                        let a = u8::from_str(comps[3])?;
                        Ok((r, g, b, a))
                    })
                    .map_err(|_: ParseIntError| wm_err!("invalid RGBA color"))?;

                label(
                    &mut image,
                    watermark_text,
                    color,
                    args.watermark_text_gravity
                        .map(|gravity_str| {
                            Gravity::try_from(&gravity_str).unwrap_or(Gravity::Center)
                        })
                        .unwrap_or(Gravity::Center),
                );
            }

            wm_try!(encode(
                &mut image,
                &Location::Path(PathBuf::from(args.output)),
                Some(FileFormat::Format(ImageFormat::Jpeg)),
                &Modifiers {
                    quality: args.quality.map(|q| q as f64),
                    strip: Strip {
                        exif: args.strip_metadata,
                        icc: args.strip_metadata,
                    },
                    identify_format: Default::default(),
                },
            ));
        }
        Command::Identify(args) => {
            let mut image = wm_try!(decode(&Location::Path(PathBuf::from(args.input)), None));

            identify(
                &mut image,
                IdentifyFormat::try_from(OsStr::new("%w %h")).ok(),
            )?;
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = real_main() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
