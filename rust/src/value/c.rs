/// Foreground Colors.
pub struct C;

impl C {
    /// Black Color.
    pub const BLACK: &'static str   = "\x1b[30m";
    /// Red Color.
    pub const RED: &'static str     = "\x1b[31m";
    /// Green Color.
    pub const GREEN: &'static str   = "\x1b[32m";
    /// Yellow Color.
    pub const YELLOW: &'static str  = "\x1b[33m";
    /// Blue Color.
    pub const BLUE: &'static str    = "\x1b[34m";
    /// Magenta Color.
    pub const MAGENTA: &'static str = "\x1b[35m";
    /// Cyan Color.
    pub const CYAN: &'static str    = "\x1b[36m";
    /// White Color.
    pub const WHITE: &'static str   = "\x1b[37m";

    /// RGB Color.
    pub fn rgb(r: u8, g: u8, b: u8) -> String {
        format!("\x1b[38;2;{r};{g};{b}m")
    }
}