// image_ops.rs
// Contains core image processing functions.
// This module is designed to be easily extensible with new operations.
// extern crate exif-rs;
extern crate exif;

use anyhow::{Result, anyhow};
use image::{DynamicImage, ImageFormat, ImageOutputFormat, imageops::FilterType};
use std::io::Cursor;

use exif::{Exif, In, Reader, Tag};
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Re-encodes an image to strip metadata.
/// This works by saving the image to an in-memory buffer (e.g., as a PNG)
/// and then re-loading it. The `image` crate typically only saves pixel data,
/// effectively stripping most non-pixel metadata like EXIF.
pub fn re_encode_image_to_strip_metadata(img: DynamicImage) -> Result<DynamicImage> {
    let mut buffer = Cursor::new(Vec::new());
    // Save as PNG to avoid loss during re-encoding for metadata stripping.
    img.write_to(&mut buffer, ImageOutputFormat::Png)
        .map_err(|e| anyhow!("Failed to write image to buffer for re-encoding: {}", e))?;
    buffer.set_position(0); // Reset cursor to beginning

    image::load_from_memory_with_format(buffer.into_inner().as_slice(), ImageFormat::Png).map_err(
        |e| {
            anyhow!(
                "Failed to reload image from buffer after re-encoding: {}",
                e
            )
        },
    )
}

/// Shrinks an image to specified dimensions and applies JPEG compression.
/// The image will be scaled down to fit within `target_width` and `target_height`
/// while maintaining its aspect ratio. `quality` is the JPEG quality (0-100).
/// The resulting image is then loaded back as a `DynamicImage`.
pub fn shrink_to_jpg(
    img: DynamicImage,
    target_width: u32,
    target_height: u32,
    quality: u8,
) -> Result<DynamicImage> {
    let resized_img = img.resize(target_width, target_height, FilterType::Lanczos3);

    // Convert to RGB8 if not already, as JPEG typically expects this for quality control.
    let rgb8_img = resized_img.to_rgb8();

    // Save the RGB8 image to an in-memory buffer as JPEG with the specified quality.
    let mut buffer = Cursor::new(Vec::new());
    rgb8_img
        .write_to(&mut buffer, ImageOutputFormat::Jpeg(quality))
        .map_err(|e| anyhow!("Failed to write shrunk image to buffer as JPG: {}", e))?;
    buffer.set_position(0); // Reset cursor to beginning

    // Load the JPEG image from the buffer back into a DynamicImage.
    image::load_from_memory_with_format(buffer.into_inner().as_slice(), ImageFormat::Jpeg)
        .map_err(|e| anyhow!("Failed to reload shrunk image from buffer: {}", e))
}

/// Saves a `DynamicImage` to a specified file path with a given format.
pub fn save_image_to_file(
    img: DynamicImage,
    path: &Path,
    format: ImageFormat,
) -> Result<DynamicImage> {
    img.save_with_format(path, format)
        .map_err(|e| anyhow!("Failed to save image to {}: {}", path.display(), e))?;
    Ok(img) // Return the image back for continued use in the app
}

pub fn print_exif(path: &Path) -> Result<(), exif::Error> {
    let file = File::open(path)?;
    let mut buf = BufReader::new(file);
    let exif = Reader::new().read_from_container(&mut buf)?;

    println!("print_exif: ");

    for &tag in &[
        Tag::Model,
        Tag::Make,
        Tag::DateTimeOriginal,
        Tag::FocalLength,
        Tag::ExposureTime,
        Tag::ApertureValue,
        Tag::FNumber,
        Tag::ExposureBiasValue,
        Tag::MeteringMode,
        Tag::ExposureProgram,
        Tag::WhiteBalance,
        Tag::Flash,
        Tag::ExposureMode,
        Tag::ExposureIndex,
        Tag::SensingMethod,
        Tag::SceneCaptureType,
        Tag::ColorSpace,
    ] {
        // move this to debug_print_exif
        // println!("print_exif: {}", tag);
        if let Some(field) = exif.get_field(tag, In::PRIMARY) {
            println!("{}: {}", tag, field.display_value().with_unit(&exif));
        }
    }

    let exif_data = exif::Reader::new().read_from_container(&mut buf)?;
    println!("print_exif: ");
    for field in exif_data.fields() {
        println!("{}: {}", field.tag, field.display_value());
    }

    println!("print_exif done");

    Ok(())
}
