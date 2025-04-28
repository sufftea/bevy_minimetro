#![allow(dead_code)]

use bevy::color::Srgba;

pub const PRIMARY: Srgba = Srgba::new(0.2, 0.8, 0.4, 1.0); // Vibrant green
pub const PRIMARY_VARIANT: Srgba = Srgba::new(0.0, 0.6, 0.3, 1.0); // Darker green

pub const SECONDARY: Srgba = Srgba::new(0.6, 0.4, 0.8, 1.0); // Medium purple
pub const SECONDARY_VARIANT: Srgba = Srgba::new(0.4, 0.2, 0.6, 1.0); // Darker purple

pub const BACKGROUND: Srgba = Srgba::new(0.1, 0.1, 0.1, 1.0); // Near black
pub const SURFACE: Srgba = Srgba::new(0.15, 0.15, 0.15, 1.0); // Dark gray

pub const ERROR: Srgba = Srgba::new(0.9, 0.2, 0.2, 1.0); // Red for errors

pub const ON_PRIMARY: Srgba = Srgba::new(0.0, 0.0, 0.0, 1.0); // Black text on primary
pub const ON_SECONDARY: Srgba = Srgba::new(1.0, 1.0, 1.0, 1.0); // White text on secondary
pub const ON_BACKGROUND: Srgba = Srgba::new(0.9, 0.9, 0.9, 1.0); // Off-white on background
pub const ON_SURFACE: Srgba = Srgba::new(0.9, 0.9, 0.9, 1.0); // Off-white on surface
pub const ON_ERROR: Srgba = Srgba::new(1.0, 1.0, 1.0, 1.0); // White on error

// Container colors
pub const CONTAINER_PRIMARY: Srgba = Srgba::new(0.15, 0.5, 0.3, 1.0); // Container with primary influence
pub const CONTAINER_SECONDARY: Srgba = Srgba::new(0.3, 0.2, 0.4, 1.0); // Container with secondary influence

// Additional accent colors
pub const ACCENT_GREEN_BRIGHT: Srgba = Srgba::new(0.3, 0.9, 0.5, 1.0); // Bright green accent
pub const ACCENT_PURPLE_BRIGHT: Srgba = Srgba::new(0.7, 0.5, 0.9, 1.0); // Bright purple accent
pub const ACCENT_GRAY_MEDIUM: Srgba = Srgba::new(0.4, 0.4, 0.4, 1.0); // Medium gray accent

// Gradient endpoints (for creating gradients)
pub const GRADIENT_GREEN_START: Srgba = Srgba::new(0.2, 0.7, 0.4, 1.0);
pub const GRADIENT_GREEN_END: Srgba = Srgba::new(0.0, 0.5, 0.2, 1.0);
pub const GRADIENT_PURPLE_START: Srgba = Srgba::new(0.5, 0.3, 0.7, 1.0);
pub const GRADIENT_PURPLE_END: Srgba = Srgba::new(0.3, 0.1, 0.5, 1.0);
