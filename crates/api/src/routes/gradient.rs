use axum::{response::IntoResponse, Json};
use gradient::{color::Color, gradient::generate_stops};

use crate::{
    error::ApiError,
    models::{GradientFromColorsRequest, GradientResponse},
};

pub async fn from_colors(
    Json(req): Json<GradientFromColorsRequest>,
) -> Result<impl IntoResponse, ApiError> {
    if req.colors.is_empty() {
        return Err(ApiError::BadRequest("at least one color is required".into()));
    }
    if req.colors.len() > 32 {
        return Err(ApiError::BadRequest("too many colors (max 32)".into()));
    }
    if req.steps < 2 {
        return Err(ApiError::BadRequest("steps must be at least 2".into()));
    }
    if req.steps > 500 {
        return Err(ApiError::BadRequest("steps must not exceed 500".into()));
    }

    let colors: Vec<Color> = req
        .colors
        .iter()
        .map(|h| Color::from_hex(h).map_err(|e| ApiError::InvalidColor(e.to_string())))
        .collect::<Result<_, _>>()?;

    let stops = generate_stops(&colors, req.steps);
    Ok(Json(GradientResponse { stops }))
}
