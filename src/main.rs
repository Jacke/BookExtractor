use pdfium_render::prelude::*;
use image::{DynamicImage, ImageBuffer, Rgba};
use std::fs::{self, File};
use std::path::Path;
use std::io::BufWriter;

fn main() {
    let resource_dir = "resources";
    let pdfium_library_path = "./pdfium-mac-arm64/lib/libpdfium.dylib"; // Укажите правильный путь к libpdfium.dylib

    // Iterate over PDF files in the resources directory
    for entry in fs::read_dir(resource_dir).expect("Could not read resources directory") {
        let entry = entry.expect("Could not read directory entry");
        let path = entry.path();

        // Check if the file is a PDF
        if path.extension().and_then(|ext| ext.to_str()) == Some("pdf") {
            // Open the PDF file using pdfium
            let pdfium = Pdfium::new(Pdfium::bind_to_library(pdfium_library_path).unwrap());
            let document = pdfium.load_pdf_from_file(&path, None).expect("Could not open PDF file");

            // Create a directory for the images
            let output_dir = path.with_extension("");
            fs::create_dir_all(&output_dir).expect("Could not create output directory");

            // Iterate over the pages in the PDF
            for page_number in 0..document.pages().len() {
                let page = document.pages().get(page_number).expect("Could not get page");

                // Get the page size to ensure the full page is rendered
                let page_width = page.width().value;
                let page_height = page.height().value;

                // Render the page to an image
                let config = PdfRenderConfig::new()
                    .set_target_width((page_width * 2.0).round() as u16) // Scale by 2
                    .set_target_height((page_height * 2.0).round() as u16); // Scale by 2

                let bitmap = page.render_with_config(&config).expect("Could not render page");

                // Convert the bitmap to DynamicImage
                let width = bitmap.width() as u32;
                let height = bitmap.height() as u32;
                let buffer: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(width, height, bitmap.as_bytes().to_vec()).expect("Invalid image data");
                let dynamic_image = DynamicImage::ImageRgba8(buffer);

                // Save the image as a JPG file
                let output_path = output_dir.join(format!("page_{}.jpg", page_number + 1));
                let output_file = File::create(&output_path).expect("Could not create output file");
                let mut writer = BufWriter::new(output_file);
                dynamic_image.write_to(&mut writer, image::ImageOutputFormat::Jpeg(80)).expect("Could not write image to file");
            }
        }
    }
}
