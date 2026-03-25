use kmeans_colors::get_kmeans;
use palette::{FromColor, IntoColor, Lab, Srgb};
use thiserror::Error;

use crate::color::Color;

#[derive(Debug, Error)]
pub enum ExtractError {
    #[error("failed to decode image: {0}")]
    Decode(#[from] image::ImageError),
    #[error("image has no pixels after resize")]
    Empty,
}

/// Extract `num_colors` dominant colors from raw image bytes (JPEG / PNG).
///
/// The image is down-sampled to 150×150 before clustering for performance.
pub fn extract_dominant_colors(image_bytes: &[u8], num_colors: usize) -> Result<Vec<Color>, ExtractError> {
    let img = image::load_from_memory(image_bytes)?;
    // Down-sample: we only need rough color distribution, not full resolution.
    let img = img.resize(150, 150, image::imageops::FilterType::Lanczos3);
    let rgba = img.to_rgba8();
    let raw = rgba.as_raw();

    if raw.is_empty() {
        return Err(ExtractError::Empty);
    }

    // Convert raw RGBA u8 bytes → Lab f32 for perceptually-uniform clustering.
    // Alpha is discarded; we cluster on RGB only.
    let lab: Vec<Lab> = raw
        .chunks_exact(4)
        .map(|px| {
            let srgb: Srgb<f32> = Srgb::new(px[0], px[1], px[2]).into_format();
            srgb.into_color()
        })
        .collect();

    let result = get_kmeans(num_colors, 20, 5.0, false, &lab, 42);

    let colors = result
        .centroids
        .iter()
        .map(|&lab_color| {
            let srgb: Srgb<f32> = Srgb::from_color(lab_color);
            let srgb: Srgb<u8> = srgb.into_format();
            let (r, g, b) = srgb.into_components();
            Color::new(r, g, b)
        })
        .collect();

    Ok(colors)
}
