use std::env::Args;

#[derive(Debug)]
pub struct Arg {
    pub r#type: ArgType,
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug)]
pub enum ArgType {
    Param,
    Flag,
}

fn split_args(args0: Args) -> Vec<String> {
    let mut args = vec![];

    for arg in args0 {
        if let Some(ch) = arg.strip_prefix('-') {
            if arg.chars().nth(1) != Some('-') {
                let _: Vec<_> = ch
                    .chars()
                    .map(|v| {
                        let mut s = String::with_capacity(2);
                        s.push('-');
                        s.push(v);
                        args.push(s)
                    })
                    .collect();
            } else {
                args.push(arg);
            }
        } else {
            args.push(arg);
        }
    }

    args
}

pub fn parse_args(args0: Args) -> Vec<Arg> {
    let args0 = split_args(args0);
    let mut args = vec![];

    let mut i = 0;
    let length = args0.len();
    while i < length {
        let mut arg = args0[i].as_str();
        if arg.starts_with("--") {
            let mut value = None;
            let argv: Vec<_>;
            if arg.contains('=') {
                argv = arg.split('=').collect();
                arg = argv[0];
                value = Some(argv[1].to_owned());
            } else if let Some(v) = args0.get(i + 1) {
                if !v.starts_with('-') {
                    value = Some(v.clone());
                    i += 1;
                }
            }
            args.push(Arg {
                r#type: ArgType::Flag,
                key: arg[2..].to_owned(),
                value,
            });
        } else if let Some(ch) = arg.strip_prefix('-') {
            let mut value = None;
            if let Some(v) = args0.get(i + 1) {
                if !v.starts_with('-') {
                    value = Some(v.clone());
                    i += 1;
                }
            }
            args.push(Arg {
                r#type: ArgType::Flag,
                key: ch.to_owned(),
                value,
            });
        } else {
            args.push(Arg {
                r#type: ArgType::Param,
                key: arg.to_owned(),
                value: None,
            });
        }

        i += 1;
    }

    args
}
