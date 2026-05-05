use std::process;

use arboard::Clipboard;
use image::{DynamicImage, ImageBuffer, Rgba};
use rxing::{helpers, DecodeHints};

fn decode_qr(img: DynamicImage) -> Vec<String> {
    let mut hints = DecodeHints::default();
    hints.TryHarder = Some(true);

    match helpers::detect_multiple_in_image_with_hints(img, &mut hints) {
        Ok(results) => results.into_iter().map(|r| r.getText().to_string()).collect(),
        Err(_) => vec![],
    }
}

fn from_clipboard() -> Result<DynamicImage, String> {
    let mut clipboard = Clipboard::new().map_err(|e| format!("clipboard error: {e}"))?;
    let data = clipboard
        .get_image()
        .map_err(|_| "no image found in clipboard — copy an image first".to_string())?;

    let buf: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(data.width as u32, data.height as u32, data.bytes.into_owned())
            .ok_or("failed to build image from clipboard bytes")?;

    Ok(DynamicImage::ImageRgba8(buf))
}

fn from_file(path: &str) -> Result<DynamicImage, String> {
    image::open(path).map_err(|e| format!("failed to open '{path}': {e}"))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let img = match args.get(1).map(String::as_str) {
        Some("--help") | Some("-h") => {
            eprintln!("usage: qr-paste [image-file]");
            eprintln!("  no argument  — read image from clipboard");
            eprintln!("  image-file   — read image from file");
            process::exit(0);
        }
        Some(path) => from_file(path),
        None => from_clipboard(),
    };

    let img = match img {
        Ok(i) => i,
        Err(e) => {
            eprintln!("error: {e}");
            process::exit(1);
        }
    };

    let results = decode_qr(img);

    if results.is_empty() {
        eprintln!("no QR codes found");
        process::exit(1);
    }

    for content in results {
        println!("{content}");
    }
}
