#[macro_export]
#[doc(hidden)]
macro_rules! assign_attrib {
    ( $tag:expr, $attrib:tt, $scanner:expr, $token:expr, Value) => {{
        $tag.$attrib = Some(None);

        $token = $scanner.scan_token()?;
        if $token.kind == TokenKind::Equal {
            $token = $scanner.scan_token()?;
            expect_token(&$token, TokenKind::String)?;
            let end = $token.content.len() - 1;
            $tag.$attrib = Some(Some(own!($token.content[1..end])));
            $token = $scanner.scan_token()?;
        }
    }};

    ( $tag:expr, $attrib:tt, $scanner:expr, $token:expr, bool) => {{
        $tag.$attrib = true;

        $token = $scanner.scan_token()?;
        if $token.kind == TokenKind::Equal {
            $token = $scanner.scan_token()?;
            expect_token(&$token, TokenKind::String)?;
            let end = $token.content.len() - 1;
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
