// main.rs
// Entry point for the simplified crustimg application.
// Handles command-line arguments, image processing, and displaying with viuer.
extern crate photon_rs;

use photon_rs::monochrome;
use photon_rs::native::{open_image, save_image};

// Import necessary crates.
use anyhow::{Context, Result};
use clap::{CommandFactory, Parser}; // For parsing command-line arguments
use image::{GenericImageView, ImageFormat}; // Import GenericImageView for dimensions()
use log::{debug, error, info, trace, warn};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::u32; // For file paths
use viuer::{Config, print_from_file, terminal_size}; // Import terminal_size for fitting image
// Import our custom image operations module.
mod image_display;
mod image_filters;
mod image_ops;
mod image_sz;
mod photon_effects;
mod photon_ops;

/// `crustyimg` - A simple command-line image viewer and editor.
///
/// Crustycrab for a mascot with a camera or some kind of digital imaging device.
///
/// If only an input image is provided, it will display the image in the terminal using viuer.
///
/// If processing options (like --strip or --shrink) and an output path are provided,
/// it will apply the operations, save the new image, and then display the saved output.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
// #[clap(disable_help_flag = true)] // Disables `--help` (but `-h` still works)
struct Cli {
    /// Path to the input image file.
    #[arg(value_name = "INPUT_IMAGE")]
    input: PathBuf,

    /// Path to save the processed output image. If not provided, no processing will occur,
    /// and the input image will simply be displayed.
    /// TODO : make a default directory for output images
    #[arg(short, long, value_name = "OUTPUT_IMAGE")]
    output: Option<PathBuf>,

    /// Shrink the image to a smaller JPEG.
    /// This will resize the image to a maximum dimension of 800px (while maintaining aspect ratio)
    /// and save it as a JPEG with quality 80.
    #[arg(long)]
    shrink: bool,

    /// Print EXIF metadata from the image.
    /// Try to use ratatui for this
    #[arg(long)]
    print_exif: bool,

    /// Display the image.
    ///
    /// Note: Cannot use ratatui for this
    #[arg(long)]
    display: bool,

    /// Do not display the image.
    #[arg(long)]
    nodisplay: bool,

    /// Save the new image.
    ///
    ///
    #[arg(long)]
    save: bool,

    /// filter1 the image.
    /// Here is the description of filter1
    /// This is a processing task
    #[arg(long)]
    filter1: bool,

    /// pixelize the image.
    /// This is a processing task
    #[arg(long)]
    pixelize: bool,

    /// solarize the image.
    #[arg(long)]
    solarize: bool,

    /// solarize the image.
    #[arg(long)]
    sepia: bool,

    /// Display the imagesize.
    /// This is an info task
    /// TODO: Try to use ratatui for this
    #[arg(long)]
    printsize: bool,

    /// Display the imagesize.
    /// This is an info task
    /// TODO: Try to use ratatui for this
    #[arg(long)]
    thumbnail: bool,

    /// Display the imagesize.
    /// This is an info task
    /// TODO: Try to use ratatui for this
    #[arg(long)]
    printthumb: bool,

    /// Display the imagesize.
    /// This is an info task
    /// TODO: Try to use ratatui for this
    #[arg(long)]
    displaymedium: bool,
    #[arg(long)]
    displaysmall: bool,

    /*  Below is TODO:
     *
     *
     *  */
    /// TODO: Crop - this will crop the image
    /// (processing)
    /// This is a processing task
    #[arg(long)]
    crop: bool,

    /// TODO: Remove EXIF metadata from the image.
    #[arg(long)]
    strip_exif: bool,
    // Width
    //   #[arg(short, long, value_name = "width")]
    //   width: u32,
    //   #[arg(short, long, value_name = "height")]
    //   height: u32,

    // Help
    //    #[arg(short = 'H', long, value_name = "help")]
    //    help: bool,
}

/// Main entry point of the application.
#[tokio::main] // Using tokio because viuer might benefit from async, though not strictly required for this simple use.
async fn main() -> Result<()> {
    // Start time
    let start = Instant::now();

    // Parse command-line arguments.
    let cli = Cli::parse();
    Cli::command().print_help().unwrap();

    // Get terminal size to fit the image.
    let (term_width, _) = terminal_size();

    // Configuration for viuer.
    let viuer_config = Config {
        // Set width to terminal width to fit the image.
        // viuer will automatically preserve aspect ratio if only width or height is set.
        width: Some(term_width as u32),
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

    // Determine if processing is requested.
    let needs_processing = cli.output.is_some()
        && (cli.strip_exif
            || cli.shrink
            || cli.filter1
            || cli.pixelize
            || cli.printthumb
            || cli.displaymedium
            || cli.displaysmall
            || cli.thumbnail);
    /*
        if cli.help {
            println!("{}", "help, i need somebody");
            Cli::command().print_help().unwrap();
            std::process::exit(0);
            //Ok(());
        }
    */

    let mut img = image::open(&cli.input)
        .with_context(|| format!("Failed to open input image: {}", cli.input.display()))?;

    println!("Pre-process: needs_processing {}", needs_processing);

    if needs_processing {
        // --- Processing Mode ---

        // make this a debug log
        // println!("Processing image: {}", cli.input.display());

        if cli.filter1 {
            println!("- Applying filter...");
            let mut img = open_image(&cli.input).expect("File should open");

            photon_rs::filters::filter(&mut img, "twenties");

            save_image(img, "new_image.jpg").expect("File should be saved");

            // Apply a sepia effect to the image.
            //monochrome::sepia(&mut img);

            //save_image(img, "sepia_raw_image.png").expect("File should be saved");

            println!("  Filter applied.");
        }
        if cli.pixelize {
            // default pixelization is 10
            // could be a command line argument
            println!("- Applying pixelization...");
            image_filters::pixelation(&cli.input, 10);
            println!("  Pixelization done.");
        }

        if cli.strip_exif {
            // this needs to be tested :D
            println!("- Stripping EXIF data...");
            img = image_ops::re_encode_image_to_strip_metadata(img)
                .context("Failed to strip EXIF data")?;
            println!("  EXIF data stripped (image re-encoded).");
        }

        if cli.shrink {
            // A lot of things can be done here.
            println!("- Shrinking image to JPG (max 800px, quality 80)...");
            // Determine target dimensions for shrinking
            let (width, height) = img.dimensions();
            let max_dim = 800;
            let new_width;
            let new_height;

            if width > max_dim || height > max_dim {
                if width > height {
                    new_width = max_dim;
                    new_height = (height as f32 * (max_dim as f32 / width as f32)) as u32;
                } else {
                    new_height = max_dim;
                    new_width = (width as f32 * (max_dim as f32 / height as f32)) as u32;
                }
            } else {
                new_width = width;
                new_height = height;
            }

            img = image_ops::shrink_to_jpg(img, new_width, new_height, 80)
                .context("Failed to shrink image to JPG")?;
            println!(
                "  Image shrunk. New dimensions: {}x{}",
                new_width, new_height
            );
        }
        if cli.strip_exif {
            println!("TODO: strip EXIF metadata");
            // img = image_ops::strip_exif(img).context("Failed to strip EXIF metadata")?;

            println!("  EXIF metadata stripped");
        }

        if cli.printthumb {
            println!("TODO: print thumbnail");
            // img = image_ops::strip_exif(img).context("Failed to strip EXIF metadata")?;
            image_display::display_as_thumbnail(&cli.input, true);
            println!("  Thumbnail printed");
        }

        // Save the processed image.
        let output_path = cli.output.as_ref().unwrap(); // We know it's Some because needs_processing is true
        let format = ImageFormat::from_path(output_path).unwrap_or(ImageFormat::Jpeg); // Default to JPEG if format can't be inferred.
        image_ops::save_image_to_file(img, output_path, format)
            .with_context(|| format!("Failed to save output image to {}", output_path.display()))?;

        println!("Processed image saved to: {}", output_path.display());

        // Display the *output* image.
        println!("\nDisplaying processed image:");
        //        print_from_file(output_path, &viuer_config).with_context(|| {
        //            format!("Failed to display output image: {}", output_path.display())
        //        })?;
    }

    // filters
    // ---  ---
    //
    if cli.solarize {
        println!("TODO: print printhalf");
        photon_effects::solar(&cli.input, true);
        println!("  printhalf printed");
    }
    if cli.sepia {
        println!("TODO: print sepia");
        photon_effects::sepia(&cli.input, true);
        println!("  sepia printed");
    }
    // if display image
    // --- Viewing Mode ---
    //
    if cli.displaymedium {
        println!("TODO: print printhalf");
        image_display::display_tiny(&cli.input, true);
        println!("  printhalf printed");
    }
    if cli.displaysmall {
        println!("TODO: print printquarter");
        image_display::display_small(&cli.input, true);
        println!("  printhalf printquarter");
    }

    if cli.thumbnail {
        println!("TODO: thumbnail");
        //            photon_ops::thumbnail(&cli.input,&cli.output, true);
        println!("Done thumbnail");
    }
    if cli.display {
        println!("Displaying image: {}", cli.display);
        print_from_file(&cli.input, &viuer_config)
            .with_context(|| format!("Failed to display input image: {}", cli.input.display()))?;
    }
    if cli.printsize {
        println!("Displaying image size: {}", cli.printsize);
        image_sz::print_size(&cli.input);
    }

    if cli.print_exif {
        println!("Displaying exif data: ");
        println!("TODO: strip EXIF metadata {}", cli.print_exif);
        //image_ops::print_exif(&cli.input);
        //print_from_file(output_path, &viuer_config).with_context(|| {
        //format!("Failed to display processed image: {}", output_path.display())
        // })?;
    }

    // Output time taken.
    let elapsed = start.elapsed();

    // Change to info log
    println!("Took {:#?} seconds to process image.", elapsed);
    println!("Took {:#?} seconds to process image.", elapsed);
    println!("Time elapsed: {}", humantime::format_duration(elapsed));

    Ok(())
}
