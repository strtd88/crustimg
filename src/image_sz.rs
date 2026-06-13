use imagesize::size;
use std::path::Path;

pub fn print_size(path: &Path) -> Result<(), exif::Error> {
    let image_path = Path::new(path);

    // Get the image dimensions
    match size(image_path) {
        Ok(size) => println!("Image dimensions: {}x{}", size.width, size.height),
        Err(why) => println!("Error getting dimensions: {:?}", why),
    }
    Ok(())
}
