mod config;
#[cfg(test)]
mod test;
mod units;

use crate::num::{Float, Integer};
use crate::val::Bytes;
use regex::{Captures, Regex};

use super::{ParseError, ParseResult};

use self::config::AirLexerConfig;

use super::AtomNode;

#[cfg_attr(debug_assertions, derive(Debug))]
pub(crate) enum FlatNode {
    Symbol(String),
    Atom(AtomNode),
}

pub(crate) fn parse(src: &str) -> ParseResult<Vec<FlatNode>> {
    let tokens = lexing(src)?;

    let mut nodes = Vec::with_capacity(tokens.len());
    for token in tokens {
        let node = match token {
            Token::Bool(b) => FlatNode::Atom(AtomNode::Bool(b)),
            Token::Int(i) => FlatNode::Atom(AtomNode::Int(i)),
            Token::Float(f) => FlatNode::Atom(AtomNode::Float(f)),
            Token::Symbol(symbol) => FlatNode::Symbol(symbol),
            Token::Letter(l) => FlatNode::Atom(AtomNode::Letter(l)),
            Token::String(s) => FlatNode::Atom(AtomNode::String(s)),
            Token::Bytes(b) => FlatNode::Atom(AtomNode::Bytes(b)),
            Token::Delimeter(_) => continue,
        };
        nodes.push(node);
    }
    Ok(nodes)
}

#[cfg_attr(debug_assertions, derive(Debug, PartialEq))]
pub(crate) enum Token {
    Delimeter(String),
    Bool(bool),
    Int(Integer),
    Float(Float),
    Symbol(String),
    Letter(String),
    String(String),
    Bytes(Bytes),
}

pub(crate) trait LexerConfig {
    fn dispatch_char(&self, c: char) -> ParseResult<&dyn UnitLexer>;
}

pub(crate) trait UnitLexer {
    fn pattern(&self) -> &Regex;
    fn lexing(&self, captures: &Captures) -> ParseResult<Token>;
}

fn lexing(src: &str) -> ParseResult<Vec<Token>> {
    let config = AirLexerConfig::new();
    let mut tokens = Vec::<Token>::new();
    let mut rest = &src[..];
    while !rest.is_empty() {
        let first = rest.chars().next().unwrap();

        let lexer = config.dispatch_char(first)?;

        let captures = lexer.pattern().captures(rest);
        if captures.is_none() {
            return ParseError::err("pattern matching failed".to_owned());
        }
        let captures = captures.unwrap();

        let m0 = captures.get(0).unwrap();
        rest = &rest[m0.end()..];
        let token = lexer.lexing(&captures)?;
        if !matches!(token, Token::Delimeter(_)) {
            tokens.push(token);
        }
    }
    return Ok(tokens);
}