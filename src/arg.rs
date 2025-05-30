#![allow(dead_code)]
use std::env::Args;

#[derive(Debug)]
pub enum Arg {
    ShortFlag(String, String),
    LongFlag(String, String),
    ShortSwitch(String),
    LongSwitch(String),
    Param(String),
}

#[derive(Debug)]
pub enum Error {
    Long(String),
    Short(String),
}

pub struct Cli<'a> {
    pub short_flags: &'a [&'a str],
    pub long_flags: &'a [&'a str],
    pub short_switches: &'a [&'a str],
    pub long_switches: &'a [&'a str],
}

fn split_args(args0: Args) -> Vec<String> {
    let mut args: Vec<String> = vec![];

    for arg in args0 {
        if let Some(ch) = arg.strip_prefix('-') {
            if ch.chars().nth(0) == Some('-') {
                args.push(arg.clone());
                continue;
            }

            let _: Vec<_> = ch.chars().map(|v| args.push(format!("-{v}"))).collect();
            continue;
        }

        args.push(arg.clone());
    }

    args
}

pub fn parse_args(args0: Args, cli: Cli) -> Result<Vec<Arg>, Error> {
    let _args0 = split_args(args0);
    let mut parts = _args0.split(|x| x == "--");
    let args0 = parts.next().unwrap();
    let mut args = vec![];

    let mut i = 0;
    let length = args0.len();
    while i < length {
        let arg = args0[i].clone();
        if let Some(arg) = arg.strip_prefix("--") {
            if arg.contains('=') {
                let mut split = arg.split('=');
                let key = split.next().unwrap();
                let value = split.collect::<Vec<_>>().join("=");
                if cli.long_flags.contains(&key) {
                    args.push(Arg::LongFlag(key.to_owned(), value));
                } else {
                    return Err(Error::Long(key.to_owned()));
                }
            } else {
                let key = arg;
                if cli.long_flags.contains(&key) {
                    args.push(Arg::LongFlag(key.to_owned(), args0[i + 1].clone()));
                    i += 1;
                } else if cli.long_switches.contains(&key) {
                    args.push(Arg::LongSwitch(key.to_owned()));
                } else {
                    return Err(Error::Long(key.to_owned()));
                }
            }
        } else if let Some(arg) = arg.strip_prefix("-") {
            if arg.contains('=') {
                let mut split = arg.split('=');
                let key = split.next().unwrap();
                let value = split.collect::<Vec<_>>().join("=");
                if cli.short_flags.contains(&key) {
                    args.push(Arg::ShortFlag(key.to_owned(), value));
                } else {
                    return Err(Error::Short(key.to_owned()));
                }
            } else {
                let key = arg;
                if cli.short_flags.contains(&key) {
                    args.push(Arg::ShortFlag(key.to_owned(), args0[i + 1].clone()));
                    i += 1;
                } else if cli.short_switches.contains(&key) {
                    args.push(Arg::ShortSwitch(key.to_owned()));
                } else {
                    return Err(Error::Short(key.to_owned()));
                }
            }
        } else {
            args.push(Arg::Param(arg));
        }
        i += 1;
    }

    args.extend(
        parts
            .collect::<Vec<_>>()
            .join(&String::from("--"))
            .iter()
            .map(|x| Arg::Param(x.clone())),
    );

    Ok(args)
}
