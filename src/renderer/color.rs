use ratatui::style::Color;

macro_rules! rgb {
	($r:expr, $g:expr, $b:expr) => {
		Color::Rgb($r, $g, $b)
	};
}

// Accents
pub const ACCENT_PRIMARY: Color = TEAL;
pub const ACCENT_SECONDARY: Color = GOLD;

// Highlights
pub const HIGHLIGHT_PRIMARY: Color = rgb!(255, 198, 95);
pub const HIGHLIGHT_SUCCESS: Color = rgb!(78, 218, 168);
pub const HIGHLIGHT_INFO: Color = rgb!(120, 180, 255);
pub const HIGHLIGHT_DANGER: Color = rgb!(240, 100, 130);
pub const HIGHLIGHT_SPECIAL: Color = rgb!(178, 140, 255);

// Layers
pub const LAYER_BASE: Color = rgb!(13, 12, 22);
pub const LAYER_SURFACE: Color = rgb!(20, 18, 34);
pub const LAYER_RAISED: Color = rgb!(28, 26, 46);
pub const LAYER_OVERLAY: Color = rgb!(38, 35, 60);
pub const LAYER_SELECT: Color = rgb!(38, 68, 118);

// Text
pub const TEXT_PRIMARY: Color = rgb!(225, 222, 245);
pub const TEXT_SECONDARY: Color = rgb!(150, 145, 180);
pub const TEXT_FAINT: Color = rgb!(88, 84, 112);

// Accents
pub const TEAL: Color = rgb!(78, 218, 197);
pub const GOLD: Color = rgb!(255, 198, 95);
pub const VIOLET: Color = rgb!(178, 140, 255);
pub const ROSE: Color = rgb!(240, 100, 130);

// Status
pub const STATUS_OK: Color = TEAL;
pub const STATUS_WARN: Color = GOLD;
pub const STATUS_ERROR: Color = ROSE;
pub const STATUS_INFO: Color = VIOLET;

// Borders
pub const BORDER: Color = rgb!(65, 60, 95);
pub const BORDER_FOCUS: Color = TEAL;
pub const BORDER_ACTIVE: Color = VIOLET;
