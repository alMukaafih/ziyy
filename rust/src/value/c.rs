/// Foreground Colors.
pub struct C;

impl C {
    /// Black Color.
    pub const BLACK: [u8; 1] = [30];
    /// Red Color.
    pub const RED: [u8; 1] = [31];
    /// Green Color.
    pub const GREEN: [u8; 1] = [32];
    /// Yellow Color.
    pub const YELLOW: [u8; 1] = [33];
    /// Blue Color.
    pub const BLUE: [u8; 1] = [34];
    /// Magenta Color.
    pub const MAGENTA: [u8; 1] = [35];
    /// Cyan Color.
    pub const CYAN: [u8; 1] = [36];
    /// White Color.
    pub const WHITE: [u8; 1] = [37];
    /// Default Color.
    pub const DEFAULT: [u8; 1] = [39];

    /// RGB Color.
    pub fn rgb(r: u8, g: u8, b: u8) -> [u8; 5] {
        [38, 2, r, g, b]
    }
}
