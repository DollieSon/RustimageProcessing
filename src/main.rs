#![allow(unused)]
use std::env;

use egui::ResizeDirection;
use image::{buffer::EnumeratePixelsMut, DynamicImage, Pixel};
use image::{GenericImageView, ImageBuffer, Rgb, Rgba, RgbaImage};
use std::cmp;
fn main() {
    // grayscale(String::from("guads.jpg"));
    let image = image::open("./Results/mercy.jpg").unwrap();
    histogram(image);
    // let inverted = convolutional_Matrix(image.clone());
    // test3();
}
// Open and save an image
// path directory starts at imageProcessing
fn test2() {
    use image::GenericImageView;

    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open("./Images/mercy.jpg").unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    // Write the contents of this image to the Writer in PNG format.
    img.save("test.png").unwrap();
}

// Open iage and make it grayscale and save it
fn test3() {
    use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};

    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let mut img = image::open("./Images/guads.jpg").unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    for (x, y, pixel) in img.as_mut_rgb8().unwrap().enumerate_pixels_mut() {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];
        let gray = (r as f32 * 0.333 + g as f32 * 0.333 + b as f32 * 0.333) as u8;
        pixel[0] = gray;
        pixel[1] = gray;
        pixel[2] = gray;
    }
    img.save("./Results/Sleep.png").unwrap();
}

fn grayscale(image_name: String) {
    use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
    let mut binding = image::open(format!("./Images/{}", image_name)).unwrap();
    //have to seperate binding and image because it de-allocates binding when you try to use it as a mutable reference
    let image = binding.as_mut_rgb8().unwrap();
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];
        let gray = (r as f32 * 0.299 + g as f32 * 0.587 + b as f32 * 0.114) as u8;
        pixel[0] = gray;
        pixel[1] = gray;
        pixel[2] = gray;
    }
    image.save(format!("./Results/{}", image_name)).unwrap();
}
// Opens Image and Flips it then saves it
// image_name: String, name of the image , must be inside Images folder
// direction: String, direction to flip the image, must be "h", "v" or "hv"
fn flip(image_name: String, direction: String) {
    use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
    let mut binding = image::open(format!("./Images/{}", image_name)).unwrap();
    let image = binding.as_mut_rgb8().unwrap();
    let (width, height) = image.dimensions();
    let mut new_image = RgbaImage::new(width, height);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let new_x = match direction.as_str() {
            "h" => width - x - 1,
            "v" => x,
            "hv" => width - x - 1,
            _ => panic!("Invalid direction"),
        };
        let new_y = match direction.as_str() {
            "h" => y,
            "v" => height - y - 1,
            "hv" => height - y - 1,
            _ => panic!("Invalid direction"),
        };
        new_image.put_pixel(new_x, new_y, pixel.to_rgba());
    }
    new_image
        .save_with_format(format!("./Results/{}", image_name), image::ImageFormat::Png)
        .unwrap();
}

//Opens Image and Inverts the colors
fn invert(mut binding: DynamicImage) -> DynamicImage {
    let image = binding.as_mut_rgb8().unwrap();
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        pixel[0] = 255 - pixel[0];
        pixel[1] = 255 - pixel[1];
        pixel[2] = 255 - pixel[2];
    }
    return binding;
}
fn matrix_multiply(color: [[u8; 3]; 3], multiplier: &[[i8; 3]; 3]) -> u8 {
    let mut sum: i128 = 0;
    for x in 0..3 {
        for y in 0..3 {
            sum += (color[x][y] as i128) * (multiplier[x][y] as i128);
        }
    }
    if (sum > u8::MAX as i128) {
        return u8::MAX;
    } else if (sum < 0) {
        return 0;
    }
    return sum as u8;
}

fn convolutional_Matrix(mut binding: DynamicImage /*,matrix: [[i8; 3]; 3]*/) -> DynamicImage {
    let (width, height) = binding.dimensions();
    let image = binding.as_mut_rgb8().unwrap();
    let new_image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(width, height);
    let zero_rgb: Rgb<u8> = Rgb([0, 0, 0]);
    for (x, y, org_pix) in image.enumerate_pixels() {
        let mut red_color: Vec<Vec<u8>> = vec![vec![0; 3]; 3];
        let mut green_color: Vec<Vec<u8>> = vec![vec![0; 3]; 3];
        let mut blue_color: Vec<Vec<u8>> = vec![vec![0; 3]; 3];
        for x_i in -1..2 {
            for y_i in -1..2 {
                let mut color = match image.get_pixel_checked(
                    cmp::max(0, i32::try_from(x).unwrap() + x_i) as u32,
                    cmp::max(0, i32::try_from(y).unwrap() + y_i) as u32,
                ) {
                    Some(pixel) => pixel,
                    None => &zero_rgb,
                };
            }
        }
    }
    return binding;
}

fn histogram(binding: DynamicImage) {
    let mut red_hist: Vec<u128> = vec![0; 256];
    let mut blue_hist: Vec<u128> = vec![0; 256];
    let mut green_hist: Vec<u128> = vec![0; 256];
    let mut gray_hist: Vec<u128> = vec![0; 256];
    for (x, y, pix) in binding.as_rgb8().unwrap().enumerate_pixels() {
        red_hist[pix[0] as usize] += 1;
        blue_hist[pix[1] as usize] += 1;
        green_hist[pix[2] as usize] += 1;
        let mut gray = (pix[0] as u16) + (pix[1] as u16) + (pix[2] as u16);
        gray /= 3;
        let index = usize::try_from(gray).unwrap();
        gray_hist[index] += 1;
    }
    println!("Red Histogram: {:?}\n\n", red_hist);
    println!("Green Histogram: {:?}\n\n", green_hist);
    println!("Blue Histogram: {:?}\n\n", blue_hist);
    println!("Gray Histogram: {:?}\n\n", gray_hist);
}
