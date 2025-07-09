use crate::parser::color::Color;
pub use options::{AnsiOptions, Effect};
pub use state::State;
use std::fmt::{Debug, Display, Write};
use std::io::Write as _;
use std::ops::{Add, AddAssign, Not, Sub, SubAssign};

mod options;
mod state;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Ansi {
    pub(crate) style: u32,
    colors: [Color; 2],
}

impl Default for Ansi {
    fn default() -> Self {
        Self::new()
    }
}

impl Ansi {
    pub fn new() -> Self {
        Ansi {
            style: 0,
            colors: [const { Color::new() }; 2],
        }
    }

    pub fn with(options: AnsiOptions) -> Self {
        let mut ansi = Ansi::new();
        ansi.set_brightness(options.brightness);
        ansi.set_under(options.under);
        ansi.set_blink(options.blink);
        ansi.set_hidden(options.hidden);
        ansi.set_italics(options.italics);
        ansi.set_negative(options.negetive);
        ansi.set_strike(options.strike);
        ansi.set_fg_color(options.fg_color);
        ansi.set_bg_color(options.bg_color);

        ansi
    }
}

fn get_style(style: &u32, offset: u32) -> bool {
    let n = (style >> (Ansi::L - offset)) & 1;
    n == 1
}

fn set_style(style: &mut u32, offset: u32, value: bool) {
    if value {
        *style |= 1 << (Ansi::L - offset);
    } else if get_style(style, offset) {
        *style ^= 1 << (Ansi::L - offset);
    }
}

macro_rules! impl_ansi {
    (
        $( ( $i:expr, $set_x:tt, $x:tt ) ),*;
        $( ( $j:expr, $set_y:tt, $y:tt ) ),*;
        $( ( $k:expr, $set_z:tt, $z:tt ) ),*
    ) => {
        impl Ansi {
            const L: u32 = 31;

        $(
            pub fn $set_x(&mut self, value: State) {
                match value {
                    State::None => {
                        set_style(&mut self.style, $i, false);
                        set_style(&mut self.style, $i + 1, false);
                        set_style(&mut self.style, $i + 2, false);
                        set_style(&mut self.style, $i + 3, false);
                    }
                    State::A => {
                        set_style(&mut self.style, $i, true);
                        set_style(&mut self.style, $i + 1, false);
                        set_style(&mut self.style, $i + 2, false);
                        set_style(&mut self.style, $i + 3, false);
                    }
                    State::B => {
                        set_style(&mut self.style, $i, false);
                        set_style(&mut self.style, $i + 1, true);
                        set_style(&mut self.style, $i + 2, false);
                        set_style(&mut self.style, $i + 3, false);
                    }
                    State::AB => {
                        set_style(&mut self.style, $i, false);
                        set_style(&mut self.style, $i + 1, true);
                        set_style(&mut self.style, $i + 2, true);
                        set_style(&mut self.style, $i + 3, false);
                    }
                    State::BA => {
                        set_style(&mut self.style, $i, true);
                        set_style(&mut self.style, $i + 1, false);
                        set_style(&mut self.style, $i + 2, false);
                        set_style(&mut self.style, $i + 3, true);
                    }
                    State::E => {
                        set_style(&mut self.style, $i, true);
                        set_style(&mut self.style, $i + 1, true);
                        set_style(&mut self.style, $i + 2, true);
                        set_style(&mut self.style, $i + 3, true);
                    }
                }
            }

            pub fn $x(&self) -> State {
                match (
                    get_style(&self.style, $i),
                    get_style(&self.style, $i + 1),
                    get_style(&self.style, $i + 2),
                    get_style(&self.style, $i +3)) {
                        (false, false, false, false) => State::None,
                        (true, false, false, false) => State::A,
                        (false, true, false, false) => State::B,
                        (false, true, true, false) => State::AB,
                        (true, false, false, true) => State::BA,
                        (true, true, true, true) => State::E,
                        _ => panic!()
                }
            }
        )*

        $(
            pub fn $set_y(&mut self, value: Effect) {
                match value {
                    Effect::None => {
                        set_style(&mut self.style, $j, false);
                        set_style(&mut self.style, $j + 15, false);
                    }
                    Effect::Apply =>  {
                        set_style(&mut self.style, $j, true);
                        set_style(&mut self.style, $j + 15, false);
                    },
                    Effect::Clear => {
                        set_style(&mut self.style, $j, false);
                        set_style(&mut self.style, $j + 15, true);
                    }
                }

            }

            pub fn $y(&self) -> Effect {
                match (get_style(&self.style, $j), get_style(&self.style, $j + 15)) {
                    (false, false) => Effect::None,
                    (true, false) => Effect::Apply,
                    (false, true) => Effect::Clear,
                    _ => panic!()
                }
            }
        )*

        $(
            pub fn $set_z(&mut self, value: Color) {
                self.colors[$k] = value;
            }

            pub fn $z(&self) -> &Color {
                &self.colors[$k]
            }
        )*
        }
    };
}

impl_ansi![
    (0, set_brightness, brightness),
    (4, set_under, under);

    (8, set_blink, blink),
    (9, set_hidden, hidden),
    (10, set_italics, italics),
    (11, set_negative, negative),
    (12, set_strike, strike);

    (0, set_fg_color, fg_color),
    (1, set_bg_color, bg_color)
];

impl Debug for Ansi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ansi")
            .field("brightness", &self.brightness())
            .field("under", &self.under())
            .field("blink", &self.blink())
            .field("hidden", &self.hidden())
            .field("italics", &self.italics())
            .field("negative", &self.negative())
            .field("strike", &self.strike())
            .field("fg_color", self.fg_color())
            .field("bg_color", self.bg_color())
            .finish()
    }
}

impl Display for Ansi {
    #[allow(clippy::recursive_format_impl)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            return f.write_fmt(format_args!("\"{}\"", self.to_string().escape_debug()));
        }

        let mut buf = Vec::with_capacity(128);
        let _ = buf.write(b"\x1b[");
        macro_rules! write_prop_style {
            ( $f:tt, $a:expr, $e:expr ) => {
                match self.$f() {
                    Effect::None => {}
                    Effect::Apply => {
                        let _ = buf.write($a);
                    }
                    Effect::Clear => {
                        let _ = buf.write($e);
                    }
                }
            };
        }

        macro_rules! write_prop_state {
            ( $f:tt, $a:expr, $b:expr, $e:expr ) => {
                match self.$f() {
                    State::None => {}
                    State::A => {
                        let _ = buf.write($a);
                    }
                    State::B => {
                        let _ = buf.write($b);
                    }
                    State::AB => {
                        let _ = buf.write($e);
                        let _ = buf.write($b);
                    }
                    State::BA => {
                        let _ = buf.write($e);
                        let _ = buf.write($a);
                    }
                    State::E => {
                        let _ = buf.write($e);
                    }
                }
            };
        }

        write_prop_state!(brightness, b"1;", b"2;", b"22;");
        write_prop_state!(under, b"4;", b"21;", b"24;");

        write_prop_style!(italics, b"3;", b"23;");
        write_prop_style!(blink, b"5;", b"25;");
        write_prop_style!(negative, b"7;", b"27;");
        write_prop_style!(hidden, b"8;", b"28;");
        write_prop_style!(strike, b"9;", b"29;");

        let _ = buf.write(self.fg_color().to_string().as_bytes());
        let _ = buf.write(self.bg_color().to_string().as_bytes());

        if buf[buf.len() - 1] == b';' {
            buf.pop();
        }

        buf.push(b'm');

        if buf.len() == 3 {
            buf.clear();
        }

        for ch in buf {
            f.write_char(ch as char)?; // all in ASCII range
        }

        Ok(())
    }
}

pub fn merge(lhs: State, rhs: State) -> State {
    match (lhs, rhs) {
        (State::None, rhs) => rhs,
        (lhs, State::None) => lhs,
        (State::E, rhs) => rhs,
        (_, State::E) => State::E,
        (State::A | State::BA, State::A) => State::A,
        (State::A | State::BA, State::B) => State::AB,
        (State::B | State::AB, State::A) => State::BA,
        (State::B | State::AB, State::B) => State::B,

        (_, rhs) => rhs,
    }
}

fn add(lhs: Effect, rhs: Effect) -> Effect {
    match (lhs, rhs) {
        (Effect::None, Effect::None) => Effect::None,
        (Effect::None, Effect::Apply) => Effect::Apply,
        (Effect::None, Effect::Clear) => Effect::Clear,
        (Effect::Apply, Effect::None) => Effect::Apply,
        (Effect::Apply, Effect::Apply) => Effect::Apply,
        (Effect::Apply, Effect::Clear) => Effect::Clear,
        (Effect::Clear, Effect::None) => Effect::Clear,
        (Effect::Clear, Effect::Apply) => Effect::Apply,
        (Effect::Clear, Effect::Clear) => Effect::Clear,
    }
}

impl AddAssign for Ansi {
    /// Add two Ansi styles together.
    fn add_assign(&mut self, rhs: Self) {
        self.set_brightness(merge(self.brightness(), rhs.brightness()));
        self.set_under(merge(self.under(), rhs.under()));

        self.set_blink(add(self.blink(), rhs.blink()));
        self.set_hidden(add(self.hidden(), rhs.hidden()));
        self.set_italics(add(self.italics(), rhs.italics()));
        self.set_negative(add(self.negative(), rhs.negative()));
        self.set_strike(add(self.strike(), rhs.strike()));

        self.colors[0] += rhs.colors[0].clone();
        self.colors[1] += rhs.colors[1].clone();
    }
}

impl Add for Ansi {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

fn diff(lhs: State, rhs: State) -> State {
    match (lhs, rhs) {
        (State::None | State::E, _) => lhs,
        (lhs, State::None | State::E) => lhs,
        (State::A, State::A | State::BA) => State::None,
        (State::B, State::A | State::BA) => State::AB,
        (State::A, State::B | State::AB) => State::BA,
        (State::B, State::B | State::AB) => State::None,

        (lhs, _) => lhs,
    }
}

fn sub(lhs: Effect, rhs: Effect) -> Effect {
    match (lhs, rhs) {
        (Effect::None, _) => Effect::None,
        (Effect::Apply, Effect::Apply) => Effect::None,
        (Effect::Clear, Effect::Clear) => Effect::None,
        (Effect::Apply, _) => Effect::Apply,
        (Effect::Clear, _) => Effect::Clear,
    }
}

impl SubAssign for Ansi {
    /// Difference between self and rhs.
    fn sub_assign(&mut self, rhs: Self) {
        self.set_brightness(diff(self.brightness(), rhs.brightness()));
        self.set_under(diff(self.under(), rhs.under()));

        self.set_blink(sub(self.blink(), rhs.blink()));
        self.set_hidden(sub(self.hidden(), rhs.hidden()));
        self.set_italics(sub(self.italics(), rhs.italics()));
        self.set_negative(sub(self.negative(), rhs.negative()));
        self.set_strike(sub(self.strike(), rhs.strike()));

        if !self.fg_color().is_empty() && !rhs.fg_color().is_empty() {
            self.set_fg_color(Color::default());
        }

        if !self.bg_color().is_empty() && !rhs.bg_color().is_empty() {
            self.set_bg_color(Color::default());
        }
    }
}

impl Sub for Ansi {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

fn invert(value: State) -> State {
    match value {
        State::None => State::None,
        State::A | State::B | State::AB | State::BA => State::E,
        State::E => State::None,
    }
}

impl Not for Ansi {
    type Output = Ansi;

    /// Invert Ansi style.
    fn not(mut self) -> Self::Output {
        macro_rules! reverse {
            ( $i:expr, $self:tt ) => {
                let value1 = get_style(&self.style, $i);
                let value2 = get_style(&self.style, $i + 15);
                set_style(&mut $self.style, $i, value2);
                set_style(&mut $self.style, $i + 15, value1);
            };
        }

        self.set_brightness(invert(self.brightness()));
        self.set_under(invert(self.under()));

        reverse!(8, self);
        reverse!(9, self);
        reverse!(10, self);
        reverse!(11, self);
        reverse!(12, self);

        if !self.fg_color().is_empty() {
            self.set_fg_color(Color::four_bit(39));
        }

        if !self.bg_color().is_empty() {
            self.set_bg_color(Color::four_bit(49));
        }

        self
    }
}

#[cfg(test)]
mod test {
    use crate::parser::ansi::Effect;

    use super::Ansi;
    use super::State;

    #[test]
    fn test_ansi_add() {
        let mut lhs = Ansi::new();
        lhs.set_brightness(State::A);

        let mut rhs = Ansi::new();
        rhs.set_blink(Effect::Apply);
        rhs.set_negative(Effect::Apply);

        lhs += rhs;

        assert_eq!(lhs.brightness(), State::A);
        assert_eq!(lhs.under(), State::None);
        assert_eq!(lhs.blink(), Effect::Apply);
        assert_eq!(lhs.hidden(), Effect::None);
        assert_eq!(lhs.strike(), Effect::None);
        assert_eq!(lhs.italics(), Effect::None);
        assert_eq!(lhs.negative(), Effect::Apply);
    }

    #[test]
    fn test_ansi_sub() {
        let mut lhs = Ansi::new();
        lhs.set_brightness(State::B);
        lhs.set_blink(Effect::Apply);
        lhs.set_negative(Effect::Apply);

        let mut rhs = Ansi::new();
        rhs.set_blink(Effect::Apply);
        rhs.set_negative(Effect::Apply);

        lhs -= rhs.clone();

        assert_ne!(lhs, rhs);

        assert_eq!(lhs.brightness(), State::B);
        assert_eq!(lhs.under(), State::None);
        assert_eq!(lhs.blink(), Effect::None);
        assert_eq!(lhs.hidden(), Effect::None);
        assert_eq!(lhs.strike(), Effect::None);
        assert_eq!(lhs.italics(), Effect::None);
        assert_eq!(lhs.negative(), Effect::None);
    }

    #[test]
    fn test_ansi_not() {
        let mut ansi = Ansi::new();
        ansi.set_brightness(State::A);
        ansi.set_blink(Effect::Apply);
        ansi.set_negative(Effect::Apply);

        let not_ansi = !ansi.clone();

        assert_eq!(ansi.brightness(), State::A);
        assert_eq!(ansi.under(), State::None);
        assert_eq!(not_ansi.blink(), Effect::Clear);
        assert_eq!(not_ansi.hidden(), Effect::None);
        assert_eq!(not_ansi.strike(), Effect::None);
        assert_eq!(not_ansi.italics(), Effect::None);
        assert_eq!(not_ansi.negative(), Effect::Clear);
    }
}
