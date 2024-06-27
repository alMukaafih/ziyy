use super::state::State;
use crate::scanner::token::TokenKind as TT;
use crate::scanner::Scanner;
use crate::value::*;
use std::collections::HashMap;
use std::io::Write;

pub struct Parser<'a, W: Write> {
    pub(super) scanner: Scanner<'a>,
    pub(super) out: &'a mut W,
    pub(super) state: State,
    pub(crate) variables: HashMap<String, String>,
}

impl<'a, W: Write> Parser<'a, W> {
    pub fn new(
        source: &'a str,
        out: &'a mut W,
        variables: HashMap<String, String>,
    ) -> Parser<'a, W> {
        Parser {
            scanner: Scanner::new(source),
            out,
            state: State::new(),
            variables,
        }
    }

    pub fn parse_to_out(&mut self) {
        let _ = write!(self.out, "{}",  RESET);
        loop {
            let token = self.scanner.scan_token();
            match token.kind {
                TT::Text => {
                    let _ = write!(self.out, "{}", token.content);
                }
                TT::B => {
                    self.state.push(token.content, B);
                    let _ = write!(self.out, "{}", B);

                    let token = self.scanner.scan_token();
                    if token.kind != TT::CloseTag {
                        panic!()
                    }
                }
                TT::I => {
                    self.state.push(token.content, I);
                    let _ = write!(self.out, "{}", I);

                    let token = self.scanner.scan_token();
                    if token.kind != TT::CloseTag {
                        panic!()
                    }
                }
                TT::S => {
                    self.state.push(token.content, S);
                    let _ = write!(self.out, "{}", S);

                    let token = self.scanner.scan_token();
                    if token.kind != TT::CloseTag {
                        panic!()
                    }
                }
                TT::U => {
                    self.state.push(token.content, U);
                    let _ = write!(self.out, "{}", U);

                    let token = self.scanner.scan_token();
                    if token.kind != TT::CloseTag {
                        panic!()
                    }
                }
                TT::C => {
                    let token = self.scanner.scan_token();
                    match token.kind {
                        TT::Dot => {
                            let token = self.scanner.scan_token();
                            match token.kind {
                                TT::Black => {
                                    self.state.push("c", C::BLACK);
                                    let _ = write!(self.out, "{}", C::BLACK);
                                }
                                TT::Blue => {
                                    self.state.push("c", C::BLUE);
                                    let _ = write!(self.out, "{}", C::BLUE);
                                }
                                TT::Cyan => {
                                    self.state.push("c", C::CYAN);
                                    let _ = write!(self.out, "{}", C::CYAN);
                                }
                                TT::Green => {
                                    self.state.push("c", C::GREEN);
                                    let _ = write!(self.out, "{}", C::GREEN);
                                }
                                TT::Magenta => {
                                    self.state.push("c", C::MAGENTA);
                                    let _ = write!(self.out, "{}", C::MAGENTA);
                                }
                                TT::Red => {
                                    self.state.push("c", C::RED);
                                    let _ = write!(self.out, "{}", C::RED);
                                }
                                TT::White => {
                                    self.state.push("c", C::WHITE);
                                    let _ = write!(self.out, "{}", C::WHITE);
                                }
                                TT::Yellow => {
                                    self.state.push("c", C::YELLOW);
                                    let _ = write!(self.out, "{}", C::YELLOW);
                                }
                                TT::Rgb => {
                                    let token = self.scanner.scan_token();
                                    if token.kind != TT::LeftParen {
                                        panic!()
                                    }

                                    let token = self.scanner.scan_token();
                                    if token.kind != TT::Number {
                                        panic!()
                                    }
                                    let r: u8 = token.content.parse().unwrap();

                                    let token = self.scanner.scan_token();
                                    if token.kind != TT::Comma {
                                        panic!()
                                    }

                                    let token = self.scanner.scan_token();
                                    if token.kind != TT::Number {
                                        panic!()
                                    }
                                    let g: u8 = token.content.parse().unwrap();

                                    let token = self.scanner.scan_token();
                                    if token.kind != TT::Comma {
                                        panic!()
                                    }

                                    let token = self.scanner.scan_token();
                                    if token.kind != TT::Number {
                                        panic!()
                                    }
                                    let b: u8 = token.content.parse().unwrap();

                                    let token = self.scanner.scan_token();
                                    if token.kind != TT::RightParen {
                                        panic!()
                                    }

                                    let rgb = C::rgb(r, g, b);
                                    self.state.push(token.content, &rgb);
                                    let _ = write!(self.out, "{}", &rgb);
                                }
                                _ => panic!(),
                            }
                        }
                        _ => panic!(),
                    }

                    let token = self.scanner.scan_token();
                    if token.kind != TT::CloseTag {
                        panic!()
                    }
                }
                TT::X => {
                    let token = self.scanner.scan_token();
                    match token.kind {
                        TT::Dot => {
                            let token = self.scanner.scan_token();
                            match token.kind {
                                TT::Black => {
                                    self.state.push("x", X::BLACK);
                                    let _ = write!(self.out, "{}", X::BLACK);
                                }
                                TT::Blue => {
                                    self.state.push("x", X::BLUE);
                                    let _ = write!(self.out, "{}", X::BLUE);
                                }
                                TT::Cyan => {
                                    self.state.push("x", X::CYAN);
                                    let _ = write!(self.out, "{}", X::CYAN);
                                }
                                TT::Green => {
                                    self.state.push("x", X::GREEN);
                                    let _ = write!(self.out, "{}", X::GREEN);
                                }
                                TT::Magenta => {
                                    self.state.push("x", X::MAGENTA);
                                    let _ = write!(self.out, "{}", X::MAGENTA);
                                }
                                TT::Red => {
                                    self.state.push("x", X::RED);
                                    let _ = write!(self.out, "{}", X::RED);
                                }
                                TT::White => {
                                    self.state.push("x", X::WHITE);
                                    let _ = write!(self.out, "{}", X::WHITE);
                                }
                                TT::Yellow => {
                                    self.state.push("x", X::YELLOW);
                                    let _ = write!(self.out, "{}", X::YELLOW);
                                }
                                TT::Rgb => {
                                    let token = self.scanner.scan_token();
                                    if token.kind != TT::LeftParen {
                                        panic!()
                                    }

                                    let token = self.scanner.scan_token();
                                    if token.kind != TT::Number {
                                        panic!()
                                    }
                                    let r: u8 = token.content.parse().unwrap();

                                    let token = self.scanner.scan_token();
                                    if token.kind != TT::Comma {
                                        panic!()
                                    }

                                    let token = self.scanner.scan_token();
                                    if token.kind != TT::Number {
                                        panic!()
                                    }
                                    let g: u8 = token.content.parse().unwrap();

                                    let token = self.scanner.scan_token();
                                    if token.kind != TT::Comma {
                                        panic!()
                                    }

                                    let token = self.scanner.scan_token();
                                    if token.kind != TT::Number {
                                        panic!()
                                    }
                                    let b: u8 = token.content.parse().unwrap();

                                    let token = self.scanner.scan_token();
                                    if token.kind != TT::RightParen {
                                        panic!()
                                    }

                                    let rgb = X::rgb(r, g, b);
                                    self.state.push("x", &rgb);
                                    let _ = write!(self.out, "{}", &rgb);
                                }
                                _ => panic!(),
                            }
                        }
                        _ => panic!(),
                    }

                    let token = self.scanner.scan_token();
                    if token.kind != TT::CloseTag {
                        panic!()
                    }
                }
                TT::Slash => {
                    let token = self.scanner.scan_token();
                    match token.kind {
                        TT::B => {
                            if self.state.current_tag() != "b" {
                                panic!()
                            }
                            self.state.pop();
                            let _ = write!(self.out, "{}", RESET_B);
                        }
                        TT::I => {
                            if self.state.current_tag() != "i" {
                                panic!()
                            }
                            self.state.pop();
                            let _ = write!(self.out, "{}", RESET_I);
                        }
                        TT::S => {
                            if self.state.current_tag() != "s" {
                                panic!()
                            }
                            self.state.pop();
                            let _ = write!(self.out, "{}", RESET_S);
                        }
                        TT::U => {
                            if self.state.current_tag() != "u" {
                                panic!()
                            }
                            self.state.pop();
                            let _ = write!(self.out, "{}", RESET_U);
                        }
                        TT::C => {
                            if self.state.current_tag() != "c" {
                                panic!()
                            }
                            self.state.pop();
                            let saved = self.state.current_save();
                            let _ = write!(self.out, "{}", saved);
                        }
                        TT::X => {
                            if self.state.current_tag() != "x" {
                                panic!()
                            }
                            self.state.pop();
                            let saved = self.state.current_save();
                            let _ = write!(self.out, "{}", saved);
                        }
                        TT::Identifier
                        | TT::Black
                        | TT::Blue
                        | TT::Cyan
                        | TT::Green
                        | TT::Magenta
                        | TT::Red
                        | TT::Rgb
                        | TT::White
                        | TT::Yellow => {
                            if self.state.current_tag() != token.content {
                                panic!()
                            }
                            self.state.pop();
                            let saved = self.state.current_save();
                            let _ = write!(self.out, "{}", saved);
                        }
                        _ => panic!(),
                    }

                    let token = self.scanner.scan_token();
                    if token.kind != TT::CloseTag {
                        panic!()
                    }
                }
                TT::Identifier
                | TT::Black
                | TT::Blue
                | TT::Cyan
                | TT::Green
                | TT::Magenta
                | TT::Red
                | TT::Rgb
                | TT::White
                | TT::Yellow => {
                    let var = self.variables.get(token.content);
                    if let Some(val) = var {
                        self.state.push(token.content, val);
                        let _ = write!(self.out, "{}", val);
                    } else {
                        panic!()
                    }
                }
                TT::Eof => {
                    let _ = write!(self.out, "{}", RESET);
                    break
                }
                _ => continue,
            }
        }
    }
}
