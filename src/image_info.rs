/// print exif data
/// print image info
///
///
use nom_exif::*;


pub fn print_exif(path: &Path) -> Result<(), exif::Error> {
    let file = File::open(path)?;

    let mut parser = MediaParser::new
    let ms = MediaSource::file_path(path)?;

    if ms.has_exif() {
               // Parse the file as an Exif-compatible file
               let mut iter: ExifIter = parser.parse(ms)?;
               // ...
           }
    Ok(())

}


pub fn print_gps_info(path: &Path) -> Result<(), exif::Error> {
    let mut parser = MediaParser::new();

    let ms = MediaSource::file_path(path)?;
    let iter: ExifIter = parser.parse(ms)?;

    let gps_info = iter.parse_gps_info()?.unwrap();
    assert_eq!(gps_info.format_iso6709(), "+43.29013+084.22713+1595.950CRSWGS_84/");
    assert_eq!(gps_info.latitude_ref, 'N');
    assert_eq!(gps_info.longitude_ref, 'E');
    assert_eq!(
        gps_info.latitude,
        [(43, 1), (17, 1), (2446, 100)].into(),
    );
    Ok(())
}
