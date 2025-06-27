use std::fmt::{Debug, Display, Write};
use std::io::Write as _;
use std::ops::{Add, AddAssign, Not, Sub, SubAssign};

use crate::parser::color::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    None,
    A,
    B,
    AB,
    BA,
    E,
}

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
}

fn get_style(style: &u32, offset: u32) -> bool {
    let n = (style >> (Ansi::L - offset)) & 1;
    if n == 1 { true } else { false }
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
        $( ( $j:expr, $set_y:tt, $y:tt, $clear_y:tt ) ),*;
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
            pub fn $set_y(&mut self, value: bool) {
                set_style(&mut self.style, $j, value);
                set_style(&mut self.style, $j + 15, !value);
            }

            pub fn $y(&self) -> bool {
                get_style(&self.style, $j)
            }

            pub fn $clear_y(&self) -> bool {
                get_style(&self.style, $j + 15)
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

    (8, set_blink, blink, clear_blink),
    (9, set_hidden, hidden, clear_hidden),
    (10, set_italics, italics, clear_italics),
    (11, set_negative, negative, clear_negative),
    (12, set_strike, strike, clear_strike);

    (0, set_fg_color, fg_color),
    (1, set_bg_color, bg_color)
];

impl Debug for Ansi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ansi")
            .field("brightness", &self.brightness())
            .field("under", &self.under())
            .field("blink", &(self.blink(), self.clear_blink()))
            .field("hidden", &(self.hidden(), self.clear_hidden()))
            .field("italics", &(self.italics(), self.clear_italics()))
            .field("negative", &(self.negative(), self.clear_negative()))
            .field("strike", &(self.strike(), self.clear_strike()))
            .field("fg_color", self.fg_color())
            .field("bg_color", self.bg_color())
            .finish()
    }
}

impl Display for Ansi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            return f.write_fmt(format_args!("\"{}\"", self.to_string().escape_debug()));
        }

        let mut buf = Vec::with_capacity(128);
        let _ = buf.write(b"\x1b[");
        macro_rules! write_prop {
            ( $f:tt, $e:expr ) => {
                if self.$f() {
                    let _ = buf.write($e);
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

        write_prop!(italics, b"3;");
        write_prop!(clear_italics, b"23;");

        write_prop!(blink, b"5;");
        write_prop!(clear_blink, b"25;");

        write_prop!(negative, b"7;");
        write_prop!(clear_negative, b"27;");

        write_prop!(hidden, b"8;");
        write_prop!(clear_hidden, b"28;");

        write_prop!(strike, b"9;");
        write_prop!(clear_strike, b"29;");

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

fn add(lhs: bool, clear: bool, rhs: bool) -> bool {
    match (lhs, clear, rhs) {
        (lhs, false, rhs) => lhs | rhs,
        (_, true, _) => false,
    }
}

impl AddAssign for Ansi {
    /// Add two Ansi styles together.
    fn add_assign(&mut self, rhs: Self) {
        macro_rules! add_styles {
            ( $offset:expr, $self:tt, $rhs:tt, $x:tt, $clear_x:tt ) => {
                let value = add($self.$x(), rhs.$clear_x(), $rhs.$x());
                set_style(&mut $self.style, $offset, value);
            };
        }

        self.set_brightness(merge(self.brightness(), rhs.brightness()));
        self.set_under(merge(self.under(), rhs.under()));

        add_styles!(8, self, rhs, blink, clear_blink);
        add_styles!(9, self, rhs, hidden, clear_hidden);
        add_styles!(10, self, rhs, italics, clear_italics);
        add_styles!(11, self, rhs, negative, clear_negative);
        add_styles!(12, self, rhs, strike, clear_strike);

        add_styles!(8 + 15, self, rhs, clear_blink, blink);
        add_styles!(9 + 15, self, rhs, clear_hidden, hidden);
        add_styles!(10 + 15, self, rhs, clear_italics, italics);
        add_styles!(11 + 15, self, rhs, clear_negative, negative);
        add_styles!(12 + 15, self, rhs, clear_strike, strike);

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

impl SubAssign for Ansi {
    /// Difference between self and rhs.
    fn sub_assign(&mut self, rhs: Self) {
        macro_rules! sub {
            ( $offset:expr, $lhs:tt, $rhs:tt ) => {
                if (get_style(&$lhs.style, $offset) && get_style(&$rhs.style, $offset)) {
                    set_style(&mut $lhs.style, $offset, false);
                }
            };
        }

        self.set_brightness(diff(self.brightness(), rhs.brightness()));
        self.set_under(diff(self.under(), rhs.under()));

        sub!(8, self, rhs);
        sub!(9, self, rhs);
        sub!(10, self, rhs);
        sub!(11, self, rhs);
        sub!(12, self, rhs);

        sub!(8 + 15, self, rhs);
        sub!(9 + 15, self, rhs);
        sub!(10 + 15, self, rhs);
        sub!(11 + 15, self, rhs);
        sub!(12 + 15, self, rhs);

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
    use super::Ansi;
    use super::State;

    #[test]
    fn test_ansi_add() {
        let mut lhs = Ansi::new();
        lhs.set_brightness(State::A);

        let mut rhs = Ansi::new();
        rhs.set_blink(true);
        rhs.set_negative(true);

        lhs += rhs;

        assert_eq!(lhs.brightness(), State::A);
        assert_eq!(lhs.under(), State::None);
        assert!(lhs.blink());
        assert!(!lhs.hidden());
        assert!(!lhs.strike());
        assert!(!lhs.italics());
        assert!(lhs.negative());
    }

    #[test]
    fn test_ansi_sub() {
        let mut lhs = Ansi::new();
        lhs.set_brightness(State::B);
        lhs.set_blink(true);
        lhs.set_negative(true);

        let mut rhs = Ansi::new();
        rhs.set_blink(true);
        rhs.set_negative(true);

        lhs -= rhs.clone();

        assert_ne!(lhs, rhs);

        assert_eq!(lhs.brightness(), State::B);
        assert_eq!(lhs.under(), State::None);
        assert!(!lhs.blink());
        assert!(!lhs.hidden());
        assert!(!lhs.strike());
        assert!(!lhs.italics());
        assert!(!lhs.negative());
    }

    #[test]
    fn test_ansi_not() {
        let mut ansi = Ansi::new();
        ansi.set_brightness(State::A);
        ansi.set_blink(true);
        ansi.set_negative(true);

        let not_ansi = !ansi.clone();

        assert_eq!(ansi.brightness(), State::A);
        assert_eq!(ansi.under(), State::None);
        assert!(!not_ansi.blink());
        assert!(!not_ansi.hidden());
        assert!(!not_ansi.strike());
        assert!(!not_ansi.italics());
        assert!(!not_ansi.negative());

        assert!(not_ansi.clear_blink());
        assert!(!not_ansi.clear_hidden());
        assert!(!not_ansi.clear_strike());
        assert!(!not_ansi.clear_italics());
        assert!(not_ansi.clear_negative());
    }
}
