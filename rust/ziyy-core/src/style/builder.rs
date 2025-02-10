use super::{Condition, Style};

/// Style Builder.
///
/// # Example
/// ```
/// let style = StyleBuilder::new()
///              .bold()
///              .italics()
///              .strike()
///              .under()
///              .finish()
/// ```
#[derive(Default)]
#[repr(transparent)]
pub struct StyleBuilder(Style);

impl StyleBuilder {
    /// Creates a new Style Builder.
    pub fn new() -> Self {
        StyleBuilder::default()
    }

    /// Turns on bold style (turns off dim style).
    pub fn bold(mut self) -> Self {
        self.0.brightness = Condition::A;
        self
    }

    /// Turns on dim style (turns off bold style).
    pub fn dim(mut self) -> Self {
        self.0.brightness = Condition::B;
        self
    }

    /// Turns on italics style.
    pub fn italics(mut self) -> Self {
        self.0.italics = true;
        self
    }

    /// Turns on strike-through.
    pub fn strike(mut self) -> Self {
        self.0.strike = true;
        self
    }

    /// Turns on underline
    pub fn under(mut self) -> Self {
        self.0.under = Condition::A;
        self
    }

    /// Completes the build and returns the [Style].
    pub fn finish(self) -> Style {
        self.0
    }
}
