mod input;
use input::{ get_input, get_option };
mod floating_image;
use floating_image::{ FloatingImage, ImageDataErrors };
use image::{ io::Reader, DynamicImage, ImageFormat, imageops::FilterType::Triangle, GenericImageView };
use std::io::BufReader;
use std::fs::File;
use std::env;

fn main() -> Result<(), ImageDataErrors> {
    // Backtrace for debugging
    env::set_var("RUST_BACKTRACE", "1");
    // Clear terminal, move cursor to start, reset colors
    print!("\x1B[2J\x1B[1;1H\u{001b}[0m");
    // Get user input
    let operation = get_option("Choose operation ('encrypt' or 'decrypt'): ", vec!["encrypt", "decrypt"], "green");
    let output_path = get_input("Enter path of output image: ", "green");
    let main_image_path = get_input("Enter path of main image: ", "green");
    let (main_image, main_image_format) = find_image_from_path(main_image_path);
    // Decrypt image
    if operation == "decrypt" {
        let mut output = FloatingImage::new(main_image.width(), main_image.height(), output_path);
        println!("Decrypting...");
        let decrypted = decrypt(main_image);
        output.set_data(decrypted)?; 
        println!("Image saved to {}", output.name);
        // Save image
        image::save_buffer_with_format(
            output.name,
            &output.data,
            output.width,
            output.height,
            image::ColorType::Rgba8,
            main_image_format,
        )
        .unwrap();
        return Ok(());
    }
    // Encrypt hidden image into main image
    let hidden_image_path: String;
    hidden_image_path = get_input("Enter path of hidden image: ", "green");
    let (hidden_image, hidden_image_format) = find_image_from_path(hidden_image_path);
    // Image format must be same
    if main_image_format != hidden_image_format {
        return Err(ImageDataErrors::DifferentImageFormats)
    } 
    println!("Scaling...");
    // Scale main image to fit inside max size (957 x 957)
    let main_image = fit_inside((957, 957), &main_image);
    // Scale hidden image to fit inside main image
    let hidden_image = fit_inside(main_image.dimensions(), &hidden_image);
    // Output has same dimensions as main image
    let mut output = FloatingImage::new(main_image.width(), main_image.height(), output_path);
    println!("Encrypting...");
    let encrypted = encrypt(main_image, hidden_image);
    output.set_data(encrypted)?; 
    println!("Image saved to {}", output.name);
    // Save image
    image::save_buffer_with_format(
        output.name,
        &output.data,
        output.width,
        output.height,
        image::ColorType::Rgba8,
        main_image_format,
    )
    .unwrap();
    Ok(())
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader: Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format: ImageFormat = image_reader.format().unwrap();
    let image: DynamicImage = image_reader.decode().unwrap();
    (image, image_format)
}

fn fit_inside(dimensions: (u32, u32), inside_image: &DynamicImage) -> DynamicImage {
    let (width, height) = dimensions;
    return inside_image.resize(width, height, Triangle);
}

fn encrypt(main_image: DynamicImage, hidden_image: DynamicImage) -> Vec<u8> {
    // Image buffers
    let main_vec = main_image.to_rgba8().into_vec();
    let hidden_vec = hidden_image.to_rgba8().into_vec();
    let mut encrypted = vec![0u8; main_vec.len()];
    // Dimensions
    let main_width = main_image.width() as usize;
    let main_height = main_image.height() as usize;
    let hidden_width = hidden_image.width() as usize;
    let hidden_height = hidden_image.height() as usize;

    let mut i;
    for h in 0..main_height {
        for w in 0..main_width {
            // Convert 2d pixel location (w, h) to 1d index in image buffer
            i = (h * main_width + w) * 4;
            if h < hidden_height && w < hidden_width  {
                encrypted.splice(i..=i + 3, encrypt_bits(&main_vec, i, &hidden_vec, (h * hidden_width + w) * 4));
            } else {
                encrypted.splice(i..=i + 3, lose_bits(&main_vec, i));
            }
        }
    }
    return encrypted;
}

fn get_rgba(vec: &Vec<u8>, start: usize) -> Vec<u8> {
    let mut rgba = Vec::new();
    // '..=' Range inclusive of end value
    for i in start..=(start + 3) {
        let val = match vec.get(i) {
            // '*' is dereferencing operator, which allows value of reference to be accessed
            Some(d) => *d,
            None => 0,
        };
        rgba.push(val);
    }
    return rgba;
}

fn encrypt_bits(main_vec: &Vec<u8>, main_start: usize, hidden_vec: &Vec<u8>, hidden_start: usize) -> Vec<u8> {
    let main_pixel = get_rgba(main_vec, main_start);
    let hidden_pixel = get_rgba(hidden_vec, hidden_start);
    let mut encrypted: Vec<u8> = Vec::with_capacity(4);
    for i in 0..3 {
        // Replace last 3 bits of main pixel with first 3 bits of hidden pixel
        encrypted.push((main_pixel[i] & 0b_1111_1000) + ((hidden_pixel[i] & 0b_1110_0000) >> 5));
    }
    let hidden_alpha_channel = if hidden_pixel[3] > 127 { 1 } else { 0 };
    encrypted.push((main_pixel[3] & 0b_1111_1110) + hidden_alpha_channel);
    return encrypted;
}

fn lose_bits(main_vec: &Vec<u8>, main_start: usize) -> Vec<u8> {
    let main_pixel = get_rgba(main_vec, main_start);
    let mut encrypted: Vec<u8> = Vec::with_capacity(4);
    for i in 0..3 {
        // Replace last 3 bits of main pixel with zeros
        encrypted.push((main_pixel[i] & 0b_1111_1000) + ((0 & 0b_1110_0000) >> 5));
    }
    // Set revealed bit as transparent (0 at end)
    encrypted.push(main_pixel[3] & 0b_1111_1110);
    return encrypted;
}

fn decrypt(image: DynamicImage) -> Vec<u8> {
    let vec = image.to_rgba8().into_vec();
    let mut decrypted = vec![0u8; vec.len()];
    // Traverse through image buffer
    let mut i = 0;
    while i < vec.len() - 4 {
        // Each pixel represented by 4 consecutive bytes (32 bits) 
        decrypted.splice(i..=i + 3, decrypt_bits(&vec, i));
        i += 4;
    }
    return decrypted;
}

fn decrypt_bits(vec: &Vec<u8>, start: usize) -> Vec<u8> {
    let pixel = get_rgba(vec, start);
    let mut decrypted: Vec<u8> = Vec::with_capacity(4);
    // RGB
    for i in 0..3 {
        // Move 3 last bits to start
        decrypted.push((pixel[i] & 0b_0000_0111) << 5);
    }
    // Alpha channel (determines transparency)
    let alpha_channel = if (pixel[3] & 0b_0000_0001) == 1 { 255 } else { 0 };
    decrypted.push(alpha_channel);
    return decrypted;
}