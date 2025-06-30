use std::{collections::HashMap, sync::LazyLock};

use crate::parser::ansi::{Ansi, AnsiOptions, State, Style};

pub static BUILTIN_STYLES: LazyLock<HashMap<&str, Ansi>> = LazyLock::new(|| {
    let mut map = HashMap::from([
        (
            "b",
            Ansi::with(AnsiOptions {
                brightness: State::A,
                ..Default::default()
            }),
        ),
        (
            "d",
            Ansi::with(AnsiOptions {
                brightness: State::B,
                ..Default::default()
            }),
        ),
        (
            "h",
            Ansi::with(AnsiOptions {
                hidden: Style::Apply,
                ..Default::default()
            }),
        ),
        (
            "i",
            Ansi::with(AnsiOptions {
                italics: Style::Apply,
                ..Default::default()
            }),
        ),
        (
            "k",
            Ansi::with(AnsiOptions {
                blink: Style::Apply,
                ..Default::default()
            }),
        ),
        (
            "r",
            Ansi::with(AnsiOptions {
                negetive: Style::Apply,
                ..Default::default()
            }),
        ),
        (
            "s",
            Ansi::with(AnsiOptions {
                strike: Style::Apply,
                ..Default::default()
            }),
        ),
        (
            "u",
            Ansi::with(AnsiOptions {
                under: State::A,
                ..Default::default()
            }),
        ),
        (
            "uu",
            Ansi::with(AnsiOptions {
                under: State::B,
                ..Default::default()
            }),
        ),
    ]);

    let aliases = [
        ("b", "strong"),
        ("d", "dim"),
        ("i", "em"),
        ("u", "ins"),
        ("k", "blink"),
        ("s", "del"),
    ];

    for (k, a) in aliases {
        let v = map.get(&k).unwrap().clone();
        map.insert(a, v);
    }

    map
});

pub static BUILTIN_TAGS: &[&str] = &[
    "a", "b", "blink", "br", "d", "del", "dim", "div", "em", "h", "i", "input", "ins", "k", "o",
    "over", "p", "r", "s", "script", "span", "strong", "style", "table", "td", "th", "tr", "u",
    "uu", "ziyy",
];
