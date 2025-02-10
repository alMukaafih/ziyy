#[macro_export]
#[doc(hidden)]
macro_rules! assign_prop_value {
    ( $tag:expr, $prop:tt, $scanner:expr, $token:expr ) => {{
        $tag.$prop = Value::None;

        $token = $scanner.scan_token()?;
        $tag.span.add(&$token.span);
        if $token.kind == TokenKind::Equal {
            $token = $scanner.scan_token()?;
            $tag.span.add(&$token.span);
            expect_token(&$token, TokenKind::String)?;
            let end = $token.content.len() - 1;
            $tag.$prop = Value::Some(own!($token.content[1..end]));
            $token = $scanner.scan_token()?;
        }
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! assign_prop_color {
    ( $tag:expr, $style:expr, $prop:tt, $ch:tt, $scanner:expr, $token:expr ) => {{
        $token = $scanner.scan_token()?;
        $tag.span.add(&$token.span);
        if $token.kind == TokenKind::Equal {
            $token = $scanner.scan_token()?;
            $tag.span.add(&$token.span);
            expect_token(&$token, TokenKind::String)?;
            let end = $token.content.len() - 1;

            $style.$prop = Some(Color::try_from((
                &$token.content[1..end],
                Channel::$ch,
                $token.span, // TODO: more accurate line reporting e.g. an array of positions for each token
            ))?);
            $token = $scanner.scan_token()?;
        }
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! assign_prop_bool {
    ( $tag:expr, $prop:tt, $scanner:expr, $token:expr ) => {{
        $tag.$prop = true;

        $token = $scanner.scan_token()?;
        if $token.kind == TokenKind::Equal {
            $token = $scanner.scan_token()?;
            expect_token(&$token, TokenKind::String)?;
            $token = $scanner.scan_token()?;
        }
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! assign_prop_cond {
    ( $tag:expr, $prop:tt, $v:expr, $scanner:expr, $token:expr ) => {{
        $tag.$prop = $v;

        $token = $scanner.scan_token()?;
        if $token.kind == TokenKind::Equal {
            $token = $scanner.scan_token()?;
            expect_token(&$token, TokenKind::String)?;
            $token = $scanner.scan_token()?;
        }
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! own {
    ( $x:expr ) => {
        $x.to_owned()
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! get_num {
    ( $kind:expr, $token:expr ) => {
        $kind.map_err(|k| Error::new(k, $token))?
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! get_num2 {
    ( $kind:expr, $tag:expr ) => {
        $kind.map_err(|k| Error {
            kind: k,
            span: $tag.span,
        })?
    };
}
