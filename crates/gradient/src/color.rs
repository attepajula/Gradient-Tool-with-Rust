use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug, Error)]
pub enum ColorError {
    #[error("invalid hex color '{0}': expected 6 hex digits with optional leading '#'")]
    InvalidHex(String),
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_hex(hex: &str) -> Result<Self, ColorError> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err(ColorError::InvalidHex(hex.to_string()));
        }
        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|_| ColorError::InvalidHex(hex.to_string()))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|_| ColorError::InvalidHex(hex.to_string()))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|_| ColorError::InvalidHex(hex.to_string()))?;
        Ok(Self { r, g, b })
    }

    pub fn to_hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    /// Linear interpolation between two colors in sRGB space.
    pub fn lerp(self, other: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            r: lerp_u8(self.r, other.r, t),
            g: lerp_u8(self.g, other.g, t),
            b: lerp_u8(self.b, other.b, t),
        }
    }
}

fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t).round() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_hex() {
        let c = Color::from_hex("#ff8800").unwrap();
        assert_eq!(c.to_hex(), "#ff8800");
    }

    #[test]
    fn lerp_midpoint() {
        let a = Color::new(0, 0, 0);
        let b = Color::new(200, 100, 50);
        let mid = a.lerp(b, 0.5);
        assert_eq!(mid.r, 100);
        assert_eq!(mid.g, 50);
        assert_eq!(mid.b, 25);
    }

    #[test]
    fn invalid_hex_rejected() {
        assert!(Color::from_hex("xyz").is_err());
        assert!(Color::from_hex("#gg0000").is_err());
    }
}
