extern crate photon_rs;
use image::DynamicImage;
use photon_rs::PhotonImage;
use photon_rs::helpers::dyn_image_from_raw;
use photon_rs::native::{open_image, save_image};
use photon_rs::transform::SamplingFilter;
use photon_rs::transform::resize;
use std::env::consts::OS;
use std::path::Path;
use viuer::{Config, print, print_from_file, terminal_size}; // Import terminal_size for fitting image

pub fn display_as_thumbnail(path: &Path, save: bool) {
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

    // Define thumbnail dimensions
    let thumbnail_width = 128;
    let thumbnail_height = 128;

    // Resize the image
    let resized_img: PhotonImage = resize(
        &mut img,
        thumbnail_width,
        thumbnail_height,
        SamplingFilter::Nearest,
    );

    // Save the thumbnail
    save_image(resized_img, "thumbnail.jpg").expect("File should be saved");
    print_from_file("thumbnail.jpg", &config);

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

pub fn display_medium(path: &Path, save: bool) {
    // Open the image
    let mut img = open_image(path).expect("File should open");

    // Get original dimensions
    //let (width, height) = img.get_dimensions();
    let width = img.get_width();
    let height = img.get_height();

    // Calculate new dimensions (50%)
    let new_width = width / 2;
    let new_height = height / 2;

    // Resize the image
    let resized_img: PhotonImage = resize(&img, new_width, new_height, SamplingFilter::Nearest);

    // viuer config — you can tweak it
    let config = Config {
        // viuer won't scale image — it will use the image's actual size
        absolute_offset: false,
        width: None,
        height: None,
        ..Default::default()
    };
    // Save the resized image
    save_image(resized_img, "half-output.jpg").expect("Failed to save image");
    print_from_file("half-output.jpg", &config);

    println!("Image resized and saved as output.jpg");
    //    Ok(())
}

pub fn display_small(path: &Path, save: bool) {
    // Open the image
    let mut img = open_image(path).expect("File should open");

    // Get original dimensions
    //let (width, height) = img.get_dimensions();
    let width = img.get_width();
    let height = img.get_height();

    // Calculate new dimensions (50%)
    let new_width = width / 4;
    let new_height = height / 4;

    // Resize the image
    let resized_img: PhotonImage = resize(&img, new_width, new_height, SamplingFilter::Nearest);

    // viuer config — you can tweak it
    let config = Config {
        // viuer won't scale image — it will use the image's actual size
        absolute_offset: false,
        width: None,
        height: None,
        ..Default::default()
    };
    let conf = viuer::Config {
        width: Some(width / 250),
        height: Some(height / 250),
        //        x: 10,
        //        y: 4,
        ..Default::default()
    };

    // Save the resized image
    save_image(resized_img, "quarter-output.jpg").expect("Failed to save image");
    print_from_file("quarter-output.jpg", &conf);

    println!("Image resized and saved as output.jpg");
    //    Ok(())
}

pub fn display_tiny(path: &Path, save: bool) {
    // Open the image
    let mut img = open_image(path).expect("File should open");

    // Get original dimensions
    //let (width, height) = img.get_dimensions();
    let width = img.get_width();
    let height = img.get_height();

    // Calculate new dimensions (50%)
    let new_width = width / 10;
    let new_height = height / 10;

    // Resize the image
    let resized_img: PhotonImage = resize(&img, new_width, new_height, SamplingFilter::Nearest);
    let conf = viuer::Config {
        width: Some(width / 500),
        height: Some(height / 500),
        //        x: 10,
        //        y: 4,
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
    // Save the resized image
    save_image(resized_img, "by-ten.jpg").expect("Failed to save image");
    print_from_file("by-ten.jpg", &conf);

    println!("Image resized and saved as output.jpg");
    //    Ok(())
}
