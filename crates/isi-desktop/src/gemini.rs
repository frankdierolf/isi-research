//! Google Gemini API client using gemini-rust
//!
//! Implements image-to-image transformation using Gemini 3 Pro Image.

use anyhow::{Context, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use gemini_rust::{Gemini, Model, Part};

use crate::clipboard::ClipboardImage;

/// Gemini API client for image transformation
pub struct GeminiClient {
    client: Gemini,
}

impl GeminiClient {
    /// Create a new Gemini client
    pub fn new(api_key: String) -> Result<Self> {
        let client = Gemini::with_model(&api_key, Model::Gemini3ProImage)
            .context("Failed to create Gemini client")?;

        Ok(Self { client })
    }

    /// Transform an image based on a text instruction
    pub async fn transform_image(
        &self,
        image: &ClipboardImage,
        instruction: &str,
    ) -> Result<ClipboardImage> {
        // Encode image to PNG then base64
        let png_bytes = image.to_png()?;
        let base64_image = BASE64.encode(&png_bytes);

        // Build and execute request
        let response = self
            .client
            .generate_content()
            .with_user_message(format!(
                "Transform this image according to the following instruction: {}",
                instruction
            ))
            .with_inline_data(&base64_image, "image/png")
            .execute()
            .await
            .context("Failed to call Gemini API")?;

        // Extract image from response
        for candidate in response.candidates.iter() {
            if let Some(parts) = &candidate.content.parts {
                for part in parts.iter() {
                    if let Part::InlineData { inline_data, .. } = part {
                        // Decode base64 image
                        let image_bytes = BASE64
                            .decode(&inline_data.data)
                            .context("Failed to decode base64 image from response")?;

                        // Parse the image
                        let transformed_image = ClipboardImage::from_bytes(&image_bytes)
                            .context("Failed to parse transformed image")?;

                        return Ok(transformed_image);
                    }
                }
            }
        }

        anyhow::bail!("No image in Gemini response. The model may not have generated an image.")
    }
}
