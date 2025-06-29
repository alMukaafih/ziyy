#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum State {
    #[default]
    None,
    A,
    B,
    AB,
    BA,
    E,
}

impl State {
    pub fn is_some(&self) -> bool {
        !matches!(self, State::None)
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
}
