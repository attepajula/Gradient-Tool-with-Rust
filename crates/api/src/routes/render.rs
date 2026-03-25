use axum::{
    body::Body,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use gradient::{
    color::Color,
    render::{render_jpeg, Paradigm, Warp},
};
use serde::Deserialize;
use tokio::time::Duration;

use crate::error::ApiError;

const RENDER_TIMEOUT: Duration = Duration::from_secs(15);

#[derive(Debug, Deserialize)]
pub struct RenderRequest {
    pub colors: Vec<String>,
    #[serde(default = "default_width")]
    pub width: u32,
    #[serde(default = "default_height")]
    pub height: u32,
    #[serde(default = "default_quality")]
    pub quality: u8,
    #[serde(default)]
    pub paradigm: Paradigm,
    #[serde(default)]
    pub warp: Warp,
}

fn default_width() -> u32 { 800 }
fn default_height() -> u32 { 120 }
fn default_quality() -> u8 { 90 }

pub async fn render(Json(req): Json<RenderRequest>) -> Result<impl IntoResponse, ApiError> {
    if req.colors.is_empty() {
        return Err(ApiError::BadRequest("at least one color is required".into()));
    }
    if req.colors.len() > 32 {
        return Err(ApiError::BadRequest("too many colors (max 32)".into()));
    }
    if req.width == 0 || req.width > 2048 {
        return Err(ApiError::BadRequest("width must be between 1 and 2048".into()));
    }
    if req.height == 0 || req.height > 2048 {
        return Err(ApiError::BadRequest("height must be between 1 and 2048".into()));
    }

    let colors: Vec<Color> = req
        .colors
        .iter()
        .map(|h| Color::from_hex(h).map_err(|e| ApiError::InvalidColor(e.to_string())))
        .collect::<Result<_, _>>()?;

    let (width, height, quality) = (req.width, req.height, req.quality.clamp(1, 100));
    let (paradigm, warp) = (req.paradigm, req.warp);

    let jpeg = tokio::time::timeout(
        RENDER_TIMEOUT,
        tokio::task::spawn_blocking(move || {
            render_jpeg(&colors, width, height, quality, paradigm, warp)
        }),
    )
    .await
    .map_err(|_| ApiError::ImageError("render timed out".into()))?
    .map_err(|e| ApiError::ImageError(e.to_string()))?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/jpeg")
        .header(header::CONTENT_DISPOSITION, "attachment; filename=\"gradient.jpg\"")
        .body(Body::from(jpeg))
        .unwrap())
}
