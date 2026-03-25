use gradient::gradient::GradientStop;
use serde::{Deserialize, Serialize};

// ── Gradient from colors ──────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct GradientFromColorsRequest {
    /// List of hex color strings, e.g. ["#ff0000", "#0000ff"]
    pub colors: Vec<String>,
    /// Number of output stops (default 10, min 2, max 500)
    #[serde(default = "default_steps")]
    pub steps: usize,
}

fn default_steps() -> usize {
    10
}

#[derive(Debug, Serialize)]
pub struct GradientResponse {
    pub stops: Vec<GradientStop>,
}

// ── Image color extraction ────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct ExtractColorsResponse {
    pub dominant_colors: Vec<String>,
    pub gradient: GradientResponse,
}
