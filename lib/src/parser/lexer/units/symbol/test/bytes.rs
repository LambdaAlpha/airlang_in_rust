use crate::parser::lexer::Token::{self, *};

pub fn expected() -> Vec<Token> {
    vec![
        Bytes(vec![]),
        Bytes(vec![0x00]),
        Bytes(vec![0xff]),
        Bytes(vec![0x00, 0x00]),
        Bytes(vec![0x00, 0x11, 0xff, 0xee]),
    ]
}