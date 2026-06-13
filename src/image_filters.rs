extern crate photon_rs;

use photon_rs::PhotonImage;
use photon_rs::effects::pixelize;
use photon_rs::native::{open_image, save_image};
use std::path::Path;

///
///pixelize(&mut img, block_size) — the second parameter controls the size of each pixelated block.
/// A higher value means more visible blocks and stronger pixelation.
///
pub fn pixelation(path: &Path, number: i32) -> Result<(), exif::Error> {
    let mut img = open_image(path).expect("File should open");

    pixelize(&mut img, number);

    save_image(img, "pixelized_image.jpg").expect("File should be saved");

    println!("  Pixelization applied.");
    Ok(())
}
