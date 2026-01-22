//! Clipboard image operations using arboard
//!
//! Handles reading and writing images to/from the system clipboard.

use anyhow::{Context, Result};
use arboard::{Clipboard, ImageData};
use image::{DynamicImage, ImageFormat, RgbaImage};
use std::borrow::Cow;
use std::io::Cursor;

/// Image data from clipboard
#[derive(Debug, Clone)]
pub struct ClipboardImage {
    pub width: usize,
    pub height: usize,
    pub rgba_data: Vec<u8>,
}

impl ClipboardImage {
    /// Create from raw RGBA data
    pub fn new(width: usize, height: usize, rgba_data: Vec<u8>) -> Self {
        Self {
            width,
            height,
            rgba_data,
        }
    }

    /// Encode image to PNG bytes
    pub fn to_png(&self) -> Result<Vec<u8>> {
        let img = RgbaImage::from_raw(self.width as u32, self.height as u32, self.rgba_data.clone())
            .context("Failed to create image from RGBA data")?;

        let mut png_bytes = Vec::new();
        let mut cursor = Cursor::new(&mut png_bytes);
        DynamicImage::ImageRgba8(img)
            .write_to(&mut cursor, ImageFormat::Png)
            .context("Failed to encode PNG")?;

        Ok(png_bytes)
    }

    /// Create from PNG bytes
    pub fn from_png(png_bytes: &[u8]) -> Result<Self> {
        let img = image::load_from_memory_with_format(png_bytes, ImageFormat::Png)
            .context("Failed to decode PNG")?;

        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();

        Ok(Self {
            width: width as usize,
            height: height as usize,
            rgba_data: rgba.into_raw(),
        })
    }

    /// Create from any image bytes (auto-detect format)
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let img = image::load_from_memory(bytes).context("Failed to decode image")?;

        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();

        Ok(Self {
            width: width as usize,
            height: height as usize,
            rgba_data: rgba.into_raw(),
        })
    }
}

/// Read image from clipboard
pub fn read_image_from_clipboard() -> Result<ClipboardImage> {
    let mut clipboard = Clipboard::new().context("Failed to access clipboard")?;

    let img_data = clipboard
        .get_image()
        .context("No image in clipboard. Copy an image first.")?;

    Ok(ClipboardImage {
        width: img_data.width,
        height: img_data.height,
        rgba_data: img_data.bytes.into_owned(),
    })
}

/// Write image to clipboard
pub fn write_image_to_clipboard(image: &ClipboardImage) -> Result<()> {
    let mut clipboard = Clipboard::new().context("Failed to access clipboard")?;

    let img_data = ImageData {
        width: image.width,
        height: image.height,
        bytes: Cow::Borrowed(&image.rgba_data),
    };

    clipboard
        .set_image(img_data)
        .context("Failed to write image to clipboard")?;

    Ok(())
}

/// Check if clipboard contains an image
pub fn has_image_in_clipboard() -> bool {
    Clipboard::new()
        .ok()
        .and_then(|mut cb| cb.get_image().ok())
        .is_some()
}
