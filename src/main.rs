use anyhow::{Context, Result, anyhow};
use clap::{Parser, value_parser};
use image::{Rgba, RgbaImage};
use std::{path::PathBuf, str::FromStr};

/// Generates an image of specified dimensions from a hex color code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Width of image in pixels
    #[arg(value_parser = value_parser!(u32).range(1..))]
    width: u32,

    /// Height of image in pixels
    #[arg(value_parser = value_parser!(u32).range(1..))]
    height: u32,

    /// Color in hex code format (e.g., "FF0000" for red)
    #[arg()]
    color: Color,

    /// Output file path [default: <hex_code>.png]
    #[arg()]
    output: Option<PathBuf>,
}

#[derive(Debug, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    hex: String,
}

impl Color {
    fn to_rgba(&self) -> Rgba<u8> {
        Rgba([self.r, self.g, self.b, self.a])
    }
}

impl FromStr for Color {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches('#');

        // check if formatted as RGB or RGBA
        if s.len() != 6 && s.len() != 8 {
            return Err(anyhow!(
                "Color must be a 6-digit (RGB) or 8-digit (RGBA) hex code (e.g., FF0000 for red, FF0000FF for opaque red)"
            ));
        }

        let r = u8::from_str_radix(&s[0..2], 16)?;
        let g = u8::from_str_radix(&s[2..4], 16)?;
        let b = u8::from_str_radix(&s[4..6], 16)?;

        let a = if s.len() == 8 {
            u8::from_str_radix(&s[6..8], 16)?
        } else {
            255 // default to fully opaque
        };

        Ok(Color {
            r,
            g,
            b,
            a,
            hex: s.to_string(),
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let img = RgbaImage::from_pixel(args.width, args.height, args.color.to_rgba());

    let output_path = args
        .output
        .unwrap_or_else(|| PathBuf::from(format!("{}.png", args.color.hex)));

    let display_path = output_path.display();
    img.save(&output_path)
        .with_context(|| format!("Failed to save image to {}", display_path))?;

    println!("Image saved to {}", display_path);

    Ok(())
}
