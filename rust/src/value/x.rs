/// Background Colors.
pub struct X;

impl X {
    /// Black Color.
    pub const BLACK: &'static str   = "\x1b[40m";
    /// Red Color.
    pub const RED: &'static str     = "\x1b[41m";
    /// Green Color.
    pub const GREEN: &'static str   = "\x1b[42m";
    /// Yellow Color.
    pub const YELLOW: &'static str  = "\x1b[43m";
    /// Blue Color.
    pub const BLUE: &'static str    = "\x1b[44m";
    /// Magenta Color.
    pub const MAGENTA: &'static str = "\x1b[45m";
    /// Cyan Color.
    pub const CYAN: &'static str    = "\x1b[46m";
    /// White Color.
    pub const WHITE: &'static str   = "\x1b[47m";

    /// RGB Color.
    pub fn rgb(r: u8, g: u8, b: u8) -> String {
        format!("\x1b[48;2;{r};{g};{b}m")
    }
}