

### List of imagemagick or convert or 
* convert image.jpg -resize 1024x768\! output_file.jpg
* convert image.jpg -quality 75 output_file.jpg
* convert output_montage.jpg output_montage.pdf
* montage image1.jpg image2.jpg image3.jpg image4.jpg output_montage.jpg

### references
* https://usage.imagemagick.org/basics/
* https://contactsunny.medium.com/a-few-basic-but-powerful-imagemagick-commands-b5809b0a1076

Use the magick mogrify program to resize an image, blur, crop, despeckle, dither, draw on, flip, join, re-sample, and much more. This tool is similar to magick except that the original image file is overwritten (unless you change the file suffix with the -format option) with any changes you request.


what are the most used imagemagick commands?
Some of the most frequently used ImageMagick commands and options include: 
magick: The main command-line tool in ImageMagick 7+ for image manipulation and conversion.
convert: Used to convert images between formats and apply various modifications.
mogrify: Allows for batch processing of images, modifying files in place or outputting to a different directory.
identify: Displays information about an image, such as its format and characteristics.
montage: Used to create composite images, often arrays of thumbnails.
composite: Overlays images in various ways, for example, to add watermarks or create layered effects. 
Common Options:
-resize: Used with convert or mogrify to change the dimensions of an image, often while maintaining aspect ratio.
-format: Specifies the output image format.
-quality: Sets the compression level for output formats like JPEG and PNG.
-strip: Removes extra non-graphical metadata, like EXIF data, to reduce file size.
-gravity: Controls placement of text or other elements when annotating or compositing images.
-pointsize: Sets the font size for text annotations.
-annotate: Adds text annotations to images.
-rotate: Rotates images by a specified number of degrees.
-append / +append: Used to combine images vertically (-append) or horizontally (+append). 
These are just a few of the many commands and options available in ImageMagick. The specific commands and options you use will depend on the image manipulation tasks you need to perform
