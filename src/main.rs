mod args;
use std::{fs::File, io::BufReader};

use args::Args;
use image::{io::Reader, DynamicImage, ImageFormat};

#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormats,
}

fn main() -> Result<(), ImageDataErrors> {
    let args = Args::new();
    let (image1, image_format1) = find_image_from_path(args.image1);
    let (image2, image_format2) = find_image_from_path(args.image2);

    if image_format1 != image_format2 {
        return Err(ImageDataErrors::DifferentImageFormats);
    }
    Ok(())
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();
    (image, image_format)
}

fn get_smallest_dimension(dim1: (u32, u32), dim2: (u32, u32)) -> (u32, u32) {
    let pix1 = dim1.0 * dim1.1;
    let pix2 = dim2.0 * dim2.1;

    if pix1 > pix2 {
        dim1
    } else {
        dim2
    }
}
