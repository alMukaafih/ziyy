use std::io::Write;
use crate::scanner::token::TokenKind as TT;
use crate::scanner::Scanner;
use super::state::State;
use crate::value::*;

pub struct Parser<'a, W: Write> {
    pub(super) scanner: Scanner<'a>,
    pub(super) out: &'a mut W,
    pub(super) state: State
}

impl<'a, W: Write> Parser<'a, W> {
    pub fn new(source: &'a str, out: &'a mut W) -> Parser<'a, W> {
        Parser {
            scanner: Scanner::new(source),
            out,
            state: State::new()
        }
    }

    pub fn parse_to_out(&mut self) {
        loop {
            let token = self.scanner.scan_token();
            match token.kind {
                TT::Text => {
                    let _ = write!(self.out, "{}", token.content);
                },
                TT::B => {
                    self.state.push(token.content, B);
                    let _ = write!(self.out, "{}", B);
                },
                TT::I => {
                    self.state.push(token.content, I);
                    let _ = write!(self.out, "{}", I);
                },
                TT::S => {
                    self.state.push(token.content, S);
                    let _ = write!(self.out, "{}", S);
                },
                TT::U => {
                    self.state.push(token.content, U);
                    let _ = write!(self.out, "{}", U);
                },
                TT::C => {
                    let token = self.scanner.scan_token();
                    match token.kind {
                        TT::Dot => {
                            let token = self.scanner.scan_token();
                            match token.kind {
                                TT::Black => {
                                    self.state.push("c", C::BLACK);
                                    let _ = write!(self.out, "{}", C::BLACK);
                                },
                                TT::Blue => {
                                    self.state.push("c", C::BLUE);
                                    let _ = write!(self.out, "{}", C::BLUE);
                                },
                                TT::Cyan => {
                                    self.state.push("c", C::CYAN);
                                    let _ = write!(self.out, "{}", C::CYAN);
                                },
                                TT::Green => {
                                    self.state.push("c", C::GREEN);
                                    let _ = write!(self.out, "{}", C::GREEN);
                                },
                                TT::Magenta => {
                                    self.state.push("c", C::MAGENTA);
                                    let _ = write!(self.out, "{}", C::MAGENTA);
                                },
                                TT::Red => {
                                    self.state.push("c", C::RED);
                                    let _ = write!(self.out, "{}", C::RED);
                                },
                                TT::White => {
                                    self.state.push("c", C::WHITE);
                                    let _ = write!(self.out, "{}", C::WHITE);
                                },
                                TT::Yellow => {
                                    self.state.push("c", C::YELLOW);
                                    let _ = write!(self.out, "{}", C::YELLOW);
                                },
                                _ => panic!()
                            }
                        },
                        _ => panic!()
                    }
                },
                TT::X => {
                    let token = self.scanner.scan_token();
                    match token.kind {
                        TT::Dot => {
                            let token = self.scanner.scan_token();
                            match token.kind {
                                TT::Black => {
                                    self.state.push("x", X::BLACK);
                                    let _ = write!(self.out, "{}", X::BLACK);
                                },
                                TT::Blue => {
                                    self.state.push("x", X::BLUE);
                                    let _ = write!(self.out, "{}", X::BLUE);
                                },
                                TT::Cyan => {
                                    self.state.push("x", X::CYAN);
                                    let _ = write!(self.out, "{}", X::CYAN);
                                },
                                TT::Green => {
                                    self.state.push("x", X::GREEN);
                                    let _ = write!(self.out, "{}", X::GREEN);
                                },
                                TT::Magenta => {
                                    self.state.push("x", X::MAGENTA);
                                    let _ = write!(self.out, "{}", X::MAGENTA);
                                },
                                TT::Red => {
                                    self.state.push("x", X::RED);
                                    let _ = write!(self.out, "{}", X::RED);
                                },
                                TT::White => {
                                    self.state.push("x", X::WHITE);
                                    let _ = write!(self.out, "{}", X::WHITE);
                                },
                                TT::Yellow => {
                                    self.state.push("x", X::YELLOW);
                                    let _ = write!(self.out, "{}", X::YELLOW);
                                },
                                _ => panic!()
                            }
                        },
                        _ => panic!()
                    }
                },
                TT::Slash => {
                    let token = self.scanner.scan_token();
                    match token.kind {
                        TT::B => {
                            if self.state.current_tag() != "b" {
                                panic!()
                            }
                            self.state.pop();
                            let _ = write!(self.out, "{}", RESET_B);
                        },
                        TT::I => {
                            if self.state.current_tag() != "i" {
                                panic!()
                            }
                            self.state.pop();
                            let _ = write!(self.out, "{}", RESET_I);
                        },
                        TT::S => {
                            if self.state.current_tag() != "s" {
                                panic!()
                            }
                            self.state.pop();
                            let _ = write!(self.out, "{}", RESET_S);
                        },
                        TT::U => {
                            if self.state.current_tag() != "u" {
                                panic!()
                            }
                            self.state.pop();
                            let _ = write!(self.out, "{}", RESET_U);
                        },
                        TT::C => {
                            if self.state.current_tag() != "c" {
                                panic!()
                            }
                            self.state.pop();
                            let saved = self.state.current_save();
                            let _ = write!(self.out, "{}", saved);
                        },
                        TT::X => {
                            if self.state.current_tag() != "x" {
                                panic!()
                            }
                            self.state.pop();
                            let saved = self.state.current_save();
                            let _ = write!(self.out, "{}", saved);
                        },
                        _ => continue
                    }
                },
                TT::Eof => break,
                _ => continue,
            }
        }
    }

    fn write_bytes(&mut self, bytes: &str) {
        let _ = write!(self.out, "{}", bytes);
    }
}