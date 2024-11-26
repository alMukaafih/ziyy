/// Background Colors.
pub struct X;

impl X {
    /// Black Color.
    pub const BLACK: [u8; 1]   = [40];
    /// Red Color.
    pub const RED: [u8; 1]     = [41];
    /// Green Color.
    pub const GREEN: [u8; 1]   = [42];
    /// Yellow Color.
    pub const YELLOW: [u8; 1]  = [43];
    /// Blue Color.
    pub const BLUE: [u8; 1]    = [44];
    /// Magenta Color.
    pub const MAGENTA: [u8; 1] = [45];
    /// Cyan Color.
    pub const CYAN: [u8; 1]    = [46];
    /// White Color.
    pub const WHITE: [u8; 1]   = [47];
    /// Default Color.
    pub const DEFAULT: [u8; 1] = [49];

    /// RGB Color.
    pub fn rgb(r: u8, g: u8, b: u8) -> [u8; 5] {
        [48, 2, r, g, b]
    }
}