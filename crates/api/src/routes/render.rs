use axum::{
    body::Body,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use gradient::{
    color::Color,
    render::{render_jpeg, GradientPoints, Paradigm, Warp},
};
use serde::Deserialize;
use tokio::time::Duration;

use crate::error::ApiError;

const RENDER_TIMEOUT: Duration = Duration::from_secs(15);

#[derive(Debug, Deserialize)]
pub struct StopInput {
    pub position: f32,
    pub hex: String,
}

#[derive(Debug, Deserialize)]
pub struct RenderRequest {
    #[serde(default)]
    pub colors: Vec<String>,
    pub stops: Option<Vec<StopInput>>,
    /// Normalized [x, y] — gradient start / radial center.
    pub point_a: Option<[f32; 2]>,
    /// Normalized [x, y] — gradient end / radial edge.
    pub point_b: Option<[f32; 2]>,
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
    #[serde(default)]
    pub noise: f32,
}

fn default_width() -> u32 { 800 }
fn default_height() -> u32 { 120 }
fn default_quality() -> u8 { 90 }

pub async fn render(Json(req): Json<RenderRequest>) -> Result<impl IntoResponse, ApiError> {
    let positioned: Vec<(f32, Color)> = match req.stops.filter(|s| !s.is_empty()) {
        Some(stops) => {
            if stops.len() > 32 {
                return Err(ApiError::BadRequest("too many stops (max 32)".into()));
            }
            let mut result: Vec<(f32, Color)> = stops
                .iter()
                .map(|s| {
                    let c = Color::from_hex(&s.hex)
                        .map_err(|e| ApiError::InvalidColor(e.to_string()))?;
                    Ok((s.position.clamp(0.0, 1.0), c))
                })
                .collect::<Result<_, _>>()?;
            result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            result
        }
        None => {
            if req.colors.is_empty() {
                return Err(ApiError::BadRequest("at least one color or stop is required".into()));
            }
            if req.colors.len() > 32 {
                return Err(ApiError::BadRequest("too many colors (max 32)".into()));
            }
            let colors: Vec<Color> = req
                .colors
                .iter()
                .map(|h| Color::from_hex(h).map_err(|e| ApiError::InvalidColor(e.to_string())))
                .collect::<Result<_, _>>()?;
            let n = colors.len();
            colors
                .into_iter()
                .enumerate()
                .map(|(i, c)| {
                    let pos = if n == 1 { 0.0 } else { i as f32 / (n - 1) as f32 };
                    (pos, c)
                })
                .collect()
        }
    };

    if req.width == 0 || req.width > 4096 {
        return Err(ApiError::BadRequest("width must be between 1 and 4096".into()));
    }
    if req.height == 0 || req.height > 4096 {
        return Err(ApiError::BadRequest("height must be between 1 and 4096".into()));
    }

    let paradigm = req.paradigm;
    let defaults = GradientPoints::default_for(paradigm);
    let points = GradientPoints {
        ax: req.point_a.map(|p| p[0]).unwrap_or(defaults.ax),
        ay: req.point_a.map(|p| p[1]).unwrap_or(defaults.ay),
        bx: req.point_b.map(|p| p[0]).unwrap_or(defaults.bx),
        by: req.point_b.map(|p| p[1]).unwrap_or(defaults.by),
    };

    let (width, height, quality) = (req.width, req.height, req.quality.clamp(1, 100));
    let warp = req.warp;
    let noise = req.noise.clamp(0.0, 1.0);

    let jpeg = tokio::time::timeout(
        RENDER_TIMEOUT,
        tokio::task::spawn_blocking(move || {
            render_jpeg(&positioned, width, height, quality, paradigm, warp, points, noise)
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
