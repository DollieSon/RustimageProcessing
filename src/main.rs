use std::env;

fn main() {
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());
}
// Open and save an image
// path directory starts at imageProcessing
fn test2() {
    use image::GenericImageView;

    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open("tests/images/jpg/progressive/cat.jpg").unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    // Write the contents of this image to the Writer in PNG format.
    img.save("test.png").unwrap();
}
