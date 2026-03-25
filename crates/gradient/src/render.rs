use image::{ImageOutputFormat, RgbImage};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

use crate::color::Color;

// ── Paradigm ──────────────────────────────────────────────────────────────────

/// How pixel coordinates are mapped to a gradient position (0.0–1.0).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Paradigm {
    /// Left → right (default).
    #[default]
    Linear,
    /// Top-left → bottom-right at 45°.
    Diagonal,
    /// Circle expanding from the center outward.
    Radial,
    /// Mirror: both edges start at 0, center is 1.
    Reflected,
}

// ── Warp ──────────────────────────────────────────────────────────────────────

/// Easing / distortion applied to the gradient position before color lookup.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Warp {
    /// No distortion (default).
    #[default]
    None,
    /// Slow start, fast end (cubic ease-in).
    EaseIn,
    /// Fast start, slow end (cubic ease-out).
    EaseOut,
    /// Slow at both ends — smooth-step curve.
    EaseInOut,
    /// Sine wave: oscillates through the gradient twice.
    Wave,
    /// Zigzag: bounces back and forth through the gradient twice.
    Zigzag,
}

// ── Rendering ─────────────────────────────────────────────────────────────────

/// Render a gradient image as JPEG bytes.
pub fn render_jpeg(
    colors: &[Color],
    width: u32,
    height: u32,
    quality: u8,
    paradigm: Paradigm,
    warp: Warp,
) -> Vec<u8> {
    let mut img = RgbImage::new(width, height);

    let cx = width as f32 / 2.0;
    let cy = height as f32 / 2.0;
    let max_radius = (cx * cx + cy * cy).sqrt();

    let w = (width.saturating_sub(1)).max(1) as f32;
    let h = (height.saturating_sub(1)).max(1) as f32;

    for y in 0..height {
        for x in 0..width {
            let raw_t: f32 = match paradigm {
                Paradigm::Linear => x as f32 / w,
                Paradigm::Diagonal => (x as f32 + y as f32) / (w + h),
                Paradigm::Radial => {
                    let dx = x as f32 - cx;
                    let dy = y as f32 - cy;
                    (dx * dx + dy * dy).sqrt() / max_radius
                }
                Paradigm::Reflected => {
                    let t = x as f32 / w;
                    1.0 - (2.0 * t - 1.0).abs()
                }
            };

            let t = apply_warp(raw_t.clamp(0.0, 1.0), warp);
            let color = sample(colors, t);
            img.put_pixel(x, y, image::Rgb([color.r, color.g, color.b]));
        }
    }

    let mut buf = Cursor::new(Vec::new());
    img.write_to(&mut buf, ImageOutputFormat::Jpeg(quality))
        .expect("JPEG encoding should not fail for a valid RgbImage");
    buf.into_inner()
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn apply_warp(t: f32, warp: Warp) -> f32 {
    match warp {
        Warp::None => t,
        Warp::EaseIn => t * t * t,
        Warp::EaseOut => 1.0 - (1.0 - t).powi(3),
        Warp::EaseInOut => t * t * (3.0 - 2.0 * t), // smoothstep
        Warp::Wave => (t * std::f32::consts::TAU * 2.0).sin() * 0.5 + 0.5,
        Warp::Zigzag => {
            let t = (t * 4.0) % 2.0;
            if t < 1.0 { t } else { 2.0 - t }
        }
    }
}

fn sample(colors: &[Color], t: f32) -> Color {
    if colors.is_empty() {
        return Color::new(0, 0, 0);
    }
    if colors.len() == 1 {
        return colors[0];
    }
    let segments = colors.len() - 1;
    let scaled = t * segments as f32;
    let seg = (scaled as usize).min(segments - 1);
    colors[seg].lerp(colors[seg + 1], scaled - seg as f32)
}
