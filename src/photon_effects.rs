// For example, to colorize an image of type `PhotonImage`:
extern crate photon_rs;
use image::DynamicImage;
use photon_rs::PhotonImage;
use photon_rs::conv::sobel_global;
use photon_rs::effects::colorize;
use photon_rs::effects::solarize;
use photon_rs::helpers::dyn_image_from_raw;
use photon_rs::monochrome;
use photon_rs::native::{open_image, save_image};
use photon_rs::transform::SamplingFilter;
use photon_rs::transform::resize;
use std::env::consts::OS;
use std::path::Path;
use viuer::{Config, print, print_from_file, terminal_size}; // Import terminal_size for fitting image

///
/// Add filters
/// https://silvia-odwyer.github.io/photon/demo.html
/// https://silvia-odwyer.github.io/photon/docs/photon/all.html
/// Greyscale
/// sepia
/// blue
/// red channel
///
///
///
///

pub fn solar(path: &Path, save: bool) {
    let show = true;
    // Open the image
    let mut img = open_image(path).expect("File should open");

    let (term_width, _) = terminal_size();
    // Configuration for viuer.
    let viuer_config = Config {
        // Set width to terminal width to fit the image.
        // viuer will automatically preserve aspect ratio if only width or height is set.
        width: Some(128 as u32),
        // Restore cursor position after printing the image.
        restore_cursor: true,
        // Make background transparent (if your terminal supports it).
        transparent: true,
        // Only print from x offset.
        x: 0,
        // Only print from y offset.
        y: 0,
        ..Default::default()
    };

    // viuer config — you can tweak it
    let config = Config {
        // viuer won't scale image — it will use the image's actual size
        absolute_offset: false,
        width: None,
        height: None,
        ..Default::default()
    };

    // solarize the image
    solarize(&mut img);

    // Save the thumbnail
    if save {
        save_image(img, "solarized_img.jpg").expect("File should be saved");
    }
    if show {
        print_from_file("solarized_img.jpg", &config);
    }
    println!("Displaying image path: {}", path.display());
    //let img = image::DynamicImage::ImageRgba8(image::RgbaImage::new(20, 10));
    //    let dynamic_image: DynamicImage = dyn_image_from_raw(&resized_img);
    //
    //print(&dynamic_image, &viuer_config).expect("Image printing failed.");

    //print_from_file(&cli.input, &viuer_config)
    //    .with_context(|| format!("Failed to display input image: {}", cli.input.display()))?;
    //
    //    Ok(())
}

pub fn sobelize(path: &Path, save: bool) {
    let show = true;
    // Open the image
    let mut img = open_image(path).expect("File should open");

    let (term_width, _) = terminal_size();
    // Configuration for viuer.
    let viuer_config = Config {
        // Set width to terminal width to fit the image.
        // viuer will automatically preserve aspect ratio if only width or height is set.
        width: Some(128 as u32),
        // Restore cursor position after printing the image.
        restore_cursor: true,
        // Make background transparent (if your terminal supports it).
        transparent: true,
        // Only print from x offset.
        x: 0,
        // Only print from y offset.
        y: 0,
        ..Default::default()
    };

    // viuer config — you can tweak it
    let config = Config {
        // viuer won't scale image — it will use the image's actual size
        absolute_offset: false,
        width: None,
        height: None,
        ..Default::default()
    };

    // sobelize the image
    sobel_global(&mut img);

    // Save the thumbnail
    if save {
        save_image(img, "sobelize_img.jpg").expect("File should be saved");
    }
    if show {
        print_from_file("sobelize_img.jpg", &config);
    }
    println!("Displaying image path: {}", path.display());
    //let img = image::DynamicImage::ImageRgba8(image::RgbaImage::new(20, 10));
    //    let dynamic_image: DynamicImage = dyn_image_from_raw(&resized_img);
    //
    //print(&dynamic_image, &viuer_config).expect("Image printing failed.");

    //print_from_file(&cli.input, &viuer_config)
    //    .with_context(|| format!("Failed to display input image: {}", cli.input.display()))?;
    //
    //    Ok(())
}

pub fn sepia(path: &Path, save: bool) {
    let show = true;
    // Open the image (a PhotonImage is returned)
    let mut img = open_image(path).expect("File should open");

    let (term_width, _) = terminal_size();
    // Configuration for viuer.
    let viuer_config = Config {
        // Set width to terminal width to fit the image.
        // viuer will automatically preserve aspect ratio if only width or height is set.
        width: Some(128 as u32),
        // Restore cursor position after printing the image.
        restore_cursor: true,
        // Make background transparent (if your terminal supports it).
        transparent: true,
        // Only print from x offset.
        x: 0,
        // Only print from y offset.
        y: 0,
        ..Default::default()
    };

    // viuer config — you can tweak it
    let config = Config {
        // viuer won't scale image — it will use the image's actual size
        absolute_offset: false,
        width: None,
        height: None,
        ..Default::default()
    };
    // Apply a sepia effect to the image.
    monochrome::sepia(&mut img);

    // Save the thumbnail
    if save {
        save_image(img, "sepia.png").expect("File should be saved");
    }
    if show {
        print_from_file("sepia.png", &config);
    }
    println!("Displaying image path: {}", path.display());
}
