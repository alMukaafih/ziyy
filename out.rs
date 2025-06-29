#![feature(prelude_import)]
#![allow(clippy::pedantic)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
pub use error::{Error, ErrorType, Result};
pub use indexer::Indexer;
pub use parser::Parser;
pub use resolver::{Resolver, document::{Document, Node}};
pub use splitter::Splitter;
mod error {
    use crate::common::Span;
    pub enum ErrorType {
        InvalidTag,
        InvalidTagName,
        InvalidTagProperty,
        InvalidTagValue,
        InvalidTagFormat,
        InvalidTagClose,
        InvalidTagOpen,
        InvalidTagSelfClose,
        InvalidTagPropertyValue,
        InvalidNumber,
        InvalidColor,
        UnexpectedToken,
        UnknownToken,
        UnexpectedEof,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ErrorType {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ErrorType::InvalidTag => "InvalidTag",
                    ErrorType::InvalidTagName => "InvalidTagName",
                    ErrorType::InvalidTagProperty => "InvalidTagProperty",
                    ErrorType::InvalidTagValue => "InvalidTagValue",
                    ErrorType::InvalidTagFormat => "InvalidTagFormat",
                    ErrorType::InvalidTagClose => "InvalidTagClose",
                    ErrorType::InvalidTagOpen => "InvalidTagOpen",
                    ErrorType::InvalidTagSelfClose => "InvalidTagSelfClose",
                    ErrorType::InvalidTagPropertyValue => "InvalidTagPropertyValue",
                    ErrorType::InvalidNumber => "InvalidNumber",
                    ErrorType::InvalidColor => "InvalidColor",
                    ErrorType::UnexpectedToken => "UnexpectedToken",
                    ErrorType::UnknownToken => "UnknownToken",
                    ErrorType::UnexpectedEof => "UnexpectedEof",
                },
            )
        }
    }
    pub struct Error {
        pub r#type: ErrorType,
        pub message: String,
        pub span: Span,
    }
    impl Error {
        pub fn new(r#type: ErrorType, message: String, span: Span) -> Self {
            Self { r#type, message, span }
        }
    }
    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(
                format_args!(
                    "Error: {0:?} at span {1:?}: {2}",
                    self.r#type,
                    self.span,
                    self.message,
                ),
            )
        }
    }
    impl std::error::Error for Error {
        fn description(&self) -> &str {
            &self.message
        }
    }
    impl std::fmt::Debug for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(
                format_args!(
                    "Error: {0:?} at line {1:?}: {2}",
                    self.r#type,
                    self.span,
                    self.message,
                ),
            )
        }
    }
    impl std::fmt::Display for ErrorType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{0:?}", self))
        }
    }
    pub type Result<T> = std::result::Result<T, Error>;
}
#[macro_use]
mod scanner {
    pub use source::Source;
    use crate::common::Span;
    mod source {
        pub trait Source<T: PartialEq> {
            fn null(&self) -> T;
            fn nl(&self) -> T;
            fn len(&self) -> usize;
            fn at(&self, i: usize) -> T;
        }
        impl Source<char> for Vec<char> {
            fn null(&self) -> char {
                '\0'
            }
            fn nl(&self) -> char {
                '\n'
            }
            fn len(&self) -> usize {
                self.as_slice().len()
            }
            fn at(&self, i: usize) -> char {
                self[i]
            }
        }
    }
    pub fn is_alpha(c: char) -> bool {
        match c {
            'a'..='z' | 'A'..='Z' | '_' => true,
            _ => false,
        }
    }
    pub fn is_digit(c: char) -> bool {
        c.is_ascii_digit()
    }
    pub fn is_alpha_numeric(c: char) -> bool {
        is_alpha(c) || is_digit(c)
    }
    pub fn is_hexdigit(c: char) -> bool {
        c.is_ascii_hexdigit()
    }
    pub trait GenericScanner<T: PartialEq, U> {
        fn source(&self) -> &impl Source<T>;
        fn tokens(&mut self) -> &mut Vec<U>;
        fn set_start(&mut self, n: usize);
        fn current(&self) -> usize;
        fn set_current(&mut self, n: usize);
        fn span(&mut self) -> &mut Span;
        fn scan_token(&mut self);
        fn scan_tokens<'a>(&'a mut self) -> Vec<U>
        where
            T: 'a,
        {
            while !self.is_at_end() {
                self.set_start(self.current());
                self.scan_token();
            }
            std::mem::take(self.tokens())
        }
        fn peek(&self) -> T {
            if self.is_at_end() {
                self.source().null()
            } else {
                self.source().at(self.current())
            }
        }
        fn peek_next(&self) -> T {
            if self.current() + 1 > self.source().len() {
                self.source().null()
            } else {
                self.source().at(self.current() + 1)
            }
        }
        fn is_at_end(&self) -> bool {
            self.current() >= self.source().len()
        }
        fn advance(&mut self) -> T {
            self.set_current(self.current() + 1);
            *self.span() += (0, 1);
            let ch = self.source().at(self.current() - 1);
            if ch == self.source().nl() {
                *self.span() += (1, 0);
            }
            ch
        }
    }
}
mod common {
    pub use position::Position;
    pub use span::Span;
    mod position {
        use std::{fmt::Display, ops::{AddAssign, SubAssign}};
        pub struct Position {
            line: usize,
            column: usize,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Position {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Position",
                    "line",
                    &self.line,
                    "column",
                    &&self.column,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Position {
            #[inline]
            fn clone(&self) -> Position {
                let _: ::core::clone::AssertParamIsClone<usize>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Position {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Position {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Position {
            #[inline]
            fn eq(&self, other: &Position) -> bool {
                self.line == other.line && self.column == other.column
            }
        }
        impl Default for Position {
            fn default() -> Self {
                Self { line: 1, column: 1 }
            }
        }
        impl Position {
            pub fn new(line: usize, column: usize) -> Self {
                Self { line, column }
            }
        }
        impl AddAssign<(usize, usize)> for Position {
            fn add_assign(&mut self, rhs: (usize, usize)) {
                let (line, column) = rhs;
                self.line += line;
                if line > 0 {
                    self.column = 1;
                } else {
                    self.column += column;
                }
            }
        }
        impl SubAssign<(usize, usize)> for Position {
            fn sub_assign(&mut self, rhs: (usize, usize)) {
                let (line, column) = rhs;
                self.line -= line;
                self.column -= column;
            }
        }
        impl PartialOrd for Position {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                match self.line.partial_cmp(&other.line) {
                    Some(core::cmp::Ordering::Equal) => {}
                    ord => return ord,
                }
                self.column.partial_cmp(&other.column)
            }
        }
        impl Display for Position {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if f.alternate() {
                    f.write_fmt(format_args!("({0},{1})", self.line, self.column))
                } else {
                    f.write_fmt(format_args!("{0}:{1}", self.line, self.column))
                }
            }
        }
    }
    mod span {
        use std::{fmt::Display, ops::{Add, AddAssign, Sub}};
        use super::Position;
        pub struct Span {
            start: Position,
            end: Position,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Span {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Span",
                    "start",
                    &self.start,
                    "end",
                    &&self.end,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Span {
            #[inline]
            fn default() -> Span {
                Span {
                    start: ::core::default::Default::default(),
                    end: ::core::default::Default::default(),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Span {
            #[inline]
            fn clone(&self) -> Span {
                let _: ::core::clone::AssertParamIsClone<Position>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Span {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Span {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Span {
            #[inline]
            fn eq(&self, other: &Span) -> bool {
                self.start == other.start && self.end == other.end
            }
        }
        impl Span {
            pub fn new(start: Position, end: Position) -> Self {
                Self { start, end }
            }
            pub fn tie_end(&mut self) {
                self.start = self.end;
            }
            pub fn tie_start(&mut self) {
                self.end = self.start;
            }
            pub fn unquote(&self) -> Self {
                let mut span = *self;
                span.start += (0, 1);
                span.end -= (0, 1);
                span
            }
            pub fn inserted() -> Self {
                let pos = Position::new(0, 0);
                Span::new(pos, pos)
            }
        }
        impl Add<(usize, usize)> for Span {
            type Output = Span;
            fn add(mut self, rhs: (usize, usize)) -> Self::Output {
                self.end += rhs;
                self
            }
        }
        impl AddAssign<(usize, usize)> for Span {
            fn add_assign(&mut self, rhs: (usize, usize)) {
                self.end += rhs;
            }
        }
        impl AddAssign for Span {
            fn add_assign(&mut self, rhs: Self) {
                self.end = rhs.end;
            }
        }
        impl Sub<(usize, usize)> for Span {
            type Output = Span;
            fn sub(mut self, rhs: (usize, usize)) -> Self::Output {
                self.start -= rhs;
                self
            }
        }
        impl Display for Span {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if f.alternate() {
                    f.write_fmt(format_args!("{0:#}..{1:#}", self.start, self.end))
                } else if *self == Span::inserted() {
                    f.write_str("\x1b[4minserted\x1b[24m")
                } else {
                    f.write_fmt(format_args!("{0}..{1}", self.start, self.end))
                }
            }
        }
    }
}
mod indexer {
    use std::mem::take;
    /// The Indexer adds indices to empty placeholders
    #[doc(hidden)]
    pub struct Indexer {
        source: String,
        start: usize,
        current: usize,
        parts: Vec<String>,
    }
    impl Default for Indexer {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Indexer {
        pub fn new() -> Self {
            Self {
                source: String::new(),
                start: 0,
                current: 0,
                parts: Vec::new(),
            }
        }
        pub fn index(&mut self, source: String) -> String {
            self.source = source;
            self.parts = Vec::with_capacity(self.source.len() / 2);
            let mut index = 0;
            while !self.is_at_end() {
                self.start = self.current;
                if self.peek(0) == b'{' && self.peek(1) == b'{' {
                    self.advance(2);
                    self.add_part();
                } else if self.peek(0) == b'{' && self.peek(1) == b'}'
                    && self.peek(2) != b'}'
                {
                    self.advance(2);
                    self.parts
                        .push(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("{{{0}}}", index),
                                );
                                res
                            }),
                        );
                    index += 1;
                } else {
                    loop {
                        self.advance(1);
                        if self.is_at_end() {
                            break;
                        }
                        if self.peek(0) == b'{' && self.peek(1) == b'{' {
                            break;
                        }
                        if self.peek(0) == b'{' && self.peek(1) == b'}'
                            && self.peek(2) != b'}'
                        {
                            break;
                        }
                    }
                    self.add_part();
                }
            }
            take(&mut self.parts).join("")
        }
        fn peek(&self, n: usize) -> u8 {
            if self.current + n >= self.source.len() {
                b'\0'
            } else {
                self.source.as_bytes()[self.current + n]
            }
        }
        fn is_at_end(&self) -> bool {
            self.current >= self.source.len()
        }
        fn advance(&mut self, n: usize) {
            self.current += n;
        }
        fn add_part(&mut self) {
            let text = self.source[self.start..self.current].to_string();
            self.parts.push(text);
        }
    }
}
mod parser {
    use crate::error::Result;
    use crate::splitter::fragment::{Fragment, FragmentType};
    use chunk::{Chunk, ChunkData};
    use word_parer::WordParser;
    pub mod chunk {
        use std::{fmt::Display, ops::{Deref, DerefMut}};
        use crate::common::Span;
        use super::tag_parer::tag::Tag;
        pub enum ChunkData {
            Tag(Tag),
            WhiteSpace(String),
            Word(String),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ChunkData {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ChunkData::Tag(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Tag",
                            &__self_0,
                        )
                    }
                    ChunkData::WhiteSpace(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "WhiteSpace",
                            &__self_0,
                        )
                    }
                    ChunkData::Word(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Word",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ChunkData {
            #[inline]
            fn clone(&self) -> ChunkData {
                match self {
                    ChunkData::Tag(__self_0) => {
                        ChunkData::Tag(::core::clone::Clone::clone(__self_0))
                    }
                    ChunkData::WhiteSpace(__self_0) => {
                        ChunkData::WhiteSpace(::core::clone::Clone::clone(__self_0))
                    }
                    ChunkData::Word(__self_0) => {
                        ChunkData::Word(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ChunkData {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ChunkData {
            #[inline]
            fn eq(&self, other: &ChunkData) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (ChunkData::Tag(__self_0), ChunkData::Tag(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (
                            ChunkData::WhiteSpace(__self_0),
                            ChunkData::WhiteSpace(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (ChunkData::Word(__self_0), ChunkData::Word(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for ChunkData {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Tag>;
                let _: ::core::cmp::AssertParamIsEq<String>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for ChunkData {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_discr, state);
                match self {
                    ChunkData::Tag(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                    ChunkData::WhiteSpace(__self_0) => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    ChunkData::Word(__self_0) => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                }
            }
        }
        impl ChunkData {
            pub fn new_tag(tag: Tag) -> Self {
                Self::Tag(tag)
            }
            pub fn new_word(word: String) -> Self {
                Self::Word(word)
            }
            pub fn new_ws(ws: String) -> Self {
                Self::WhiteSpace(ws)
            }
            pub fn is_tag(&self) -> bool {
                match self {
                    ChunkData::Tag(_) => true,
                    _ => false,
                }
            }
            pub fn is_tag_and<F>(&self, f: F) -> bool
            where
                F: FnOnce(&Tag) -> bool,
            {
                match self {
                    ChunkData::Tag(tag) => f(tag),
                    _ => false,
                }
            }
            pub fn is_word(&self) -> bool {
                match self {
                    ChunkData::Word(_) => true,
                    _ => false,
                }
            }
            pub fn is_ws(&self) -> bool {
                match self {
                    ChunkData::WhiteSpace(_) => true,
                    _ => false,
                }
            }
            pub fn tag(&self) -> Option<&Tag> {
                if let ChunkData::Tag(tag) = self { Some(tag) } else { None }
            }
            pub fn word(&self) -> Option<&String> {
                if let ChunkData::Word(word) = self { Some(word) } else { None }
            }
            pub fn ws(&self) -> Option<&String> {
                if let ChunkData::WhiteSpace(ws) = self { Some(ws) } else { None }
            }
            pub fn tag_mut(&mut self) -> Option<&mut Tag> {
                if let ChunkData::Tag(tag) = self { Some(tag) } else { None }
            }
        }
        impl Display for ChunkData {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if f.alternate() {
                    match self {
                        ChunkData::Tag(tag) => f.write_fmt(format_args!("{0:#}", tag)),
                        ChunkData::WhiteSpace(ws) => {
                            f.write_fmt(format_args!("{0:?}", ws))
                        }
                        ChunkData::Word(word) => word.fmt(f),
                    }
                } else {
                    match self {
                        ChunkData::Tag(tag) => tag.fmt(f),
                        ChunkData::WhiteSpace(ws) => ws.fmt(f),
                        ChunkData::Word(word) => word.fmt(f),
                    }
                }
            }
        }
        pub struct Chunk {
            pub data: ChunkData,
            pub span: Span,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Chunk {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Chunk",
                    "data",
                    &self.data,
                    "span",
                    &&self.span,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Chunk {
            #[inline]
            fn clone(&self) -> Chunk {
                Chunk {
                    data: ::core::clone::Clone::clone(&self.data),
                    span: ::core::clone::Clone::clone(&self.span),
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Chunk {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Chunk {
            #[inline]
            fn eq(&self, other: &Chunk) -> bool {
                self.data == other.data && self.span == other.span
            }
        }
        impl Deref for Chunk {
            type Target = ChunkData;
            fn deref(&self) -> &Self::Target {
                &self.data
            }
        }
        impl DerefMut for Chunk {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.data
            }
        }
        impl Display for Chunk {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if f.alternate() {
                    f.write_fmt(
                        format_args!(
                            "{0:#} \u{1b}[38;5;59m--> {1}\u{1b}[39m",
                            self.data,
                            self.span,
                        ),
                    )
                } else {
                    self.data.fmt(f)
                }
            }
        }
    }
    pub mod color {
        use crate::common::Span;
        use crate::error::{Error, ErrorType};
        use crate::scanner::GenericScanner;
        pub use number::Number;
        use scanner::Scanner;
        use std::collections::VecDeque;
        use std::fmt::Display;
        use token::Token;
        use token::TokenType::*;
        mod number {
            use std::fmt::Display;
            pub enum Number {
                U8(u8),
                PlaceHolder(String),
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Number {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        Number::U8(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "U8",
                                &__self_0,
                            )
                        }
                        Number::PlaceHolder(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "PlaceHolder",
                                &__self_0,
                            )
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Number {
                #[inline]
                fn clone(&self) -> Number {
                    match self {
                        Number::U8(__self_0) => {
                            Number::U8(::core::clone::Clone::clone(__self_0))
                        }
                        Number::PlaceHolder(__self_0) => {
                            Number::PlaceHolder(::core::clone::Clone::clone(__self_0))
                        }
                    }
                }
            }
            impl Display for Number {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        Number::U8(u) => u.fmt(f),
                        Number::PlaceHolder(s) => s.fmt(f),
                    }
                }
            }
            impl From<u8> for Number {
                fn from(value: u8) -> Self {
                    Number::U8(value)
                }
            }
            impl From<String> for Number {
                fn from(value: String) -> Self {
                    Number::PlaceHolder(value)
                }
            }
        }
        mod scanner {
            use crate::common::Span;
            use crate::scanner::{
                GenericScanner, Source, is_alpha, is_alpha_numeric, is_digit, is_hexdigit,
            };
            use super::token::Token;
            use super::token::TokenType::{self, *};
            use std::collections::HashMap;
            use std::str::FromStr;
            use std::sync::LazyLock;
            pub static COLORS: LazyLock<HashMap<&str, TokenType>> = LazyLock::new(|| {
                [
                    ("bblack", BG_BLACK),
                    ("bblue", BG_BLUE),
                    ("bbyte", BG_FIXED),
                    ("bcyan", BG_CYAN),
                    ("bdefault", BG_DEFAULT),
                    ("bfalse", BG_DEFAULT),
                    ("bgreen", BG_GREEN),
                    ("bmagenta", BG_MAGENTA),
                    ("bred", BG_RED),
                    ("brgb", BG_RGB),
                    ("bwhite", BG_WHITE),
                    ("byellow", BG_YELLOW),
                    ("fblack", FG_BLACK),
                    ("fblue", FG_BLUE),
                    ("fbyte", FG_FIXED),
                    ("fcyan", FG_CYAN),
                    ("fdefault", FG_DEFAULT),
                    ("ffalse", FG_DEFAULT),
                    ("fgreen", FG_GREEN),
                    ("fmagenta", FG_MAGENTA),
                    ("fred", FG_RED),
                    ("frgb", FG_RGB),
                    ("fwhite", FG_WHITE),
                    ("fyellow", FG_YELLOW),
                ]
                    .into()
            });
            pub struct Scanner {
                source: Vec<char>,
                tokens: Vec<Token>,
                start: usize,
                current: usize,
                span: Span,
            }
            impl GenericScanner<char, Token> for Scanner {
                fn source(&self) -> &impl Source<char> {
                    &self.source
                }
                fn tokens(&mut self) -> &mut Vec<Token> {
                    &mut self.tokens
                }
                fn set_start(&mut self, n: usize) {
                    self.start = n;
                }
                fn current(&self) -> usize {
                    self.current
                }
                fn set_current(&mut self, n: usize) {
                    self.current = n;
                }
                fn span(&mut self) -> &mut Span {
                    &mut self.span
                }
                fn scan_token(&mut self) {
                    (|s: &mut Scanner| {
                        let c = s.advance();
                        match c {
                            '(' => s.add_token(LEFT_PAREN),
                            ')' => s.add_token(RIGHT_PAREN),
                            ',' => s.add_token(COMMA),
                            '{' => s.place_holder(),
                            '#' => s.hex(),
                            c => {
                                if is_digit(c) {
                                    s.number();
                                } else if is_alpha(c) {
                                    s.identifier();
                                } else {}
                            }
                        }
                    })(self);
                }
            }
            impl Scanner {
                pub fn new(source: String, mut span: Span) -> Self {
                    span.tie_start();
                    Self {
                        source: source.chars().collect(),
                        tokens: ::alloc::vec::Vec::new(),
                        start: 0,
                        current: 0,
                        span,
                    }
                }
                fn identifier(&mut self) {
                    while is_alpha_numeric(self.peek()) {
                        self.advance();
                    }
                    let k = self.source[self.start..self.current].to_string();
                    if let Some(r#type) = COLORS.get(k.as_str()) {
                        self.add_token(*r#type);
                    } else {
                        self.add_token(IDENTIFIER);
                    }
                }
                fn number(&mut self) {
                    while is_digit(self.peek()) {
                        self.advance();
                    }
                    if self.peek() == '.' && is_digit(self.peek_next()) {
                        self.advance();
                        while is_digit(self.peek()) {
                            self.advance();
                        }
                    }
                    let value = self.source[self.start..self.current].to_string();
                    self.add_token2(
                        NUMBER,
                        Some(f64::from_str(value.as_str()).unwrap().round())
                            .map(|x| {
                                if x > 255.0 { 255 } else if x < 0.0 { 0 } else { x as u8 }
                            }),
                    );
                }
                fn place_holder(&mut self) {
                    loop {
                        if self.peek() == '}' && self.peek_next() == '}' {
                            self.advance();
                        } else if self.peek() == '}' || self.is_at_end() {
                            break;
                        }
                        self.advance();
                    }
                    if self.is_at_end() {
                        return;
                    }
                    self.advance();
                    self.add_token(PLACE_HOLDER);
                }
                fn hex(&mut self) {
                    while is_hexdigit(self.peek()) {
                        self.advance();
                    }
                    self.add_token(HEX);
                }
                fn add_token(&mut self, r#type: TokenType) {
                    self.add_token2(r#type, None);
                }
                fn add_token2(&mut self, r#type: TokenType, literal: Option<u8>) {
                    let text = self.source[self.start..self.current].to_string();
                    self.tokens.push(Token::new(r#type, text, literal, self.span));
                }
            }
            trait ToString {
                fn to_string(&self) -> String;
            }
            impl ToString for [char] {
                fn to_string(&self) -> String {
                    let mut text = String::with_capacity(self.len());
                    for ch in self {
                        text.push(*ch)
                    }
                    text
                }
            }
        }
        mod token {
            use crate::common::Span;
            #[allow(non_camel_case_types)]
            #[allow(clippy::upper_case_acronyms)]
            pub enum TokenType {
                FG_BLACK,
                FG_RED,
                FG_GREEN,
                FG_YELLOW,
                FG_BLUE,
                FG_MAGENTA,
                FG_CYAN,
                FG_WHITE,
                FG_RGB,
                FG_FIXED,
                FG_DEFAULT,
                BG_BLACK,
                BG_RED,
                BG_GREEN,
                BG_YELLOW,
                BG_BLUE,
                BG_MAGENTA,
                BG_CYAN,
                BG_WHITE,
                BG_RGB,
                BG_FIXED,
                BG_DEFAULT,
                HEX,
                LEFT_PAREN,
                RIGHT_PAREN,
                PLACE_HOLDER,
                COMMA,
                NUMBER,
                IDENTIFIER,
            }
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            #[allow(clippy::upper_case_acronyms)]
            impl ::core::clone::Clone for TokenType {
                #[inline]
                fn clone(&self) -> TokenType {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            #[allow(clippy::upper_case_acronyms)]
            impl ::core::marker::Copy for TokenType {}
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            #[allow(clippy::upper_case_acronyms)]
            impl ::core::fmt::Debug for TokenType {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            TokenType::FG_BLACK => "FG_BLACK",
                            TokenType::FG_RED => "FG_RED",
                            TokenType::FG_GREEN => "FG_GREEN",
                            TokenType::FG_YELLOW => "FG_YELLOW",
                            TokenType::FG_BLUE => "FG_BLUE",
                            TokenType::FG_MAGENTA => "FG_MAGENTA",
                            TokenType::FG_CYAN => "FG_CYAN",
                            TokenType::FG_WHITE => "FG_WHITE",
                            TokenType::FG_RGB => "FG_RGB",
                            TokenType::FG_FIXED => "FG_FIXED",
                            TokenType::FG_DEFAULT => "FG_DEFAULT",
                            TokenType::BG_BLACK => "BG_BLACK",
                            TokenType::BG_RED => "BG_RED",
                            TokenType::BG_GREEN => "BG_GREEN",
                            TokenType::BG_YELLOW => "BG_YELLOW",
                            TokenType::BG_BLUE => "BG_BLUE",
                            TokenType::BG_MAGENTA => "BG_MAGENTA",
                            TokenType::BG_CYAN => "BG_CYAN",
                            TokenType::BG_WHITE => "BG_WHITE",
                            TokenType::BG_RGB => "BG_RGB",
                            TokenType::BG_FIXED => "BG_FIXED",
                            TokenType::BG_DEFAULT => "BG_DEFAULT",
                            TokenType::HEX => "HEX",
                            TokenType::LEFT_PAREN => "LEFT_PAREN",
                            TokenType::RIGHT_PAREN => "RIGHT_PAREN",
                            TokenType::PLACE_HOLDER => "PLACE_HOLDER",
                            TokenType::COMMA => "COMMA",
                            TokenType::NUMBER => "NUMBER",
                            TokenType::IDENTIFIER => "IDENTIFIER",
                        },
                    )
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            #[allow(clippy::upper_case_acronyms)]
            impl ::core::marker::StructuralPartialEq for TokenType {}
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            #[allow(clippy::upper_case_acronyms)]
            impl ::core::cmp::PartialEq for TokenType {
                #[inline]
                fn eq(&self, other: &TokenType) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            pub struct Token {
                pub r#type: TokenType,
                pub lexeme: String,
                pub literal: Option<u8>,
                pub span: Span,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Token {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field4_finish(
                        f,
                        "Token",
                        "type",
                        &self.r#type,
                        "lexeme",
                        &self.lexeme,
                        "literal",
                        &self.literal,
                        "span",
                        &&self.span,
                    )
                }
            }
            impl Token {
                pub fn new(
                    r#type: TokenType,
                    lexeme: String,
                    literal: Option<u8>,
                    span: Span,
                ) -> Self {
                    Token {
                        r#type,
                        literal,
                        lexeme,
                        span,
                    }
                }
            }
        }
        pub struct Rgb(pub u8, pub u8, pub u8, pub u8);
        #[automatically_derived]
        impl ::core::clone::Clone for Rgb {
            #[inline]
            fn clone(&self) -> Rgb {
                Rgb(
                    ::core::clone::Clone::clone(&self.0),
                    ::core::clone::Clone::clone(&self.1),
                    ::core::clone::Clone::clone(&self.2),
                    ::core::clone::Clone::clone(&self.3),
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Rgb {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Rgb {
            #[inline]
            fn eq(&self, other: &Rgb) -> bool {
                self.0 == other.0 && self.1 == other.1 && self.2 == other.2
                    && self.3 == other.3
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Rgb {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u8>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Rgb {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state);
                ::core::hash::Hash::hash(&self.1, state);
                ::core::hash::Hash::hash(&self.2, state);
                ::core::hash::Hash::hash(&self.3, state)
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Rgb {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field4_finish(
                    f,
                    "Rgb",
                    &self.0,
                    &self.1,
                    &self.2,
                    &&self.3,
                )
            }
        }
        impl Display for Rgb {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let Rgb(r, g, b, n) = self;
                f.write_fmt(format_args!("{0};2;{1};{2};{3};", n, r, g, b))
            }
        }
        pub struct Ansi256(pub u8, pub u8);
        #[automatically_derived]
        impl ::core::clone::Clone for Ansi256 {
            #[inline]
            fn clone(&self) -> Ansi256 {
                Ansi256(
                    ::core::clone::Clone::clone(&self.0),
                    ::core::clone::Clone::clone(&self.1),
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Ansi256 {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Ansi256 {
            #[inline]
            fn eq(&self, other: &Ansi256) -> bool {
                self.0 == other.0 && self.1 == other.1
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Ansi256 {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u8>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Ansi256 {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state);
                ::core::hash::Hash::hash(&self.1, state)
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Ansi256 {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field2_finish(
                    f,
                    "Ansi256",
                    &self.0,
                    &&self.1,
                )
            }
        }
        impl Display for Ansi256 {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let Ansi256(i, n) = self;
                f.write_fmt(format_args!("{0};5;{1};", n, i))
            }
        }
        pub struct Ansi4Bit(u8);
        #[automatically_derived]
        impl ::core::clone::Clone for Ansi4Bit {
            #[inline]
            fn clone(&self) -> Ansi4Bit {
                Ansi4Bit(::core::clone::Clone::clone(&self.0))
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Ansi4Bit {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Ansi4Bit {
            #[inline]
            fn eq(&self, other: &Ansi4Bit) -> bool {
                self.0 == other.0
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Ansi4Bit {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u8>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Ansi4Bit {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Ansi4Bit {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "Ansi4Bit",
                    &&self.0,
                )
            }
        }
        impl Display for Ansi4Bit {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
        pub enum Color {
            Ansi256(Ansi256),
            Ansi4Bit(Ansi4Bit),
            Rgb(Rgb),
            String(String),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Color {
            #[inline]
            fn clone(&self) -> Color {
                match self {
                    Color::Ansi256(__self_0) => {
                        Color::Ansi256(::core::clone::Clone::clone(__self_0))
                    }
                    Color::Ansi4Bit(__self_0) => {
                        Color::Ansi4Bit(::core::clone::Clone::clone(__self_0))
                    }
                    Color::Rgb(__self_0) => {
                        Color::Rgb(::core::clone::Clone::clone(__self_0))
                    }
                    Color::String(__self_0) => {
                        Color::String(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Color {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Color {
            #[inline]
            fn eq(&self, other: &Color) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (Color::Ansi256(__self_0), Color::Ansi256(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (Color::Ansi4Bit(__self_0), Color::Ansi4Bit(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (Color::Rgb(__self_0), Color::Rgb(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (Color::String(__self_0), Color::String(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for Color {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Ansi256>;
                let _: ::core::cmp::AssertParamIsEq<Ansi4Bit>;
                let _: ::core::cmp::AssertParamIsEq<Rgb>;
                let _: ::core::cmp::AssertParamIsEq<String>;
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Color {
            #[inline]
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                ::core::hash::Hash::hash(&__self_discr, state);
                match self {
                    Color::Ansi256(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                    Color::Ansi4Bit(__self_0) => {
                        ::core::hash::Hash::hash(__self_0, state)
                    }
                    Color::Rgb(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                    Color::String(__self_0) => ::core::hash::Hash::hash(__self_0, state),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Color {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Color::Ansi256(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Ansi256",
                            &__self_0,
                        )
                    }
                    Color::Ansi4Bit(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Ansi4Bit",
                            &__self_0,
                        )
                    }
                    Color::Rgb(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Rgb",
                            &__self_0,
                        )
                    }
                    Color::String(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "String",
                            &__self_0,
                        )
                    }
                }
            }
        }
        impl Default for Color {
            fn default() -> Self {
                Self::new()
            }
        }
        impl Color {
            pub const fn new() -> Self {
                Color::String(String::new())
            }
            pub fn with(s: String) -> Self {
                Color::String(s)
            }
            fn rgb(
                mut next: impl FnMut() -> Result<Token, Error>,
                n: u8,
            ) -> Result<Color, Error> {
                let token = next()?;
                expect(&token, LEFT_PAREN, ErrorType::UnexpectedToken)?;
                let token = next()?;
                let mut r: Number = Number::U8(0);
                let mut g: Number = Number::U8(0);
                let mut b: Number = Number::U8(0);
                match token.r#type {
                    NUMBER | PLACE_HOLDER => {
                        r = match token.r#type {
                            NUMBER => token.literal.unwrap().into(),
                            PLACE_HOLDER => token.lexeme.clone().into(),
                            _ => {
                                return Err(
                                    Error::new(
                                        ErrorType::InvalidNumber,
                                        ::alloc::__export::must_use({
                                            let res = ::alloc::fmt::format(
                                                format_args!("{0:?} is not a valid number", token.lexeme),
                                            );
                                            res
                                        }),
                                        token.span,
                                    ),
                                );
                            }
                        };
                        let token = next()?;
                        expect(&token, COMMA, ErrorType::UnexpectedToken)?;
                        let token = next()?;
                        g = match token.r#type {
                            NUMBER => token.literal.unwrap().into(),
                            PLACE_HOLDER => token.lexeme.clone().into(),
                            _ => {
                                return Err(
                                    Error::new(
                                        ErrorType::InvalidNumber,
                                        ::alloc::__export::must_use({
                                            let res = ::alloc::fmt::format(
                                                format_args!("{0:?} is not a valid number", token.lexeme),
                                            );
                                            res
                                        }),
                                        token.span,
                                    ),
                                );
                            }
                        };
                        let token = next()?;
                        expect(&token, COMMA, ErrorType::UnexpectedToken)?;
                        let token = next()?;
                        b = match token.r#type {
                            NUMBER => token.literal.unwrap().into(),
                            PLACE_HOLDER => token.lexeme.clone().into(),
                            _ => {
                                return Err(
                                    Error::new(
                                        ErrorType::InvalidNumber,
                                        ::alloc::__export::must_use({
                                            let res = ::alloc::fmt::format(
                                                format_args!("{0:?} is not a valid number", token.lexeme),
                                            );
                                            res
                                        }),
                                        token.span,
                                    ),
                                );
                            }
                        };
                    }
                    HEX => {
                        match token.lexeme.len() {
                            4 => {
                                r = u8::from_str_radix(&token.lexeme[1..2].repeat(2), 16)
                                    .unwrap()
                                    .into();
                                g = u8::from_str_radix(&token.lexeme[2..3].repeat(2), 16)
                                    .unwrap()
                                    .into();
                                b = u8::from_str_radix(&token.lexeme[3..4].repeat(2), 16)
                                    .unwrap()
                                    .into();
                            }
                            7 => {
                                r = u8::from_str_radix(&token.lexeme[1..3], 16)
                                    .unwrap()
                                    .into();
                                g = u8::from_str_radix(&token.lexeme[3..5], 16)
                                    .unwrap()
                                    .into();
                                b = u8::from_str_radix(&token.lexeme[5..7], 16)
                                    .unwrap()
                                    .into();
                            }
                            _ => {}
                        }
                    }
                    _ => {
                        return Err(
                            Error::new(
                                ErrorType::InvalidNumber,
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("{0:?} is not a valid number", token.lexeme),
                                    );
                                    res
                                }),
                                token.span,
                            ),
                        );
                    }
                }
                let token = next()?;
                expect(&token, RIGHT_PAREN, ErrorType::UnexpectedToken)?;
                match (&r, &g, &b) {
                    (Number::U8(r), Number::U8(g), Number::U8(b)) => {
                        Ok(Color::Rgb(Rgb(*r, *g, *b, n)))
                    }
                    _ => {
                        Ok(
                            Color::String(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("{0};2;{1};{2};{3};", n, r, g, b),
                                    );
                                    res
                                }),
                            ),
                        )
                    }
                }
            }
            fn fixed(
                mut next: impl FnMut() -> Result<Token, Error>,
                n: u8,
            ) -> Result<Color, Error> {
                let token = next()?;
                expect(&token, LEFT_PAREN, ErrorType::UnexpectedToken)?;
                let token = next()?;
                let i: Number = match token.r#type {
                    NUMBER => token.literal.unwrap().into(),
                    PLACE_HOLDER => token.lexeme.clone().into(),
                    _ => {
                        return Err(
                            Error::new(
                                ErrorType::InvalidNumber,
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("{0:?} is not a valid number", token.lexeme),
                                    );
                                    res
                                }),
                                token.span,
                            ),
                        );
                    }
                };
                let token = next()?;
                expect(&token, RIGHT_PAREN, ErrorType::UnexpectedToken)?;
                match &i {
                    Number::U8(i) => Ok(Color::Ansi256(Ansi256(*i, n))),
                    Number::PlaceHolder(_) => {
                        Ok(
                            Color::String(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("{0};5;{1};", n, i),
                                    );
                                    res
                                }),
                            ),
                        )
                    }
                }
            }
            pub fn four_bit(n: u8) -> Color {
                Color::Ansi4Bit(Ansi4Bit(n))
            }
            pub fn is_empty(&self) -> bool {
                self.to_string().is_empty()
            }
        }
        impl TryFrom<(String, Span)> for Color {
            type Error = crate::error::Error;
            fn try_from(source: (String, Span)) -> Result<Self, Self::Error> {
                if source.0.is_empty() {
                    return Ok(Color::String(source.0));
                }
                let mut scanner = Scanner::new(source.0, source.1);
                let mut tokens: VecDeque<_> = scanner.scan_tokens().into();
                let mut next = || {
                    if tokens.is_empty() {
                        return Err(
                            Error::new(
                                ErrorType::UnexpectedEof,
                                "Unexpected end of input".to_string(),
                                Span::default(),
                            ),
                        );
                    }
                    Ok(tokens.pop_front().unwrap())
                };
                let token = next()?;
                let color = match token.r#type {
                    token::TokenType::FG_BLACK => Color::four_bit(30),
                    token::TokenType::FG_RED => Color::four_bit(31),
                    token::TokenType::FG_GREEN => Color::four_bit(32),
                    token::TokenType::FG_YELLOW => Color::four_bit(33),
                    token::TokenType::FG_BLUE => Color::four_bit(34),
                    token::TokenType::FG_MAGENTA => Color::four_bit(35),
                    token::TokenType::FG_CYAN => Color::four_bit(36),
                    token::TokenType::FG_WHITE => Color::four_bit(37),
                    token::TokenType::FG_RGB => Color::rgb(next, 38)?,
                    token::TokenType::FG_FIXED => Color::fixed(next, 38)?,
                    token::TokenType::FG_DEFAULT => Color::four_bit(39),
                    token::TokenType::BG_BLACK => Color::four_bit(40),
                    token::TokenType::BG_RED => Color::four_bit(41),
                    token::TokenType::BG_GREEN => Color::four_bit(42),
                    token::TokenType::BG_YELLOW => Color::four_bit(43),
                    token::TokenType::BG_BLUE => Color::four_bit(44),
                    token::TokenType::BG_MAGENTA => Color::four_bit(45),
                    token::TokenType::BG_CYAN => Color::four_bit(46),
                    token::TokenType::BG_WHITE => Color::four_bit(47),
                    token::TokenType::BG_RGB => Color::rgb(next, 48)?,
                    token::TokenType::BG_FIXED => Color::fixed(next, 38)?,
                    token::TokenType::BG_DEFAULT => Color::four_bit(49),
                    _ => {
                        return Err(
                            Error::new(
                                ErrorType::InvalidColor,
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("{0:?} is not a valid color", token.lexeme),
                                    );
                                    res
                                }),
                                token.span,
                            ),
                        );
                    }
                };
                Ok(color)
            }
        }
        impl From<Color> for String {
            fn from(color: Color) -> Self {
                color.to_string()
            }
        }
        impl Display for Color {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Color::Ansi256(ansi256) => ansi256.fmt(f),
                    Color::Ansi4Bit(ansi4_bit) => ansi4_bit.fmt(f),
                    Color::Rgb(rgb) => rgb.fmt(f),
                    Color::String(s) => s.fmt(f),
                }
            }
        }
        fn expect(
            token: &Token,
            expected: token::TokenType,
            error: ErrorType,
        ) -> Result<(), Error> {
            if token.r#type == expected {
                Ok(())
            } else {
                Err(
                    Error::new(
                        error,
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Expected {0:?}, but found {1:?}",
                                    expected,
                                    token.r#type,
                                ),
                            );
                            res
                        }),
                        token.span,
                    ),
                )
            }
        }
    }
    pub mod tag_parer {
        use crate::error::{Error, ErrorType};
        use crate::{scanner::GenericScanner, splitter::fragment::Fragment};
        use scanner::Scanner;
        use std::collections::VecDeque;
        use tag::{Tag, TagType};
        use token::{Token, TokenType::*};
        use super::color::Color;
        mod scanner {
            use super::token::{Token, TokenType::{self, *}};
            use crate::{
                common::Span,
                scanner::{is_alpha, is_alpha_numeric, GenericScanner, Source},
                splitter::fragment::Fragment,
            };
            pub struct Scanner {
                source: Vec<char>,
                tokens: Vec<Token>,
                start: usize,
                current: usize,
                span: Span,
            }
            impl Scanner {
                pub fn new(mut source: Fragment) -> Self {
                    source.span.tie_start();
                    Self {
                        source: source.lexeme.chars().collect(),
                        tokens: ::alloc::vec::Vec::new(),
                        start: 0,
                        current: 0,
                        span: source.span,
                    }
                }
                fn identifier(&mut self) {
                    while is_alpha_numeric(self.peek()) {
                        self.advance();
                    }
                    self.add_token(IDENTIFIER);
                }
                fn string(&mut self, c: char) {
                    while self.peek() != c && !self.is_at_end() {
                        if self.peek() == '\n' {
                            self.span += (1, 0);
                        }
                        self.advance();
                    }
                    if self.is_at_end() {
                        self.add_token2(ERROR, Some("Unterminated String.".to_owned()));
                        return;
                    }
                    self.advance();
                    let value = self
                        .source[self.start + 1..self.current - 1]
                        .to_string();
                    self.add_token2(STRING, Some(value));
                }
                fn matches(&mut self, expected: char) -> bool {
                    if self.is_at_end() {
                        return false;
                    }
                    if self.source[self.current] != expected {
                        return false;
                    }
                    self.current += 1;
                    true
                }
                fn add_token(&mut self, r#type: TokenType) {
                    self.add_token2(r#type, None);
                }
                fn add_token2(&mut self, r#type: TokenType, literal: Option<String>) {
                    let text = self.source[self.start..self.current].to_string();
                    self.tokens.push(Token::new(r#type, text, literal, self.span));
                }
            }
            impl GenericScanner<char, Token> for Scanner {
                fn source(&self) -> &impl Source<char> {
                    &self.source
                }
                fn tokens(&mut self) -> &mut Vec<Token> {
                    &mut self.tokens
                }
                fn set_start(&mut self, n: usize) {
                    self.start = n;
                }
                fn current(&self) -> usize {
                    self.current
                }
                fn set_current(&mut self, n: usize) {
                    self.current = n;
                }
                fn span(&mut self) -> &mut Span {
                    &mut self.span
                }
                fn scan_token(&mut self) {
                    (|s: &mut Scanner| {
                        let c = s.advance();
                        match c {
                            '!' => s.add_token(BANG),
                            '=' => s.add_token(EQUAL),
                            '-' => s.add_token(DASH),
                            '>' => s.add_token(GREATER),
                            '/' => {
                                let r#type = if s.matches('>') {
                                    SLASH_GREATER
                                } else {
                                    SLASH
                                };
                                s.add_token(r#type);
                            }
                            '<' => {
                                let r#type = if s.matches('/') { LESS_SLASH } else { LESS };
                                s.add_token(r#type);
                            }
                            ' ' | '\r' | '\t' => {}
                            '\n' => {
                                s.span += (1, 0);
                            }
                            '"' => s.string('"'),
                            '\'' => s.string('\''),
                            c => {
                                if is_alpha(c) {
                                    s.identifier();
                                } else {
                                    s.add_token2(
                                        ERROR,
                                        Some("Unexpected character.".to_owned()),
                                    );
                                }
                            }
                        }
                    })(self);
                }
            }
            trait ToString {
                fn to_string(&self) -> String;
            }
            impl ToString for [char] {
                fn to_string(&self) -> String {
                    let mut text = String::with_capacity(self.len());
                    for ch in self {
                        text.push(*ch)
                    }
                    text
                }
            }
        }
        pub mod tag {
            use std::{fmt::Display, ops::{Deref, DerefMut, Not}};
            use crate::parser::word_parer::ansi::Ansi;
            pub enum TagType {
                #[default]
                Open,
                Close,
                SelfClose,
            }
            #[automatically_derived]
            impl ::core::default::Default for TagType {
                #[inline]
                fn default() -> TagType {
                    Self::Open
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for TagType {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            TagType::Open => "Open",
                            TagType::Close => "Close",
                            TagType::SelfClose => "SelfClose",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for TagType {
                #[inline]
                fn clone(&self) -> TagType {
                    match self {
                        TagType::Open => TagType::Open,
                        TagType::Close => TagType::Close,
                        TagType::SelfClose => TagType::SelfClose,
                    }
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for TagType {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for TagType {
                #[inline]
                fn eq(&self, other: &TagType) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for TagType {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            #[automatically_derived]
            impl ::core::hash::Hash for TagType {
                #[inline]
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    ::core::hash::Hash::hash(&__self_discr, state)
                }
            }
            pub struct Tag {
                pub r#type: TagType,
                ansi: Ansi,
                data: [String; 3],
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Tag {
                #[inline]
                fn clone(&self) -> Tag {
                    Tag {
                        r#type: ::core::clone::Clone::clone(&self.r#type),
                        ansi: ::core::clone::Clone::clone(&self.ansi),
                        data: ::core::clone::Clone::clone(&self.data),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Tag {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Tag {
                #[inline]
                fn eq(&self, other: &Tag) -> bool {
                    self.r#type == other.r#type && self.ansi == other.ansi
                        && self.data == other.data
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for Tag {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<TagType>;
                    let _: ::core::cmp::AssertParamIsEq<Ansi>;
                    let _: ::core::cmp::AssertParamIsEq<[String; 3]>;
                }
            }
            #[automatically_derived]
            impl ::core::hash::Hash for Tag {
                #[inline]
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                    ::core::hash::Hash::hash(&self.r#type, state);
                    ::core::hash::Hash::hash(&self.ansi, state);
                    ::core::hash::Hash::hash(&self.data, state)
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Tag {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "Tag",
                        "type",
                        &self.r#type,
                        "ansi",
                        &self.ansi,
                        "data",
                        &&self.data,
                    )
                }
            }
            impl Default for Tag {
                fn default() -> Self {
                    Self {
                        r#type: TagType::SelfClose,
                        ansi: Ansi::new(),
                        data: [const { String::new() }; 3],
                    }
                }
            }
            impl Tag {
                pub fn with_name(name: &str) -> Self {
                    let mut tag = Tag::default();
                    tag.set_name(name.to_string());
                    tag
                }
                pub fn inherit(&mut self, src: &Tag) {
                    if !src.fg_color().is_empty() && self.fg_color().is_empty() {
                        self.set_fg_color(src.fg_color().clone());
                    }
                    if !src.bg_color().is_empty() && self.bg_color().is_empty() {
                        self.set_bg_color(src.bg_color().clone());
                    }
                    self.ansi.style |= self.ansi.style;
                }
                pub fn clear_styles(&mut self) {
                    self.ansi = Ansi::new();
                }
            }
            impl Tag {
                pub fn set_name(&mut self, value: String) {
                    self.data[0] = value;
                }
                pub fn name(&self) -> &String {
                    &self.data[0]
                }
                pub fn set_custom(&mut self, value: String) {
                    self.data[1] = value;
                }
                pub fn custom(&self) -> &String {
                    &self.data[1]
                }
                pub fn set_src(&mut self, value: String) {
                    self.data[2] = value;
                }
                pub fn src(&self) -> &String {
                    &self.data[2]
                }
            }
            impl Not for Tag {
                type Output = Self;
                fn not(self) -> Self::Output {
                    let mut tag = self;
                    tag.r#type = match tag.r#type {
                        TagType::Open => TagType::Close,
                        TagType::Close => TagType::Open,
                        TagType::SelfClose => TagType::SelfClose,
                    };
                    tag
                }
            }
            impl Deref for Tag {
                type Target = Ansi;
                fn deref(&self) -> &Self::Target {
                    &self.ansi
                }
            }
            impl DerefMut for Tag {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.ansi
                }
            }
            impl Display for Tag {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    if f.alternate() {
                        match self.r#type {
                            TagType::Open | TagType::SelfClose => f.write_str("<"),
                            TagType::Close => f.write_str("</"),
                        }?;
                        f.write_str(self.name())?;
                        match self.r#type {
                            TagType::Open | TagType::Close => f.write_str(">"),
                            TagType::SelfClose => f.write_str("/>"),
                        }?;
                        return Ok(());
                    }
                    if self.name() == "br" {
                        return f.write_str("\n");
                    } else if self.name() == "p" && !self.custom().is_empty() {
                        f.write_fmt(
                            format_args!(
                                "{0}",
                                " ".repeat(self.custom().parse::<usize>().unwrap_or(0)),
                            ),
                        )?;
                    }
                    self.ansi.fmt(f)?;
                    if self.name() == "a" {
                        match self.r#type {
                            TagType::Open => {
                                f.write_str("\x1b]8;;")?;
                                f.write_str(self.custom())?;
                                f.write_str("\x1b\\")?;
                            }
                            TagType::Close => {
                                f.write_str("\x1b]8;;\x1b\\")?;
                            }
                            TagType::SelfClose => {}
                        }
                    }
                    Ok(())
                }
            }
        }
        mod token {
            use crate::common::Span;
            #[allow(non_camel_case_types)]
            #[allow(clippy::upper_case_acronyms)]
            #[repr(u8)]
            /// TokenType represents the different types of tokens that can be recognized by the parser.
            /// It is used to categorize the tokens during the parsing process.
            /// Each variant corresponds to a specific type of token that can be encountered in the input.
            /// The variants are named in uppercase to follow the convention of naming constants.
            /// The variants are:
            /// - LESS: Represents the '<' character.
            /// - GREATER: Represents the '>' character.
            /// - LESS_SLASH: Represents the '</' character.
            /// - SLASH: Represents the '/' character.
            /// - SLASH_GREATER: Represents the '/>' character.
            /// - IDENTIFIER: Represents an identifier token.
            /// - STRING: Represents a string token.
            /// - DASH: Represents a '-' character.
            /// - BANG: Represents a '!' character.
            /// - ERROR: Represents an error token.
            ///
            /// This enum is used in the `Token` struct to specify the type of token being represented.
            /// It is also used in the `Scanner` and `TagParser` modules to categorize tokens during the scanning and parsing process.
            pub enum TokenType {
                LESS,
                GREATER,
                LESS_SLASH,
                SLASH,
                SLASH_GREATER,
                IDENTIFIER,
                EQUAL,
                STRING,
                DASH,
                BANG,
                ERROR,
            }
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            #[allow(clippy::upper_case_acronyms)]
            impl ::core::fmt::Debug for TokenType {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            TokenType::LESS => "LESS",
                            TokenType::GREATER => "GREATER",
                            TokenType::LESS_SLASH => "LESS_SLASH",
                            TokenType::SLASH => "SLASH",
                            TokenType::SLASH_GREATER => "SLASH_GREATER",
                            TokenType::IDENTIFIER => "IDENTIFIER",
                            TokenType::EQUAL => "EQUAL",
                            TokenType::STRING => "STRING",
                            TokenType::DASH => "DASH",
                            TokenType::BANG => "BANG",
                            TokenType::ERROR => "ERROR",
                        },
                    )
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            #[allow(clippy::upper_case_acronyms)]
            impl ::core::clone::Clone for TokenType {
                #[inline]
                fn clone(&self) -> TokenType {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            #[allow(clippy::upper_case_acronyms)]
            impl ::core::marker::Copy for TokenType {}
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            #[allow(clippy::upper_case_acronyms)]
            impl ::core::marker::StructuralPartialEq for TokenType {}
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            #[allow(clippy::upper_case_acronyms)]
            impl ::core::cmp::PartialEq for TokenType {
                #[inline]
                fn eq(&self, other: &TokenType) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            /// The `Token` struct represents a token in the parsing process.
            /// It contains information about the token's type, lexeme, literal value, and the line number
            /// where the token was found in the source code.
            /// The `Token` struct is used to represent a token in the parsing process.
            /// It is created by the `Scanner` module when scanning the input source code.
            /// The `Token` struct is used in the `TagParser` module to represent tokens during the parsing process.
            /// The `Token` struct is also used in the `Stage3` module to represent tokens during the parsing process.
            /// The `Token` struct is used to represent a token in the parsing process.
            /// The `Token` struct is used to represent a token in the parsing process.
            /// The `Token` struct is used to represent a token in the parsing process.
            /// The `Token` struct is used to represent a token in the parsing process.
            pub struct Token {
                pub r#type: TokenType,
                pub lexeme: String,
                pub literal: Option<String>,
                pub span: Span,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Token {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field4_finish(
                        f,
                        "Token",
                        "type",
                        &self.r#type,
                        "lexeme",
                        &self.lexeme,
                        "literal",
                        &self.literal,
                        "span",
                        &&self.span,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Token {
                #[inline]
                fn clone(&self) -> Token {
                    Token {
                        r#type: ::core::clone::Clone::clone(&self.r#type),
                        lexeme: ::core::clone::Clone::clone(&self.lexeme),
                        literal: ::core::clone::Clone::clone(&self.literal),
                        span: ::core::clone::Clone::clone(&self.span),
                    }
                }
            }
            impl Token {
                pub fn new(
                    r#type: TokenType,
                    lexeme: String,
                    literal: Option<String>,
                    span: Span,
                ) -> Self {
                    Token {
                        r#type,
                        literal,
                        lexeme,
                        span,
                    }
                }
            }
        }
        pub struct TagParser {
            stack: Vec<Tag>,
        }
        impl Default for TagParser {
            fn default() -> Self {
                Self::new()
            }
        }
        impl TagParser {
            pub fn new() -> Self {
                Self {
                    stack: Vec::with_capacity(8),
                }
            }
            pub fn parse(&mut self, source: Fragment) -> Result<Tag, Error> {
                let mut scanner = Scanner::new(source.clone());
                let tokens = scanner.scan_tokens();
                let open = &tokens[0].r#type;
                let close = &tokens[tokens.len() - 1].r#type;
                let tag_type = match (open, close) {
                    (LESS, GREATER) => TagType::Open,
                    (LESS, SLASH_GREATER) => TagType::SelfClose,
                    (LESS_SLASH, GREATER) => TagType::Close,
                    _ => {
                        return Err(
                            Error::new(
                                ErrorType::InvalidTag,
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Invalid tag: {0:?} {1:?}", open, close),
                                    );
                                    res
                                }),
                                tokens[0].span,
                            ),
                        );
                    }
                };
                let span = tokens[0].span;
                let mut tokens: VecDeque<_> = tokens[1..tokens.len()].to_vec().into();
                let mut next = || {
                    if tokens.is_empty() {
                        return Err(
                            Error::new(
                                ErrorType::InvalidTag,
                                "Unexpected end of input".to_string(),
                                span,
                            ),
                        );
                    }
                    Ok(tokens.pop_front().unwrap())
                };
                let token = next()?;
                expect(&token, IDENTIFIER, ErrorType::InvalidTagName)?;
                let mut tag = Tag::default();
                tag.r#type = tag_type;
                tag.set_name(token.lexeme.clone());
                match tag.name().as_str() {
                    "b" | "strong" => {
                        tag.set_bold(true);
                    }
                    "d" | "dim" => {
                        tag.set_dim(true);
                    }
                    "i" => {
                        tag.set_italics(true);
                    }
                    "u" | "ins" => {
                        tag.set_under(true);
                    }
                    "k" | "blink" => {
                        tag.set_blink(true);
                    }
                    "r" => {
                        tag.set_negative(true);
                    }
                    "h" => {
                        tag.set_hidden(true);
                    }
                    "s" | "del" => {
                        tag.set_strike(true);
                    }
                    _ => {}
                }
                let mut token = next()?;
                while token.r#type == IDENTIFIER {
                    match token.lexeme.as_str() {
                        "b" | "bold" => {
                            tag.set_bold(true);
                            token = next()?;
                            if token.r#type == EQUAL {
                                token = next()?;
                                expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                let s = token.literal.unwrap();
                                if s == "false" {
                                    tag.set_bold(false);
                                } else if s == "true" {} else {}
                                token = next()?;
                            }
                        }
                        "d" | "dim" => {
                            tag.set_dim(true);
                            token = next()?;
                            if token.r#type == EQUAL {
                                token = next()?;
                                expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                let s = token.literal.unwrap();
                                if s == "false" {
                                    tag.set_dim(false);
                                } else if s == "true" {} else {}
                                token = next()?;
                            }
                        }
                        "k" | "blink" => {
                            tag.set_blink(true);
                            token = next()?;
                            if token.r#type == EQUAL {
                                token = next()?;
                                expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                let s = token.literal.unwrap();
                                if s == "false" {
                                    tag.set_blink(false);
                                } else if s == "true" {} else {}
                                token = next()?;
                            }
                        }
                        "h" | "hidden" | "hide" | "invisible" => {
                            tag.set_hidden(true);
                            token = next()?;
                            if token.r#type == EQUAL {
                                token = next()?;
                                expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                let s = token.literal.unwrap();
                                if s == "false" {
                                    tag.set_hidden(false);
                                } else if s == "true" {} else {}
                                token = next()?;
                            }
                        }
                        "s" | "strike" | "strike-through" => {
                            tag.set_strike(true);
                            token = next()?;
                            if token.r#type == EQUAL {
                                token = next()?;
                                expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                let s = token.literal.unwrap();
                                if s == "false" {
                                    tag.set_strike(false);
                                } else if s == "true" {} else {}
                                token = next()?;
                            }
                        }
                        "i" | "italics" => {
                            tag.set_italics(true);
                            token = next()?;
                            if token.r#type == EQUAL {
                                token = next()?;
                                expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                let s = token.literal.unwrap();
                                if s == "false" {
                                    tag.set_italics(false);
                                } else if s == "true" {} else {}
                                token = next()?;
                            }
                        }
                        "r" | "invert" | "reverse" | "negative" => {
                            tag.set_negative(true);
                            token = next()?;
                            if token.r#type == EQUAL {
                                token = next()?;
                                expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                let s = token.literal.unwrap();
                                if s == "false" {
                                    tag.set_negative(false);
                                } else if s == "true" {} else {}
                                token = next()?;
                            }
                        }
                        "u" | "under" | "underline" => {
                            tag.set_under(true);
                            token = next()?;
                            if token.r#type == EQUAL {
                                token = next()?;
                                expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                let s = token.literal.unwrap();
                                if s == "false" {
                                    tag.set_under(false);
                                } else if s == "true" {} else {}
                                token = next()?;
                            }
                        }
                        "uu" | "double-under" | "double-underline" => {
                            tag.set_double_under(true);
                            token = next()?;
                            if token.r#type == EQUAL {
                                token = next()?;
                                expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                let s = token.literal.unwrap();
                                if s == "false" {
                                    tag.set_double_under(false);
                                } else if s == "true" {} else {}
                                token = next()?;
                            }
                        }
                        "c" | "fg" => {
                            tag.set_fg_color(Color::new());
                            token = next()?;
                            if token.r#type == EQUAL {
                                token = next()?;
                                expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                let color: Color = (
                                    ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!("{0}{1}", "f", token.literal.unwrap()),
                                        );
                                        res
                                    }),
                                    token.span,
                                )
                                    .try_into()?;
                                tag.set_fg_color(color.into());
                                token = next()?;
                            }
                        }
                        "x" | "bg" => {
                            tag.set_bg_color(Color::new());
                            token = next()?;
                            if token.r#type == EQUAL {
                                token = next()?;
                                expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                let color: Color = (
                                    ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!("{0}{1}", "b", token.literal.unwrap()),
                                        );
                                        res
                                    }),
                                    token.span,
                                )
                                    .try_into()?;
                                tag.set_bg_color(color.into());
                                token = next()?;
                            }
                        }
                        "black" | "blue" | "cyan" | "green" | "magenta" | "red" | "white"
                        | "yellow" => {
                            let color = |pre: &str| -> Result<_, _> {
                                let c: Color = (
                                    ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!("{1}{0}", token.lexeme, pre),
                                        );
                                        res
                                    }),
                                    token.span - (0, 1),
                                )
                                    .try_into()?;
                                Ok(c)
                            };
                            if tag.name() == "c" {
                                tag.set_fg_color(color("f")?);
                            } else if tag.name() == "x" {
                                tag.set_bg_color(color("b")?);
                            }
                            {
                                token = next()?;
                                if token.r#type == EQUAL {
                                    token = next()?;
                                    expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                    token = next()?;
                                }
                            };
                        }
                        "fixed" => {
                            token = next()?;
                            if token.r#type == EQUAL {
                                token = next()?;
                                expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                token = next()?;
                            }
                            let color = |pre: &str| -> Result<_, _> {
                                let c: Color = (
                                    ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!("{1}fixed({0})", token.literal.unwrap(), pre),
                                        );
                                        res
                                    }),
                                    token.span.unquote() - (0, 7),
                                )
                                    .try_into()?;
                                Ok(c)
                            };
                            if tag.name() == "c" {
                                tag.set_fg_color(color("f")?);
                            } else if tag.name() == "x" {
                                tag.set_bg_color(color("b")?);
                            }
                        }
                        "rgb" => {
                            token = next()?;
                            if token.r#type == EQUAL {
                                token = next()?;
                                expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                token = next()?;
                            }
                            let color = |pre: &str| -> Result<_, _> {
                                let c: Color = (
                                    ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!("{1}rgb({0})", token.literal.unwrap(), pre),
                                        );
                                        res
                                    }),
                                    token.span.unquote() - (0, 5),
                                )
                                    .try_into()?;
                                Ok(c)
                            };
                            if tag.name() == "c" {
                                tag.set_fg_color(color("f")?);
                            } else if tag.name() == "x" {
                                tag.set_bg_color(color("b")?);
                            }
                        }
                        "double" => {
                            if match tag.name().as_str() {
                                "u" | "ins" => true,
                                _ => false,
                            } {
                                tag.set_double_under(true);
                                token = next()?;
                                if token.r#type == EQUAL {
                                    token = next()?;
                                    expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                    let s = token.literal.unwrap();
                                    if s == "false" {
                                        tag.set_under(true);
                                    } else if s == "true" {} else {}
                                    token = next()?;
                                }
                            } else {
                                {
                                    token = next()?;
                                    if token.r#type == EQUAL {
                                        token = next()?;
                                        expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                        token = next()?;
                                    }
                                };
                            }
                        }
                        "n" => {
                            if tag.name() == "br" {
                                {
                                    tag.set_custom(String::new());
                                    token = next()?;
                                    if token.r#type == EQUAL {
                                        token = next()?;
                                        expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                        tag.set_custom(token.literal.unwrap());
                                        token = next()?;
                                    }
                                };
                            } else {
                                {
                                    token = next()?;
                                    if token.r#type == EQUAL {
                                        token = next()?;
                                        expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                        token = next()?;
                                    }
                                };
                            }
                        }
                        "href" => {
                            if tag.name() == "a" {
                                {
                                    tag.set_custom(String::new());
                                    token = next()?;
                                    if token.r#type == EQUAL {
                                        token = next()?;
                                        expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                        tag.set_custom(token.literal.unwrap());
                                        token = next()?;
                                    }
                                };
                            } else {
                                {
                                    token = next()?;
                                    if token.r#type == EQUAL {
                                        token = next()?;
                                        expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                        token = next()?;
                                    }
                                };
                            }
                        }
                        "name" => {
                            if tag.name() == "let" {
                                {
                                    tag.set_custom(String::new());
                                    token = next()?;
                                    if token.r#type == EQUAL {
                                        token = next()?;
                                        expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                        tag.set_custom(token.literal.unwrap());
                                        token = next()?;
                                    }
                                };
                            } else {
                                {
                                    token = next()?;
                                    if token.r#type == EQUAL {
                                        token = next()?;
                                        expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                        token = next()?;
                                    }
                                };
                            }
                        }
                        "tab" => {
                            if tag.name() == "p" {
                                {
                                    tag.set_custom(String::new());
                                    token = next()?;
                                    if token.r#type == EQUAL {
                                        token = next()?;
                                        expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                        tag.set_custom(token.literal.unwrap());
                                        token = next()?;
                                    }
                                };
                            } else {
                                {
                                    token = next()?;
                                    if token.r#type == EQUAL {
                                        token = next()?;
                                        expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                        token = next()?;
                                    }
                                };
                            }
                        }
                        "src" => {
                            tag.set_src(String::new());
                            token = next()?;
                            if token.r#type == EQUAL {
                                token = next()?;
                                expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                tag.set_src(token.literal.unwrap());
                                token = next()?;
                            }
                        }
                        _ => {
                            {
                                token = next()?;
                                if token.r#type == EQUAL {
                                    token = next()?;
                                    expect(&token, STRING, ErrorType::InvalidTagPropertyValue)?;
                                    token = next()?;
                                }
                            };
                        }
                    }
                }
                match tag.r#type {
                    TagType::Open => {
                        self.stack.push(tag.clone());
                    }
                    TagType::SelfClose => {}
                    TagType::Close => {
                        if let Some(last) = self.stack.pop() {
                            if last.name() != tag.name() {
                                return Err(
                                    Error::new(
                                        ErrorType::InvalidTag,
                                        ::alloc::__export::must_use({
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "Mismatched tag: {0:?} {1:?}",
                                                    tag.name(),
                                                    last.name(),
                                                ),
                                            );
                                            res
                                        }),
                                        token.span,
                                    ),
                                );
                            }
                        }
                    }
                }
                Ok(tag)
            }
        }
        fn expect(
            token: &Token,
            expected: token::TokenType,
            error: ErrorType,
        ) -> Result<(), Error> {
            if token.r#type == expected {
                Ok(())
            } else {
                Err(
                    Error::new(
                        error,
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Expected {0:?}, but found {1:?}",
                                    expected,
                                    token.r#type,
                                ),
                            );
                            res
                        }),
                        token.span,
                    ),
                )
            }
        }
    }
    pub mod word_parer {
        use super::chunk::{Chunk, ChunkData};
        use super::color::{Ansi256, Color, Rgb};
        use super::tag_parer::tag::Tag;
        use crate::common::Span;
        use crate::error::Error;
        use crate::scanner::GenericScanner;
        use crate::splitter::fragment::Fragment;
        use scanner::Scanner;
        use std::collections::VecDeque;
        use token::Token;
        pub mod ansi {
            use std::fmt::{Display, Write};
            use std::io::Write as _;
            use std::ops::{AddAssign, Not, SubAssign};
            use crate::parser::color::Color;
            pub struct Ansi {
                pub(crate) style: u32,
                colors: [Color; 2],
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Ansi {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Ansi",
                        "style",
                        &self.style,
                        "colors",
                        &&self.colors,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Ansi {
                #[inline]
                fn clone(&self) -> Ansi {
                    Ansi {
                        style: ::core::clone::Clone::clone(&self.style),
                        colors: ::core::clone::Clone::clone(&self.colors),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Ansi {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Ansi {
                #[inline]
                fn eq(&self, other: &Ansi) -> bool {
                    self.style == other.style && self.colors == other.colors
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for Ansi {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    let _: ::core::cmp::AssertParamIsEq<u32>;
                    let _: ::core::cmp::AssertParamIsEq<[Color; 2]>;
                }
            }
            #[automatically_derived]
            impl ::core::hash::Hash for Ansi {
                #[inline]
                fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                    ::core::hash::Hash::hash(&self.style, state);
                    ::core::hash::Hash::hash(&self.colors, state)
                }
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
            impl Ansi {
                const L: u32 = 31;
                pub fn set_bold(&mut self, value: bool) {
                    set_style(&mut self.style, 0, value);
                    set_style(&mut self.style, 0 + 15, !value);
                    set_style(&mut self.style, 0 + 1, false);
                    set_style(&mut self.style, 0 + 16, false);
                }
                pub fn bold(&self) -> bool {
                    get_style(&self.style, 0)
                }
                pub fn clear_bold(&self) -> bool {
                    get_style(&self.style, 0 + 15)
                }
                pub fn set_dim(&mut self, value: bool) {
                    set_style(&mut self.style, 0 + 1, value);
                    set_style(&mut self.style, 0 + 16, !value);
                    set_style(&mut self.style, 0, false);
                    set_style(&mut self.style, 0 + 15, false);
                }
                pub fn dim(&self) -> bool {
                    get_style(&self.style, 0 + 1)
                }
                pub fn clear_dim(&self) -> bool {
                    get_style(&self.style, 0 + 16)
                }
                pub fn clear_bd(&mut self) {
                    set_style(&mut self.style, 0 + 15, true);
                    set_style(&mut self.style, 0 + 16, true);
                }
                pub fn set_under(&mut self, value: bool) {
                    set_style(&mut self.style, 2, value);
                    set_style(&mut self.style, 2 + 15, !value);
                    set_style(&mut self.style, 2 + 1, false);
                    set_style(&mut self.style, 2 + 16, false);
                }
                pub fn under(&self) -> bool {
                    get_style(&self.style, 2)
                }
                pub fn clear_under(&self) -> bool {
                    get_style(&self.style, 2 + 15)
                }
                pub fn set_double_under(&mut self, value: bool) {
                    set_style(&mut self.style, 2 + 1, value);
                    set_style(&mut self.style, 2 + 16, !value);
                    set_style(&mut self.style, 2, false);
                    set_style(&mut self.style, 2 + 15, false);
                }
                pub fn double_under(&self) -> bool {
                    get_style(&self.style, 2 + 1)
                }
                pub fn clear_double_under(&self) -> bool {
                    get_style(&self.style, 2 + 16)
                }
                pub fn clear_ud(&mut self) {
                    set_style(&mut self.style, 2 + 15, true);
                    set_style(&mut self.style, 2 + 16, true);
                }
                pub fn set_blink(&mut self, value: bool) {
                    set_style(&mut self.style, 4, value);
                    set_style(&mut self.style, 4 + 15, !value);
                }
                pub fn blink(&self) -> bool {
                    get_style(&self.style, 4)
                }
                pub fn clear_blink(&self) -> bool {
                    get_style(&self.style, 4 + 15)
                }
                pub fn set_hidden(&mut self, value: bool) {
                    set_style(&mut self.style, 5, value);
                    set_style(&mut self.style, 5 + 15, !value);
                }
                pub fn hidden(&self) -> bool {
                    get_style(&self.style, 5)
                }
                pub fn clear_hidden(&self) -> bool {
                    get_style(&self.style, 5 + 15)
                }
                pub fn set_strike(&mut self, value: bool) {
                    set_style(&mut self.style, 6, value);
                    set_style(&mut self.style, 6 + 15, !value);
                }
                pub fn strike(&self) -> bool {
                    get_style(&self.style, 6)
                }
                pub fn clear_strike(&self) -> bool {
                    get_style(&self.style, 6 + 15)
                }
                pub fn set_italics(&mut self, value: bool) {
                    set_style(&mut self.style, 7, value);
                    set_style(&mut self.style, 7 + 15, !value);
                }
                pub fn italics(&self) -> bool {
                    get_style(&self.style, 7)
                }
                pub fn clear_italics(&self) -> bool {
                    get_style(&self.style, 7 + 15)
                }
                pub fn set_negative(&mut self, value: bool) {
                    set_style(&mut self.style, 8, value);
                    set_style(&mut self.style, 8 + 15, !value);
                }
                pub fn negative(&self) -> bool {
                    get_style(&self.style, 8)
                }
                pub fn clear_negative(&self) -> bool {
                    get_style(&self.style, 8 + 15)
                }
                pub fn set_fg_color(&mut self, value: Color) {
                    self.colors[0] = value;
                }
                pub fn fg_color(&self) -> &Color {
                    &self.colors[0]
                }
                pub fn set_bg_color(&mut self, value: Color) {
                    self.colors[1] = value;
                }
                pub fn bg_color(&self) -> &Color {
                    &self.colors[1]
                }
            }
            impl Display for Ansi {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    if f.alternate() {
                        return f
                            .write_fmt(
                                format_args!("\"{0}\"", self.to_string().escape_debug()),
                            );
                    }
                    let mut buf = Vec::with_capacity(128);
                    let _ = buf.write(b"\x1b[");
                    if self.dim() {
                        let _ = buf.write(b"2;");
                    }
                    if self.bold() {
                        let _ = buf.write(b"1;");
                    }
                    if self.clear_bold() {
                        let _ = buf.write(b"22;");
                    }
                    if self.italics() {
                        let _ = buf.write(b"3;");
                    }
                    if self.clear_italics() {
                        let _ = buf.write(b"23;");
                    }
                    if self.under() {
                        let _ = buf.write(b"4;");
                    }
                    if self.double_under() {
                        let _ = buf.write(b"21;");
                    }
                    if self.clear_under() {
                        let _ = buf.write(b"24;");
                    }
                    if self.blink() {
                        let _ = buf.write(b"5;");
                    }
                    if self.clear_blink() {
                        let _ = buf.write(b"25;");
                    }
                    if self.negative() {
                        let _ = buf.write(b"7;");
                    }
                    if self.clear_negative() {
                        let _ = buf.write(b"27;");
                    }
                    if self.hidden() {
                        let _ = buf.write(b"8;");
                    }
                    if self.clear_hidden() {
                        let _ = buf.write(b"28;");
                    }
                    if self.strike() {
                        let _ = buf.write(b"9;");
                    }
                    if self.clear_strike() {
                        let _ = buf.write(b"29;");
                    }
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
                        f.write_char(ch as char)?;
                    }
                    Ok(())
                }
            }
            impl AddAssign for Ansi {
                fn add_assign(&mut self, rhs: Self) {
                    let lhs_styles = (self.style >> (Ansi::L - 8)) & 0x1FF;
                    let lhs_clear_styles = (self.style >> (Ansi::L - 23)) & 0x1FF;
                    let rhs_styles = (rhs.style >> (Ansi::L - 8)) & 0x1FF;
                    let rhs_clear_styles = (rhs.style >> (Ansi::L - 23)) & 0x1FF;
                    let styles = lhs_styles & rhs_clear_styles;
                    let clear_styles = lhs_clear_styles & rhs_styles;
                    self.style &= clear_styles << (Ansi::L - 8);
                    self.style |= styles << (Ansi::L - 23);
                    self.colors = rhs.colors;
                }
            }
            impl SubAssign for Ansi {
                fn sub_assign(&mut self, rhs: Self) {
                    let style = self.style & rhs.style;
                    self.style ^= style;
                }
            }
            impl Not for Ansi {
                type Output = Ansi;
                fn not(mut self) -> Self::Output {
                    let styles = (self.style >> (Ansi::L - 8)) & 0x1FF;
                    let clear_styles = (self.style >> (Ansi::L - 23)) & 0x1FF;
                    self.style &= clear_styles << (Ansi::L - 8);
                    self.style |= styles << (Ansi::L - 23);
                    self
                }
            }
        }
        mod scanner {
            use super::token::Token;
            use crate::common::Span;
            use crate::scanner::{GenericScanner, Source};
            use crate::splitter::fragment::Fragment;
            pub struct Scanner {
                source: Vec<char>,
                tokens: Vec<Token>,
                start: usize,
                current: usize,
                span: Span,
            }
            impl Scanner {
                pub fn new(mut source: Fragment) -> Self {
                    source.span.tie_start();
                    Self {
                        source: source.lexeme.chars().collect(),
                        tokens: ::alloc::vec::Vec::new(),
                        start: 0,
                        current: 0,
                        span: source.span,
                    }
                }
                fn escape(&mut self) {
                    if self.is_at_end() {
                        return;
                    }
                    let c = self.advance();
                    let mut scan_until = |limit: u8, tester: fn(c: char) -> bool| {
                        let mut i = 0;
                        while i < limit && tester(self.peek()) {
                            self.advance();
                            i += 1;
                        }
                    };
                    fn is_hexdigit(c: char) -> bool {
                        c.is_ascii_hexdigit()
                    }
                    fn is_octdigit(c: char) -> bool {
                        match c {
                            '0'..'8' => true,
                            _ => false,
                        }
                    }
                    match c {
                        'a' => self.add_token('\x07'),
                        'b' => self.add_token('\x08'),
                        'e' => self.add_token('\x1b'),
                        'f' => self.add_token('\x0c'),
                        'n' => self.add_token('\x0a'),
                        'r' => self.add_token('\x0d'),
                        't' => self.add_token('\t'),
                        'v' => self.add_token('\x0b'),
                        '\\' => self.add_token('\\'),
                        '<' => self.add_token('<'),
                        '>' => self.add_token('>'),
                        '0' => {
                            scan_until(3, is_octdigit);
                            let num = u32::from_str_radix(&self.text()[2..], 8).unwrap();
                            self.add_token(
                                char::from_u32(num).unwrap_or(char::REPLACEMENT_CHARACTER),
                            );
                        }
                        'x' => {
                            scan_until(2, is_hexdigit);
                            let num = u32::from_str_radix(&self.text()[2..], 16)
                                .unwrap();
                            self.add_token(
                                char::from_u32(num).unwrap_or(char::REPLACEMENT_CHARACTER),
                            );
                        }
                        'u' => {
                            scan_until(4, is_hexdigit);
                            let num = u32::from_str_radix(&self.text()[2..], 16)
                                .unwrap();
                            self.add_token(
                                char::from_u32(num).unwrap_or(char::REPLACEMENT_CHARACTER),
                            );
                        }
                        'U' => {
                            scan_until(8, is_hexdigit);
                            let num = u32::from_str_radix(&self.text()[2..], 16)
                                .unwrap();
                            self.add_token(
                                char::from_u32(num).unwrap_or(char::REPLACEMENT_CHARACTER),
                            );
                        }
                        _ => {
                            self.add_token('\\');
                            self.add_token(c);
                        }
                    };
                }
                fn text(&self) -> String {
                    self.source[self.start..self.current].to_string()
                }
                fn add_token(&mut self, literal: char) {
                    self.tokens.push(Token::new(literal, self.span));
                    self.span.tie_end();
                }
            }
            impl GenericScanner<char, Token> for Scanner {
                fn source(&self) -> &impl Source<char> {
                    &self.source
                }
                fn tokens(&mut self) -> &mut Vec<Token> {
                    &mut self.tokens
                }
                fn set_start(&mut self, n: usize) {
                    self.start = n;
                }
                fn current(&self) -> usize {
                    self.current
                }
                fn set_current(&mut self, n: usize) {
                    self.current = n;
                }
                fn span(&mut self) -> &mut Span {
                    &mut self.span
                }
                fn scan_token(&mut self) {
                    (|s: &mut Scanner| {
                        let c = s.advance();
                        match c {
                            '\\' => s.escape(),
                            '\x1b' => {
                                s.add_token('\x1b');
                            }
                            _ => s.add_token(c),
                        }
                    })(self);
                }
            }
            trait ToString {
                fn to_string(&self) -> String;
            }
            impl ToString for [char] {
                fn to_string(&self) -> String {
                    let mut text = String::with_capacity(self.len());
                    for ch in self {
                        text.push(*ch)
                    }
                    text
                }
            }
        }
        mod token {
            use crate::common::Span;
            pub struct Token {
                pub literal: char,
                #[allow(dead_code)]
                pub span: Span,
            }
            #[automatically_derived]
            impl ::core::default::Default for Token {
                #[inline]
                fn default() -> Token {
                    Token {
                        literal: ::core::default::Default::default(),
                        span: ::core::default::Default::default(),
                    }
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Token {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Token",
                        "literal",
                        &self.literal,
                        "span",
                        &&self.span,
                    )
                }
            }
            impl Token {
                pub fn new(literal: char, span: Span) -> Self {
                    Token { literal, span }
                }
            }
        }
        pub struct WordParser;
        impl Default for WordParser {
            fn default() -> Self {
                Self::new()
            }
        }
        impl WordParser {
            pub fn new() -> Self {
                Self
            }
            pub fn parse(&self, source: Fragment) -> Result<Vec<Chunk>, Error> {
                let mut scanner = Scanner::new(source);
                let tokens = scanner.scan_tokens();
                let mut chunks = ::alloc::vec::Vec::new();
                let mut i = 0;
                let len = tokens.len();
                loop {
                    if i >= len {
                        break;
                    }
                    let c = tokens[i].literal;
                    if c == '\x1b' && tokens[i + 1].literal == '[' {
                        let g = i;
                        i += 2;
                        let h = i;
                        if !match tokens[i].literal {
                            '\x30'..='\x39' | '\x3b' | '\x40'..='\x7e' => true,
                            _ => false,
                        } {
                            while i < len && tokens[i].literal != '\x1b' {
                                i += 1;
                            }
                            let word = tokens[h..i].to_string();
                            chunks
                                .push(Chunk {
                                    data: ChunkData::Word(word),
                                    span: tokens[h..i].to_span(),
                                });
                            break;
                        }
                        while i < len
                            && !match tokens[i].literal {
                                '\x40'..='\x7e' => true,
                                _ => false,
                            }
                        {
                            i += 1;
                        }
                        if tokens[i].literal == 'm' {
                            let escape_sequence = tokens[h..i].to_string();
                            if let Ok(tag) = self.ansi_to_tag(escape_sequence) {
                                chunks
                                    .push(Chunk {
                                        data: ChunkData::Tag(tag),
                                        span: tokens[g..i].to_span(),
                                    });
                            }
                        } else {
                            while i < len && tokens[i].literal != '\x1b' {
                                i += 1;
                            }
                            let word = tokens[h..i].to_string();
                            chunks
                                .push(Chunk {
                                    data: ChunkData::Word(word),
                                    span: tokens[h..i].to_span(),
                                });
                        }
                        i += 1;
                        continue;
                    } else {
                        let h = i;
                        while i < len && tokens[i].literal != '\x1b' {
                            i += 1;
                        }
                        let word = tokens[h..i].to_string();
                        chunks
                            .push(Chunk {
                                data: ChunkData::Word(word),
                                span: tokens[h..i].to_span(),
                            })
                    }
                    i += 1;
                }
                Ok(chunks)
            }
            fn ansi_to_tag(&self, source: String) -> Result<Tag, i8> {
                let mut parts = source
                    .split(';')
                    .map(|x| {
                        if x.is_empty() {
                            Ok(0)
                        } else {
                            x.parse::<i32>().map_err(|_| 0)
                        }
                    })
                    .collect::<VecDeque<_>>();
                let mut next = || parts.pop_front().unwrap_or(Err(-1));
                let mut tag = Tag::default();
                loop {
                    let num = next();
                    let num = match num {
                        Ok(n) => n,
                        Err(-1) => break,
                        Err(_) => return Err(0),
                    };
                    match num {
                        -1 => break,
                        0 => tag = Tag::default(),
                        1 => {
                            tag.set_bold(true);
                        }
                        2 => {
                            tag.set_dim(true);
                        }
                        22 => {
                            tag.clear_bd();
                        }
                        3 => {
                            tag.set_italics(true);
                        }
                        23 => {
                            tag.set_italics(false);
                        }
                        4 => {
                            tag.set_under(true);
                        }
                        21 => {
                            tag.set_double_under(true);
                        }
                        24 => {
                            tag.clear_ud();
                        }
                        5 => {
                            tag.set_blink(true);
                        }
                        25 => {
                            tag.set_blink(false);
                        }
                        7 => {
                            tag.set_negative(true);
                        }
                        27 => {
                            tag.set_negative(false);
                        }
                        8 => {
                            tag.set_hidden(true);
                        }
                        28 => {
                            tag.set_hidden(false);
                        }
                        9 => {
                            tag.set_strike(true);
                        }
                        29 => {
                            tag.set_strike(false);
                        }
                        fg @ 30..=37 | fg @ 39 => {
                            tag.set_fg_color(
                                Color::four_bit({
                                    if fg > 255 {
                                        255u8
                                    } else if fg < 0 {
                                        0u8
                                    } else {
                                        fg as u8
                                    }
                                }),
                            )
                        }
                        bg @ 40..=47 | bg @ 49 => {
                            tag.set_bg_color(
                                Color::four_bit({
                                    if bg > 255 {
                                        255u8
                                    } else if bg < 0 {
                                        0u8
                                    } else {
                                        bg as u8
                                    }
                                }),
                            )
                        }
                        38 => {
                            let num = next()?;
                            if num == 2 {
                                let r = next()?;
                                let g = next()?;
                                let b = next()?;
                                tag.set_fg_color(
                                    Color::Rgb(
                                        Rgb(
                                            38,
                                            {
                                                if r > 255 { 255u8 } else if r < 0 { 0u8 } else { r as u8 }
                                            },
                                            {
                                                if g > 255 { 255u8 } else if g < 0 { 0u8 } else { g as u8 }
                                            },
                                            {
                                                if b > 255 { 255u8 } else if b < 0 { 0u8 } else { b as u8 }
                                            },
                                        ),
                                    ),
                                );
                            }
                            if num == 5 {
                                let fixed = next()?;
                                tag.set_fg_color(
                                    Color::Ansi256(
                                        Ansi256(
                                            38,
                                            {
                                                if fixed > 255 {
                                                    255u8
                                                } else if fixed < 0 {
                                                    0u8
                                                } else {
                                                    fixed as u8
                                                }
                                            },
                                        ),
                                    ),
                                );
                            }
                        }
                        48 => {
                            let num = next()?;
                            if num == 2 {
                                let r = next()?;
                                let g = next()?;
                                let b = next()?;
                                tag.set_fg_color(
                                    Color::Rgb(
                                        Rgb(
                                            48,
                                            {
                                                if r > 255 { 255u8 } else if r < 0 { 0u8 } else { r as u8 }
                                            },
                                            {
                                                if g > 255 { 255u8 } else if g < 0 { 0u8 } else { g as u8 }
                                            },
                                            {
                                                if b > 255 { 255u8 } else if b < 0 { 0u8 } else { b as u8 }
                                            },
                                        ),
                                    ),
                                );
                            }
                            if num == 5 {
                                let fixed = next()?;
                                tag.set_fg_color(
                                    Color::Ansi256(
                                        Ansi256(
                                            48,
                                            {
                                                if fixed > 255 {
                                                    255u8
                                                } else if fixed < 0 {
                                                    0u8
                                                } else {
                                                    fixed as u8
                                                }
                                            },
                                        ),
                                    ),
                                );
                            }
                        }
                        _ => {}
                    }
                }
                tag.set_name("$ansi".to_string());
                Ok(tag)
            }
        }
        trait Transform {
            fn to_string(&self) -> String;
            fn to_span(&self) -> Span;
        }
        impl Transform for [Token] {
            fn to_string(&self) -> String {
                let mut text = String::with_capacity(self.len());
                for token in self {
                    text.push(token.literal)
                }
                text
            }
            fn to_span(&self) -> Span {
                let mut span = Span::inserted();
                for token in self {
                    if span == Span::inserted() {
                        span = token.span;
                    } else {
                        span += token.span;
                    }
                }
                span
            }
        }
    }
    #[doc(hidden)]
    pub struct Parser {}
    impl Default for Parser {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Parser {
        pub fn new() -> Self {
            Self {}
        }
        pub fn parse(&self, frags: Vec<Fragment>) -> Result<Vec<Chunk>> {
            let mut tag_parser = tag_parer::TagParser::new();
            let word_parer = WordParser::new();
            let mut chunks = ::alloc::vec::Vec::new();
            for frag in frags {
                let span = frag.span;
                match frag.r#type {
                    FragmentType::Error => {
                        {
                            ::std::io::_eprint(
                                format_args!("Error fragment encountered: {0:?}\n", frag),
                            );
                        };
                    }
                    FragmentType::Tag => {
                        chunks
                            .push(Chunk {
                                data: ChunkData::Tag(tag_parser.parse(frag)?),
                                span,
                            });
                    }
                    FragmentType::Whitespace => {
                        chunks
                            .push(Chunk {
                                data: ChunkData::WhiteSpace(frag.lexeme),
                                span,
                            });
                    }
                    FragmentType::Word => {
                        let chs = word_parer.parse(frag)?;
                        chunks.extend_from_slice(&chs);
                    }
                }
            }
            Ok(chunks)
        }
    }
}
mod resolver {
    use std::{collections::HashMap, rc::Rc};
    use crate::{
        BUILTIN_TAGS, common::Span,
        parser::{
            chunk::{Chunk, ChunkData},
            tag_parer::tag::{Tag, TagType},
        },
    };
    use document::{Document, Node};
    pub mod document {
        use std::{
            cell::RefCell, fmt::{self, Debug, Display, Formatter},
            rc::Rc,
        };
        use crate::{
            common::Span,
            parser::{
                chunk::{Chunk, ChunkData},
                tag_parer::tag::{Tag, TagType},
            },
        };
        pub use node::Node;
        mod display {
            use std::fmt::Display;
            /// Indentation token
            struct Token {
                /// Is followed by a brother
                siblings: bool,
                /// Is intermediate while printing children
                children: bool,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Token {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Token",
                        "siblings",
                        &self.siblings,
                        "children",
                        &&self.children,
                    )
                }
            }
            impl Display for Token {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    let Token { siblings, children } = self;
                    f.write_fmt(
                        format_args!(
                            "{0}",
                            match (siblings, children) {
                                (true, true) => "│   ",
                                (true, false) => "├── ",
                                (false, true) => "    ",
                                (false, false) => "└── ",
                            },
                        ),
                    )
                }
            }
            impl Token {
                /// Create a new indentation token
                fn new(siblings: bool) -> Self {
                    Token { siblings, children: false }
                }
                /// Set children flag before starting displaying children
                fn set_children(&mut self) {
                    self.children = true;
                }
            }
            /// Manages the state during the display operation
            pub struct Indentation {
                tokens: Vec<Token>,
                ignore_root: bool,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Indentation {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Indentation",
                        "tokens",
                        &self.tokens,
                        "ignore_root",
                        &&self.ignore_root,
                    )
                }
            }
            impl Display for Indentation {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    let first: usize = if self.ignore_root { 1 } else { 0 };
                    for token in &self.tokens[first..] {
                        f.write_fmt(format_args!("{0}", token))?;
                    }
                    Ok(())
                }
            }
            impl Indentation {
                /// Creates a new indentation handler
                pub fn new(ignore_root: bool) -> Self {
                    Indentation {
                        tokens: Vec::new(),
                        ignore_root,
                    }
                }
                /// Adds a new layer of indentation
                pub fn indent(&mut self, siblings: bool) -> &mut Self {
                    let len = self.tokens.len();
                    if len > 0 {
                        self.tokens[len - 1].set_children();
                    }
                    self.tokens.push(Token::new(siblings));
                    self
                }
                /// Removes the last layer of indentation
                pub fn deindent(&mut self) -> &mut Self {
                    self.tokens.pop();
                    self
                }
            }
        }
        mod iter {
            use std::rc::Rc;
            use super::Node;
            pub struct Children {
                front: Option<Rc<Node>>,
                back: Option<Rc<Node>>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Children {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Children",
                        "front",
                        &self.front,
                        "back",
                        &&self.back,
                    )
                }
            }
            impl Clone for Children {
                fn clone(&self) -> Self {
                    Self {
                        front: self.front.clone(),
                        back: self.back.clone(),
                    }
                }
            }
            impl Iterator for Children {
                type Item = Rc<Node>;
                fn next(&mut self) -> Option<Self::Item> {
                    if self.front == self.back {
                        let node = self.front.take();
                        self.back = None;
                        node
                    } else {
                        let node = self.front.take();
                        self.front = node.as_ref().and_then(Node::next_sibling);
                        node
                    }
                }
            }
            /// Open or close edge of a node.
            pub enum Edge {
                /// Open.
                Open(Rc<Node>),
                /// Close.
                Close(Rc<Node>),
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Edge {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        Edge::Open(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Open",
                                &__self_0,
                            )
                        }
                        Edge::Close(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "Close",
                                &__self_0,
                            )
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Edge {
                #[inline]
                fn clone(&self) -> Edge {
                    match self {
                        Edge::Open(__self_0) => {
                            Edge::Open(::core::clone::Clone::clone(__self_0))
                        }
                        Edge::Close(__self_0) => {
                            Edge::Close(::core::clone::Clone::clone(__self_0))
                        }
                    }
                }
            }
            impl Eq for Edge {}
            impl PartialEq for Edge {
                fn eq(&self, other: &Self) -> bool {
                    match (self, other) {
                        (Edge::Open(a), Edge::Open(b))
                        | (Edge::Close(a), Edge::Close(b)) => a == b,
                        _ => false,
                    }
                }
            }
            /// Iterator which traverses a subtree.
            pub struct Traverse {
                root: Option<Rc<Node>>,
                edge: Option<Edge>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Traverse {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Traverse",
                        "root",
                        &self.root,
                        "edge",
                        &&self.edge,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Traverse {
                #[inline]
                fn clone(&self) -> Traverse {
                    Traverse {
                        root: ::core::clone::Clone::clone(&self.root),
                        edge: ::core::clone::Clone::clone(&self.edge),
                    }
                }
            }
            impl Iterator for Traverse {
                type Item = Edge;
                fn next(&mut self) -> Option<Self::Item> {
                    match &self.edge {
                        None => {
                            if let Some(root) = &self.root {
                                self.edge = Some(Edge::Open(root.clone()));
                            }
                        }
                        Some(Edge::Open(node)) => {
                            if let Some(first_child) = node.first_child() {
                                self.edge = Some(Edge::Open(first_child));
                            } else {
                                self.edge = Some(Edge::Close(node.clone()));
                            }
                        }
                        Some(Edge::Close(node)) => {
                            if *node == self.root.clone().unwrap() {
                                self.root = None;
                                self.edge = None;
                            } else if let Some(next_sibling) = node.next_sibling() {
                                self.edge = Some(Edge::Open(next_sibling));
                            } else {
                                self.edge = node.parent().map(Edge::Close);
                            }
                        }
                    }
                    self.edge.clone()
                }
            }
            /// Iterator over a node and its descendants.
            pub struct Descendants(Traverse);
            #[automatically_derived]
            impl ::core::fmt::Debug for Descendants {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Descendants",
                        &&self.0,
                    )
                }
            }
            impl Clone for Descendants {
                fn clone(&self) -> Self {
                    Descendants(self.0.clone())
                }
            }
            impl Iterator for Descendants {
                type Item = Rc<Node>;
                fn next(&mut self) -> Option<Self::Item> {
                    for edge in &mut self.0 {
                        if let Edge::Open(node) = edge {
                            return Some(node);
                        }
                    }
                    None
                }
            }
            /// Iterator over ancestors.
            pub struct Ancestors(Option<Rc<Node>>);
            #[automatically_derived]
            impl ::core::fmt::Debug for Ancestors {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Ancestors",
                        &&self.0,
                    )
                }
            }
            impl Clone for Ancestors {
                fn clone(&self) -> Self {
                    Ancestors(self.0.clone())
                }
            }
            impl Iterator for Ancestors {
                type Item = Rc<Node>;
                fn next(&mut self) -> Option<Self::Item> {
                    let node = self.0.take();
                    self.0 = node.as_ref().and_then(Node::parent);
                    node
                }
            }
            /// Iterator over previous siblings.
            pub struct PrevSiblings(Option<Rc<Node>>);
            #[automatically_derived]
            impl ::core::fmt::Debug for PrevSiblings {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "PrevSiblings",
                        &&self.0,
                    )
                }
            }
            impl Clone for PrevSiblings {
                fn clone(&self) -> Self {
                    PrevSiblings(self.0.clone())
                }
            }
            impl Iterator for PrevSiblings {
                type Item = Rc<Node>;
                fn next(&mut self) -> Option<Self::Item> {
                    let node = self.0.take();
                    self.0 = node.as_ref().and_then(Node::prev_sibling);
                    node
                }
            }
            /// Iterator over next siblings.
            pub struct NextSiblings(Option<Rc<Node>>);
            #[automatically_derived]
            impl ::core::fmt::Debug for NextSiblings {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "NextSiblings",
                        &&self.0,
                    )
                }
            }
            impl Clone for NextSiblings {
                fn clone(&self) -> Self {
                    NextSiblings(self.0.clone())
                }
            }
            impl Iterator for NextSiblings {
                type Item = Rc<Node>;
                fn next(&mut self) -> Option<Self::Item> {
                    let node = self.0.take();
                    self.0 = node.as_ref().and_then(Node::next_sibling);
                    node
                }
            }
            /// Iterator over first children.
            pub struct FirstChildren(Option<Rc<Node>>);
            #[automatically_derived]
            impl ::core::fmt::Debug for FirstChildren {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "FirstChildren",
                        &&self.0,
                    )
                }
            }
            impl Clone for FirstChildren {
                fn clone(&self) -> Self {
                    FirstChildren(self.0.clone())
                }
            }
            impl Iterator for FirstChildren {
                type Item = Rc<Node>;
                fn next(&mut self) -> Option<Self::Item> {
                    let node = self.0.take();
                    self.0 = node.as_ref().and_then(Node::first_child);
                    node
                }
            }
            /// Iterator over last children.
            pub struct LastChildren(Option<Rc<Node>>);
            #[automatically_derived]
            impl ::core::fmt::Debug for LastChildren {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "LastChildren",
                        &&self.0,
                    )
                }
            }
            impl Clone for LastChildren {
                fn clone(&self) -> Self {
                    LastChildren(self.0.clone())
                }
            }
            impl Iterator for LastChildren {
                type Item = Rc<Node>;
                fn next(&mut self) -> Option<Self::Item> {
                    let node = self.0.take();
                    self.0 = node.as_ref().and_then(Node::last_child);
                    node
                }
            }
            impl Node {
                /// Returns an iterator over children.
                pub fn children(self: &Rc<Node>) -> Children {
                    Children {
                        front: self.first_child(),
                        back: self.last_child(),
                    }
                }
                /// Returns an iterator which traverses the subtree starting at this node.
                pub fn traverse(self: &Rc<Node>) -> Traverse {
                    Traverse {
                        root: Some(self.clone()),
                        edge: None,
                    }
                }
                /// Returns an iterator over this node and its descendants.
                pub fn descendants(self: &Rc<Node>) -> Descendants {
                    Descendants(self.traverse())
                }
                /// Returns an iterator over ancestors.
                pub fn ancestors(self: &Rc<Node>) -> Ancestors {
                    Ancestors(Some(self.clone()))
                }
            }
        }
        mod node {
            use std::{
                cell::{Ref, RefCell},
                rc::{Rc, Weak},
            };
            use super::Document;
            use crate::parser::chunk::Chunk;
            struct Kin {
                parent: Option<u32>,
                prev_sibling: Option<u32>,
                next_sibling: Option<u32>,
                children: Option<(u32, u32)>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Kin {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field4_finish(
                        f,
                        "Kin",
                        "parent",
                        &self.parent,
                        "prev_sibling",
                        &self.prev_sibling,
                        "next_sibling",
                        &self.next_sibling,
                        "children",
                        &&self.children,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Kin {
                #[inline]
                fn clone(&self) -> Kin {
                    Kin {
                        parent: ::core::clone::Clone::clone(&self.parent),
                        prev_sibling: ::core::clone::Clone::clone(&self.prev_sibling),
                        next_sibling: ::core::clone::Clone::clone(&self.next_sibling),
                        children: ::core::clone::Clone::clone(&self.children),
                    }
                }
            }
            #[doc(hidden)]
            pub struct Node {
                id: u32,
                kin: RefCell<Kin>,
                doc: Weak<Document>,
                chunk: RefCell<Chunk>,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Node {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field4_finish(
                        f,
                        "Node",
                        "id",
                        &self.id,
                        "kin",
                        &self.kin,
                        "doc",
                        &self.doc,
                        "chunk",
                        &&self.chunk,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Node {
                #[inline]
                fn clone(&self) -> Node {
                    Node {
                        id: ::core::clone::Clone::clone(&self.id),
                        kin: ::core::clone::Clone::clone(&self.kin),
                        doc: ::core::clone::Clone::clone(&self.doc),
                        chunk: ::core::clone::Clone::clone(&self.chunk),
                    }
                }
            }
            impl Node {
                pub fn new(id: u32, chunk: Chunk, doc: Weak<Document>) -> Self {
                    Self {
                        id,
                        kin: RefCell::new(Kin {
                            parent: None,
                            prev_sibling: None,
                            next_sibling: None,
                            children: None,
                        }),
                        doc,
                        chunk: RefCell::new(chunk),
                    }
                }
                pub fn id(self: &Rc<Node>) -> u32 {
                    self.id
                }
                pub fn doc(self: &Rc<Node>) -> Rc<Document> {
                    self.doc.upgrade().unwrap()
                }
                /// Returns the chunk of this node.
                pub fn chunk(self: &Rc<Node>) -> &RefCell<Chunk> {
                    &self.chunk
                }
                fn axis<F>(self: &Rc<Node>, f: F) -> Option<Rc<Node>>
                where
                    F: FnOnce(Ref<Kin>) -> Option<u32>,
                {
                    f(self.kin.borrow()).map(|id| self.doc().get(id))
                }
                /// Returns the parent of this node.
                pub fn parent(self: &Rc<Node>) -> Option<Rc<Self>> {
                    self.axis(|node| node.parent)
                }
                /// Returns the previous sibling of this node.
                pub fn prev_sibling(self: &Rc<Node>) -> Option<Rc<Self>> {
                    self.axis(|node| node.prev_sibling)
                }
                /// Returns the next sibling of this node.
                pub fn next_sibling(self: &Rc<Node>) -> Option<Rc<Self>> {
                    self.axis(|node| node.next_sibling)
                }
                /// Returns the first child of this node.
                pub fn first_child(self: &Rc<Node>) -> Option<Rc<Self>> {
                    self.axis(|node| node.children.map(|(id, _)| id))
                }
                /// Returns the last child of this node.
                pub fn last_child(self: &Rc<Node>) -> Option<Rc<Self>> {
                    self.axis(|node| node.children.map(|(_, id)| id))
                }
                /// Returns true if this node has children.
                pub fn has_children(self: &Rc<Node>) -> bool {
                    self.kin.borrow().children.is_some()
                }
                /// Appends a new child to this node.
                pub fn append(self: &Rc<Node>, value: Chunk) -> Rc<Node> {
                    let id = self.doc().orphan(value).id;
                    self.append_id(id)
                }
                /// Prepends a new child to this node.
                pub fn prepend(self: &Rc<Node>, value: Chunk) -> Rc<Node> {
                    let id = self.doc().orphan(value).id;
                    self.prepend_id(id)
                }
                /// Inserts a new sibling before this node.
                ///
                /// # Panics
                ///
                /// Panics if this node is an orphan.
                pub fn insert_before(self: &Rc<Node>, value: Chunk) -> Rc<Node> {
                    let id = self.doc().orphan(value).id;
                    self.insert_id_before(id)
                }
                /// Inserts a new sibling after this node.
                ///
                /// # Panics
                ///
                /// Panics if this node is an orphan.
                pub fn insert_after(self: &Rc<Node>, value: Chunk) -> Rc<Node> {
                    let id = self.doc().orphan(value).id;
                    self.insert_id_after(id)
                }
                /// Detaches this node from its parent.
                pub fn detach(self: &Rc<Node>) {
                    let mut kin = self.kin.borrow_mut();
                    let parent_id = match kin.parent {
                        Some(id) => id,
                        None => return,
                    };
                    let prev_sibling_id = kin.prev_sibling;
                    let next_sibling_id = kin.next_sibling;
                    {
                        kin.parent = None;
                        kin.prev_sibling = None;
                        kin.next_sibling = None;
                    }
                    if let Some(id) = prev_sibling_id {
                        self.doc().node(id).kin.borrow_mut().next_sibling = next_sibling_id;
                    }
                    if let Some(id) = next_sibling_id {
                        self.doc().node(id).kin.borrow_mut().prev_sibling = prev_sibling_id;
                    }
                    let doc = self.doc();
                    let parent = doc.node(parent_id);
                    let mut parent_kin = parent.kin.borrow_mut();
                    let (first_child_id, last_child_id) = parent_kin.children.unwrap();
                    if first_child_id == last_child_id {
                        parent_kin.children = None;
                    } else if first_child_id == self.id {
                        parent_kin.children = Some((
                            next_sibling_id.unwrap(),
                            last_child_id,
                        ));
                    } else if last_child_id == self.id {
                        parent_kin.children = Some((
                            first_child_id,
                            prev_sibling_id.unwrap(),
                        ));
                    }
                }
                /// Appends a child to this node.
                pub fn append_id(self: &Rc<Node>, new_child_id: u32) -> Rc<Node> {
                    match (&(self.id), &(new_child_id)) {
                        (left_val, right_val) => {
                            if *left_val == *right_val {
                                let kind = ::core::panicking::AssertKind::Ne;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::Some(
                                        format_args!("Cannot append node as a child to itself"),
                                    ),
                                );
                            }
                        }
                    };
                    let mut kin = self.kin.borrow_mut();
                    let last_child_id = kin.children.map(|(_, id)| id);
                    if last_child_id != Some(new_child_id) {
                        {
                            let new_child = self.doc().get(new_child_id);
                            new_child.detach();
                            let mut new_child_kin = new_child.kin.borrow_mut();
                            new_child_kin.parent = Some(self.id);
                            new_child_kin.prev_sibling = last_child_id;
                        }
                        if let Some(id) = last_child_id {
                            self.doc().node(id).kin.borrow_mut().next_sibling = Some(
                                new_child_id,
                            );
                        }
                        kin.children = match kin.children {
                            Some((first_child_id, _)) => {
                                Some((first_child_id, new_child_id))
                            }
                            None => Some((new_child_id, new_child_id)),
                        };
                    }
                    self.doc().get(new_child_id)
                }
                /// Prepends a child to this node.
                pub fn prepend_id(self: &Rc<Node>, new_child_id: u32) -> Rc<Node> {
                    match (&(self.id), &(new_child_id)) {
                        (left_val, right_val) => {
                            if *left_val == *right_val {
                                let kind = ::core::panicking::AssertKind::Ne;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::Some(
                                        format_args!("Cannot prepend node as a child to itself"),
                                    ),
                                );
                            }
                        }
                    };
                    let mut kin = self.kin.borrow_mut();
                    let first_child_id = kin.children.map(|(id, _)| id);
                    if first_child_id != Some(new_child_id) {
                        let new_child = self.doc().get(new_child_id);
                        new_child.detach();
                        let mut new_child_kin = new_child.kin.borrow_mut();
                        new_child_kin.parent = Some(self.id);
                        new_child_kin.next_sibling = first_child_id;
                        if let Some(id) = first_child_id {
                            self.doc().node(id).kin.borrow_mut().prev_sibling = Some(
                                new_child_id,
                            );
                        }
                        kin.children = match kin.children {
                            Some((_, last_child_id)) => {
                                Some((new_child_id, last_child_id))
                            }
                            None => Some((new_child_id, new_child_id)),
                        };
                    }
                    self.doc().get(new_child_id)
                }
                /// Inserts a sibling before this node.
                ///
                /// # Panics
                ///
                /// - Panics if `new_sibling_id` is not valid.
                /// - Panics if this node is an orphan.
                pub fn insert_id_before(
                    self: &Rc<Node>,
                    new_sibling_id: u32,
                ) -> Rc<Node> {
                    match (&(self.id), &(new_sibling_id)) {
                        (left_val, right_val) => {
                            if *left_val == *right_val {
                                let kind = ::core::panicking::AssertKind::Ne;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::Some(
                                        format_args!("Cannot insert node as a sibling of itself"),
                                    ),
                                );
                            }
                        }
                    };
                    let mut kin = self.kin.borrow_mut();
                    let parent_id = kin.parent.unwrap();
                    let prev_sibling_id = kin.prev_sibling;
                    {
                        let new_sibling = self.doc().get(new_sibling_id);
                        new_sibling.detach();
                        let mut new_sibling_kin = new_sibling.kin.borrow_mut();
                        new_sibling_kin.parent = Some(parent_id);
                        new_sibling_kin.prev_sibling = prev_sibling_id;
                        new_sibling_kin.next_sibling = Some(self.id);
                    }
                    if let Some(id) = prev_sibling_id {
                        self.doc().node(id).kin.borrow_mut().next_sibling = Some(
                            new_sibling_id,
                        );
                    }
                    kin.prev_sibling = Some(new_sibling_id);
                    {
                        let doc = self.doc();
                        let parent = doc.node(parent_id);
                        let mut parent_kin = parent.kin.borrow_mut();
                        let (first_child_id, last_child_id) = parent_kin
                            .children
                            .unwrap();
                        if first_child_id == self.id {
                            parent_kin.children = Some((new_sibling_id, last_child_id));
                        }
                    }
                    self.doc().get(new_sibling_id)
                }
                /// Inserts a sibling after this node.
                ///
                /// # Panics
                ///
                /// - Panics if `new_sibling_id` is not valid.
                /// - Panics if this node is an orphan.
                pub fn insert_id_after(
                    self: &Rc<Node>,
                    new_sibling_id: u32,
                ) -> Rc<Node> {
                    match (&(self.id), &(new_sibling_id)) {
                        (left_val, right_val) => {
                            if *left_val == *right_val {
                                let kind = ::core::panicking::AssertKind::Ne;
                                ::core::panicking::assert_failed(
                                    kind,
                                    &*left_val,
                                    &*right_val,
                                    ::core::option::Option::Some(
                                        format_args!("Cannot insert node as a sibling of itself"),
                                    ),
                                );
                            }
                        }
                    };
                    let mut kin = self.kin.borrow_mut();
                    let parent_id = kin.parent.unwrap();
                    let next_sibling_id = kin.next_sibling;
                    {
                        let new_sibling = self.doc().get(new_sibling_id);
                        new_sibling.detach();
                        let mut new_sibling_kin = new_sibling.kin.borrow_mut();
                        new_sibling_kin.parent = Some(parent_id);
                        new_sibling_kin.prev_sibling = Some(self.id);
                        new_sibling_kin.next_sibling = next_sibling_id;
                    }
                    if let Some(id) = next_sibling_id {
                        self.doc().node(id).kin.borrow_mut().prev_sibling = Some(
                            new_sibling_id,
                        );
                    }
                    kin.next_sibling = Some(new_sibling_id);
                    {
                        let doc = self.doc();
                        let parent = doc.node(parent_id);
                        let mut parent_kin = parent.kin.borrow_mut();
                        let (first_child_id, last_child_id) = parent_kin
                            .children
                            .unwrap();
                        if last_child_id == self.id {
                            parent_kin.children = Some((first_child_id, new_sibling_id));
                        }
                    }
                    self.doc().get(new_sibling_id)
                }
                /// Returns the string representation of this node.
                pub fn to_string(self: &Rc<Node>, buf: &mut String) {
                    if self.has_children() {
                        let tag_chunk = self.chunk.borrow();
                        let tag = tag_chunk.data.tag().unwrap();
                        buf.push_str(tag.to_string().as_str());
                        for child in self.children() {
                            child.to_string(buf);
                        }
                    } else {
                        buf.push_str(self.chunk.borrow().data.to_string().as_str());
                    }
                }
                pub fn clear_styles(self: &Rc<Node>) {
                    if self.chunk.borrow().is_tag() {
                        let mut tag_chunk = self.chunk.borrow_mut();
                        let tag = tag_chunk.data.tag_mut().unwrap();
                        tag.clear_styles();
                        for child in self.children() {
                            child.clear_styles();
                        }
                    }
                }
            }
            impl PartialEq for Node {
                fn eq(&self, other: &Self) -> bool {
                    let kin = self.kin.borrow();
                    let other_kin = other.kin.borrow();
                    self.id == other.id && kin.parent == other_kin.parent
                        && kin.prev_sibling == other_kin.prev_sibling
                        && kin.next_sibling == other_kin.next_sibling
                        && kin.children == other_kin.children
                        && self.chunk == other.chunk
                }
            }
            impl Eq for Node {}
        }
        #[doc(hidden)]
        pub struct Document {
            nodes: RefCell<Vec<Rc<Node>>>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Document {
            #[inline]
            fn clone(&self) -> Document {
                Document {
                    nodes: ::core::clone::Clone::clone(&self.nodes),
                }
            }
        }
        impl Document {
            pub fn new() -> Rc<Self> {
                let doc = Rc::new(Self {
                    nodes: RefCell::new(::alloc::vec::Vec::new()),
                });
                let mut tag = Tag::default();
                tag.set_name("$root".to_string());
                tag.r#type = TagType::Open;
                let node = Rc::new(
                    Node::new(
                        0,
                        Chunk {
                            data: ChunkData::Tag(tag),
                            span: Span::inserted(),
                        },
                        Rc::downgrade(&Rc::clone(&doc)),
                    ),
                );
                {
                    let mut nodes = doc.nodes.borrow_mut();
                    nodes.push(node)
                }
                doc
            }
            pub fn get(self: &Rc<Self>, id: u32) -> Rc<Node> {
                self.nodes.borrow()[id as usize].clone()
            }
            pub fn node(self: &Rc<Self>, id: u32) -> Rc<Node> {
                self.get(id)
            }
            pub fn root(self: &Rc<Document>) -> Rc<Node> {
                self.get(0)
            }
            pub fn root2(&self) -> Rc<Node> {
                self.nodes.borrow()[0].clone()
            }
            pub fn orphan(self: &Rc<Document>, chunk: Chunk) -> Rc<Node> {
                let doc = Rc::downgrade(&Rc::clone(self));
                let mut nodes = self.nodes.borrow_mut();
                let id = nodes.len();
                let node = Rc::new(
                    Node::new(id.try_into().expect("maximum nodes exceeded"), chunk, doc),
                );
                nodes.push(node.clone());
                node
            }
            pub fn len(self: &Rc<Self>) -> usize {
                self.nodes.borrow().len()
            }
        }
        impl Debug for Document {
            fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
                use iter::Edge;
                if f.alternate() {
                    f.write_fmt(format_args!("Tree {{"))?;
                    for edge in self.root2().traverse() {
                        match edge {
                            Edge::Open(node) if node.has_children() => {
                                f.write_fmt(format_args!(" {0:?} => {{", node.chunk()))?;
                            }
                            Edge::Open(node) if node.next_sibling().is_some() => {
                                f.write_fmt(format_args!(" {0:?},", node.chunk()))?;
                            }
                            Edge::Open(node) => {
                                f.write_fmt(format_args!(" {0:?}", node.chunk()))?;
                            }
                            Edge::Close(node) if node.has_children() => {
                                if node.next_sibling().is_some() {
                                    f.write_fmt(format_args!(" }},"))?;
                                } else {
                                    f.write_fmt(format_args!(" }}"))?;
                                }
                            }
                            _ => {}
                        }
                    }
                    f.write_fmt(format_args!(" }}"))
                } else {
                    f.debug_struct("Tree").field("vec", &self.nodes).finish()
                }
            }
        }
        impl Display for Document {
            fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
                use display::Indentation;
                use iter::Edge;
                let mut indent: Indentation = Indentation::new(true);
                for edge in self.root2().traverse() {
                    match edge {
                        Edge::Open(node) if node.has_children() => {
                            indent.indent(node.next_sibling().is_some());
                            f.write_fmt(
                                format_args!("{1}{0:#}\n", node.chunk().borrow(), indent),
                            )?;
                        }
                        Edge::Open(node) => {
                            indent.indent(node.next_sibling().is_some());
                            f.write_fmt(
                                format_args!("{1}{0:#}\n", node.chunk().borrow(), indent),
                            )?;
                            indent.deindent();
                        }
                        Edge::Close(node) if node.has_children() => {
                            indent.deindent();
                        }
                        _ => {}
                    }
                }
                Ok(())
            }
        }
    }
    #[doc(hidden)]
    pub struct Resolver {}
    impl Default for Resolver {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Resolver {
        pub fn new() -> Self {
            Self {}
        }
        pub fn resolve(&mut self, chunks: Vec<Chunk>) -> Rc<Document> {
            let tree = Document::new();
            let mut node = tree.root();
            for chunk in chunks.iter() {
                match &chunk.data {
                    ChunkData::Tag(tag) => {
                        match tag.r#type {
                            TagType::Open => {
                                node = node.append(chunk.clone());
                            }
                            TagType::Close => {
                                node.append(chunk.clone());
                                node = node.parent().unwrap();
                            }
                            TagType::SelfClose => {
                                node.append(chunk.clone());
                            }
                        }
                    }
                    ChunkData::WhiteSpace(_) => {
                        node.append(chunk.clone());
                    }
                    ChunkData::Word(_) => {
                        node.append(chunk.clone());
                    }
                }
            }
            let node = tree.root();
            {
                let mut detachables = ::alloc::vec::Vec::new();
                let mut bindings: HashMap<String, Tag> = HashMap::new();
                Resolver::resolve_bindings(&mut bindings, &node, &mut detachables);
                for node in &detachables {
                    node.detach();
                }
            }
            {
                let mut detachables = ::alloc::vec::Vec::new();
                Resolver::optimize_ws(&node, &mut detachables);
                for node in &detachables {
                    node.detach();
                }
            }
            Resolver::_resolve(&node, "$root");
            tree
        }
        /// Resolve all declared bindings: <let />
        fn resolve_bindings(
            bindings: &mut HashMap<String, Tag>,
            node: &Rc<Node>,
            detachables: &mut Vec<Rc<Node>>,
        ) {
            for child in node.children() {
                let mut child_chunk = child.chunk().borrow_mut();
                if child_chunk.is_tag() {
                    let tag = child_chunk.tag_mut().unwrap();
                    let name = tag.name().clone();
                    if !BUILTIN_TAGS.contains(&name.as_str()) {
                        for ansector in child.ancestors() {
                            if let Some(binding) = bindings
                                .get(
                                    &::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!("{0}/{1}", ansector.id(), name),
                                        );
                                        res
                                    }),
                                )
                            {
                                tag.inherit(binding);
                                break;
                            }
                        }
                    }
                    if !tag.src().is_empty() {
                        for ansector in child.ancestors() {
                            if let Some(binding) = bindings
                                .get(
                                    &::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!("{0}/{1}", ansector.id(), tag.src()),
                                        );
                                        res
                                    }),
                                )
                            {
                                tag.inherit(binding);
                                break;
                            }
                        }
                    }
                    if name == "let" {
                        let tag = tag.clone();
                        let name = tag.custom();
                        let id = node.id();
                        bindings
                            .insert(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("{0}/{1}", id, name),
                                    );
                                    res
                                }),
                                tag,
                            );
                        detachables.push(child.clone());
                    }
                }
                Resolver::resolve_bindings(bindings, &child, detachables);
            }
        }
        /// Optimizes Excess Whitespace
        fn optimize_ws(node: &Rc<Node>, detachables: &mut Vec<Rc<Node>>) {
            for child in node.children() {
                let mut child_chunk = child.chunk().borrow_mut();
                if child_chunk.is_ws() {
                    if child.id() == 1 {
                        detachables.push(child.clone());
                    } else if child.id() as usize == child.doc().len() - 1
                        && child_chunk.ws().is_some_and(|s| s.contains("\n"))
                    {
                        child_chunk.data = ChunkData::WhiteSpace("\n".to_string());
                    } else {
                        child_chunk.data = ChunkData::WhiteSpace(" ".to_string());
                    }
                    if let Some(first) = child
                        .next_sibling()
                        .and_then(|next| next.first_child())
                    {
                        if first.chunk().borrow().is_ws() {
                            detachables.push(first);
                        }
                    } else if child
                        .next_sibling()
                        .is_some_and(|node| {
                            node.chunk()
                                .borrow()
                                .is_tag_and(|tag| tag.r#type == TagType::Close)
                        })
                    {
                        if let Some(next) = node.next_sibling() {
                            if next.chunk().borrow().is_ws() {
                                detachables.push(next);
                            }
                        }
                    } else if let Some(next) = child.next_sibling() {
                        if next.chunk().borrow().is_ws() {
                            detachables.push(next);
                        }
                    }
                } else if child_chunk.is_tag() {
                    let name = child_chunk.tag().unwrap().name();
                    if match name.as_str() {
                        "p" | "ziyy" | "$root" | "div" => true,
                        _ => false,
                    } {
                        if let Some(first) = child.first_child() {
                            if first.chunk().borrow().is_ws() {
                                detachables.push(first);
                            }
                        }
                    } else if name == "br" {
                        if let Some(prev) = child.prev_sibling() {
                            if prev.chunk().borrow().is_ws() {
                                detachables.push(prev);
                            }
                        }
                        if let Some(next) = child.next_sibling() {
                            if next.chunk().borrow().is_ws() {
                                detachables.push(next);
                            }
                        }
                    }
                }
                Resolver::optimize_ws(&child, detachables);
            }
        }
        fn _resolve(node: &Rc<Node>, node_name: &str) {
            for child in node.children() {
                let mut child_chunk = child.chunk().borrow_mut();
                if child_chunk.is_tag() {
                    let tag = child_chunk.tag_mut().unwrap();
                    if tag.r#type == TagType::Open {
                        let name = tag.name();
                        if match name.as_str() {
                            "ziyy" | "p" | "div" => true,
                            _ => false,
                        } {
                            if match node_name {
                                "ziyy" | "$root" | "p" | "div" => true,
                                _ => false,
                            }
                                && node
                                    .first_child()
                                    .is_some_and(|first| first.id() == child.id())
                            {} else {
                                child
                                    .insert_before(Chunk {
                                        data: ChunkData::WhiteSpace("\n".to_string()),
                                        span: Span::inserted(),
                                    });
                            }
                        } else if name == "a" {
                            for grand_child in child.children() {
                                grand_child.clear_styles();
                            }
                        }
                        let last = child.last_child().unwrap();
                        let mut last_chunk = last.chunk().borrow_mut();
                        if last_chunk.is_tag_and(|tag| tag.r#type == TagType::Close) {
                            *last_chunk.tag_mut().unwrap() = !tag.clone();
                        } else {
                            last.insert_after(Chunk {
                                data: ChunkData::Tag(!tag.clone()),
                                span: Span::inserted(),
                            });
                        }
                    }
                    Resolver::_resolve(&child, tag.name());
                } else {
                    Resolver::_resolve(&child, node_name);
                }
            }
        }
    }
}
mod splitter {
    use std::mem::take;
    use fragment::Fragment;
    use fragment::FragmentType::{self, *};
    use crate::common::Span;
    pub mod fragment {
        use crate::common::Span;
        #[allow(clippy::upper_case_acronyms)]
        pub enum FragmentType {
            Error,
            Tag,
            Whitespace,
            Word,
        }
        #[automatically_derived]
        #[allow(clippy::upper_case_acronyms)]
        impl ::core::fmt::Debug for FragmentType {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        FragmentType::Error => "Error",
                        FragmentType::Tag => "Tag",
                        FragmentType::Whitespace => "Whitespace",
                        FragmentType::Word => "Word",
                    },
                )
            }
        }
        #[automatically_derived]
        #[allow(clippy::upper_case_acronyms)]
        impl ::core::clone::Clone for FragmentType {
            #[inline]
            fn clone(&self) -> FragmentType {
                match self {
                    FragmentType::Error => FragmentType::Error,
                    FragmentType::Tag => FragmentType::Tag,
                    FragmentType::Whitespace => FragmentType::Whitespace,
                    FragmentType::Word => FragmentType::Word,
                }
            }
        }
        pub struct Fragment {
            pub r#type: FragmentType,
            pub lexeme: String,
            pub span: Span,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Fragment {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Fragment",
                    "type",
                    &self.r#type,
                    "lexeme",
                    &self.lexeme,
                    "span",
                    &&self.span,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Fragment {
            #[inline]
            fn clone(&self) -> Fragment {
                Fragment {
                    r#type: ::core::clone::Clone::clone(&self.r#type),
                    lexeme: ::core::clone::Clone::clone(&self.lexeme),
                    span: ::core::clone::Clone::clone(&self.span),
                }
            }
        }
        impl Fragment {
            pub fn new(r#type: FragmentType, lexeme: String, span: Span) -> Self {
                Fragment { r#type, lexeme, span }
            }
        }
    }
    #[doc(hidden)]
    pub struct Splitter {
        source: Vec<char>,
        fragments: Vec<Fragment>,
        start: usize,
        current: usize,
        span: Span,
    }
    impl Default for Splitter {
        fn default() -> Self {
            Self::new()
        }
    }
    enum Quote {
        Single,
        Double,
        None,
    }
    impl Splitter {
        pub fn new() -> Self {
            Self {
                source: ::alloc::vec::Vec::new(),
                fragments: ::alloc::vec::Vec::new(),
                start: 0,
                current: 0,
                span: Span::default(),
            }
        }
        pub fn split(&mut self, source: String) -> Vec<Fragment> {
            self.source = source.chars().collect();
            while !self.is_at_end() {
                self.start = self.current;
                let mut c = self.advance();
                match c {
                    ' ' | '\r' | '\t' | '\n' => self.whitespace(),
                    '\\' => {
                        c = self.advance();
                        loop {
                            if self.is_at_end() {
                                break;
                            }
                            if is_whitespace(self.peek()) {
                                break;
                            }
                            if match self.peek() {
                                '<' => true,
                                _ => false,
                            } {
                                break;
                            }
                            if match c {
                                '\\' => true,
                                _ => false,
                            } {
                                self.advance();
                            }
                            self.advance();
                        };
                        self.add_fragment(Word);
                    }
                    '<' => self.tag(),
                    _ => {
                        loop {
                            if self.is_at_end() {
                                break;
                            }
                            if is_whitespace(self.peek()) {
                                break;
                            }
                            if match self.peek() {
                                '<' => true,
                                _ => false,
                            } {
                                break;
                            }
                            if match c {
                                '\\' => true,
                                _ => false,
                            } {
                                self.advance();
                            }
                            self.advance();
                        };
                        self.add_fragment(Word);
                    }
                }
            }
            take(&mut self.fragments)
        }
        fn tag(&mut self) {
            let mut quote = Quote::None;
            loop {
                let c = self.advance();
                if self.is_at_end() {
                    self.add_fragment(Error);
                    return;
                }
                let close = match self.peek() {
                    '>' => true,
                    _ => false,
                };
                let single = match self.peek() {
                    '\'' => true,
                    _ => false,
                };
                let double = match self.peek() {
                    '"' => true,
                    _ => false,
                };
                let esc = match c {
                    '\\' => true,
                    _ => false,
                };
                match quote {
                    Quote::Single => {
                        if single && !esc {
                            quote = Quote::None;
                        }
                    }
                    Quote::Double => {
                        if double && !esc {
                            quote = Quote::None;
                        }
                    }
                    Quote::None => {
                        if close {
                            break;
                        } else if single {
                            quote = Quote::Single;
                        } else if double {
                            quote = Quote::Double;
                        }
                    }
                }
            }
            self.advance();
            self.add_fragment(Tag);
        }
        fn whitespace(&mut self) {
            while is_whitespace(self.peek()) {
                self.advance();
            }
            self.add_fragment(Whitespace);
        }
        fn peek(&self) -> char {
            if self.is_at_end() { '\0' } else { self.source[self.current] }
        }
        fn is_at_end(&self) -> bool {
            self.current >= self.source.len()
        }
        fn advance(&mut self) -> char {
            self.current += 1;
            self.span += (0, 1);
            let ch = self.source[self.current - 1];
            if ch == '\n' {
                self.span += (1, 0);
            }
            ch
        }
        fn add_fragment(&mut self, r#type: FragmentType) {
            let text = self.source[self.start..self.current].to_string();
            self.fragments.push(Fragment::new(r#type, text, self.span));
            self.span.tie_end();
        }
    }
    trait ToString {
        fn to_string(&self) -> String;
    }
    impl ToString for [char] {
        fn to_string(&self) -> String {
            let mut text = String::with_capacity(self.len());
            for ch in self {
                text.push(*ch)
            }
            text
        }
    }
    fn is_whitespace(c: char) -> bool {
        match c {
            ' ' | '\t' | '\n' => true,
            _ => false,
        }
    }
}
static BUILTIN_TAGS: &[&str] = &[
    "a",
    "b",
    "br",
    "d",
    "div",
    "h",
    "i",
    "k",
    "p",
    "r",
    "s",
    "span",
    "table",
    "td",
    "tr",
    "u",
    "ziyy",
];
/// Styles the given text using ziyy.
///
/// # Example
///
/// ```
/// use ziyy::style;
///
/// let styled_text = style("This is <b>bold</b> text");
/// ```
/// # Panics
///
/// This function will panic if the parser encounters an error while parsing the input source.
///
pub fn style<T: AsRef<str>>(source: T) -> String {
    let mut indexer = Indexer::new();
    let source = indexer.index(source.as_ref().to_string());
    let mut splitter = Splitter::new();
    let frags = splitter.split(source);
    let parser = Parser::new();
    let chunks = parser.parse(frags).unwrap();
    let mut resolver = Resolver::new();
    let output = resolver.resolve(chunks);
    let mut buf = String::new();
    output.root().to_string(&mut buf);
    buf
}
