use axum::{extract::Multipart, response::IntoResponse, Json};
use gradient::{extractor::extract_dominant_colors, gradient::generate_stops};
use tokio::time::Duration;

use crate::{
    error::ApiError,
    models::{ExtractColorsResponse, GradientResponse},
};

const MAX_BYTES: usize = 10 * 1024 * 1024; // 10 MB
const NUM_COLORS: usize = 5;
const GRADIENT_STEPS: usize = 20;
const EXTRACT_TIMEOUT: Duration = Duration::from_secs(20);

pub async fn extract_colors(mut multipart: Multipart) -> Result<impl IntoResponse, ApiError> {
    let mut image_bytes: Option<Vec<u8>> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::BadRequest(e.to_string()))?
    {
        if field.name() == Some("image") {
            let data = field
                .bytes()
                .await
                .map_err(|e| ApiError::BadRequest(e.to_string()))?;
            if data.len() > MAX_BYTES {
                return Err(ApiError::PayloadTooLarge);
            }
            image_bytes = Some(data.to_vec());
            break;
        }
    }

    let bytes = image_bytes.ok_or_else(|| ApiError::MissingField("image".into()))?;

    // Validate image format before doing any heavy work.
    let format = image::guess_format(&bytes)
        .map_err(|_| ApiError::ImageError("unrecognized image format".into()))?;
    if !matches!(format, image::ImageFormat::Jpeg | image::ImageFormat::Png) {
        return Err(ApiError::ImageError("only JPEG and PNG are supported".into()));
    }

    let colors = tokio::time::timeout(
        EXTRACT_TIMEOUT,
        tokio::task::spawn_blocking(move || extract_dominant_colors(&bytes, NUM_COLORS)),
    )
    .await
    .map_err(|_| ApiError::ImageError("processing timed out".into()))?
    .map_err(|e| ApiError::ImageError(e.to_string()))?
    .map_err(|e| ApiError::ImageError(e.to_string()))?;

    let dominant_colors: Vec<String> = colors.iter().map(|c| c.to_hex()).collect();
    let stops = generate_stops(&colors, GRADIENT_STEPS);

    Ok(Json(ExtractColorsResponse {
        dominant_colors,
        gradient: GradientResponse { stops },
    }))
}
