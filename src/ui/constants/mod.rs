use eframe::epaint;
use epaint::Color32;

/// Enum representing different UI themes.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
    Summer,
    Autumn,
    Winter,
    Spring,
}

impl Theme {
    /// Returns the background color for the selected theme.
    pub fn background_color(&self) -> Color32 {
        match self {
            Theme::Dark => Color32::BLACK,
            Theme::Light => Color32::WHITE,
            Theme::Summer => Color32::from_rgb(255, 165, 0),  // Orange
            Theme::Autumn => Color32::from_rgb(139, 69, 19),  // Brown
            Theme::Winter => Color32::from_rgb(173, 216, 230), // Light Blue
            Theme::Spring => Color32::from_rgb(0, 209, 35), // Light Green
        }
    }

    /// Returns the text color for the selected theme.
    pub fn text_color(&self) -> Color32 {
        match self {
            Theme::Dark => Color32::WHITE,
            Theme::Light => Color32::BLACK,
            _ => Color32::BLACK, // Default black text for non-dark themes
        }
    }

    /// Returns the bar color for the selected theme.
    pub fn bar_color(&self) -> Color32 {
        match self {
            Theme::Dark => Color32::GRAY,         // Sivo
            Theme::Light => Color32::GOLD,        // Zlatno
            Theme::Winter => Color32::from_rgb(0, 0, 139),    // Dark Blue
            Theme::Spring => Color32::from_rgb(0, 100, 0),    // Dark Green
            Theme::Summer => Color32::RED,        // Crveno
            Theme::Autumn => Color32::BLACK,      // Crno
        }
    }
}


/// The minimum value for the generated random vector elements.
pub const FLOOR: usize = 1;

/// The maximum value for the generated random vector elements.
pub const CEIL: usize = 21;

/// The size of the random vector to be generated.
pub const VECTOR_SIZE: usize = 100;
