use super::lexer::{config::AirLexerConfig, Lexer, Token};
use super::pass::deep;
use crate::val::{List, Map, Val};

pub fn stringify_compat(val: &Val) -> String {
    let mut tokens = vec![];
    val_to_tokens_compat(val, &mut tokens);
    let lexer = Lexer {
        config: AirLexerConfig::new(),
    };
    lexer.stringify_tokens(&tokens)
}

pub fn stringify_comfort(val: &Val) -> String {
    let mut tokens = vec![];
    val_to_tokens_comfort(val, &mut tokens);
    let lexer = Lexer {
        config: AirLexerConfig::new(),
    };
    lexer.stringify_tokens(&tokens)
}

pub fn stringify_pretty(val: &Val) -> String {
    let mut tokens = vec![];
    val_to_tokens_pretty(val, &mut tokens, 0);
    let lexer = Lexer {
        config: AirLexerConfig::new(),
    };
    lexer.stringify_tokens(&tokens)
}

fn val_to_tokens_compat(val: &Val, tokens: &mut Vec<Token>) {
    match val {
        Val::Bool(b) => tokens.push(Token::Bool(*b)),
        Val::Int(i) => tokens.push(Token::Int(*i.to_owned())),
        Val::Float(f) => tokens.push(Token::Float(*f.to_owned())),
        Val::String(s) => tokens.push(Token::String(*s.to_owned())),
        Val::Letter(s) => tokens.push(Token::Letter(*s.to_owned())),
        Val::Symbol(s) => tokens.push(Token::Symbol(*s.to_owned())),
        Val::Bytes(b) => {
            tokens.push(Token::Bytes(*b.clone()));
        }
        Val::List(l) => {
            list_to_tokens_compat(&l, tokens);
        }
        Val::Map(m) => {
            map_to_tokens_compat(&m, tokens);
        }
        Val::Ltree(t) => {
            if matches!(t.root, Val::Infix(_)) {
                tokens.push(Token::Symbol(deep::WRAP_LEFT.to_owned()));
                val_to_tokens_compat(&t.root, tokens);
                tokens.push(Token::Symbol(deep::WRAP_RIGHT.to_owned()));
            } else {
                val_to_tokens_compat(&t.root, tokens);
            }
            val_to_tokens_compat(&t.root, tokens);
            list_to_tokens_compat(&t.leaves, tokens);
        }
        Val::Mtree(t) => {
            if matches!(t.root, Val::Infix(_)) {
                tokens.push(Token::Symbol(deep::WRAP_LEFT.to_owned()));
                val_to_tokens_compat(&t.root, tokens);
                tokens.push(Token::Symbol(deep::WRAP_RIGHT.to_owned()));
            } else {
                val_to_tokens_compat(&t.root, tokens);
            }
            map_to_tokens_compat(&t.leaves, tokens);
        }
        Val::Infix(i) => {
            val_to_tokens_compat(&i.left, tokens);
            tokens.push(Token::Delimeter(" ".to_owned()));
            if matches!(i.infix, Val::List(_) | Val::Map(_) | Val::Infix(_)) {
                tokens.push(Token::Symbol(deep::WRAP_LEFT.to_owned()));
                val_to_tokens_compat(&i.infix, tokens);
                tokens.push(Token::Symbol(deep::WRAP_RIGHT.to_owned()));
            } else {
                val_to_tokens_compat(&i.infix, tokens);
            }
            tokens.push(Token::Delimeter(" ".to_owned()));
            if matches!(i.right, Val::List(_) | Val::Map(_) | Val::Infix(_)) {
                tokens.push(Token::Symbol(deep::WRAP_LEFT.to_owned()));
                val_to_tokens_compat(&i.right, tokens);
                tokens.push(Token::Symbol(deep::WRAP_RIGHT.to_owned()));
            } else {
                val_to_tokens_compat(&i.right, tokens);
            }
        }
    }
}

fn val_to_tokens_comfort(val: &Val, tokens: &mut Vec<Token>) {
    match val {
        Val::Bool(b) => tokens.push(Token::Bool(*b)),
        Val::Int(i) => tokens.push(Token::Int(*i.to_owned())),
        Val::Float(f) => tokens.push(Token::Float(*f.to_owned())),
        Val::String(s) => tokens.push(Token::String(*s.to_owned())),
        Val::Letter(s) => tokens.push(Token::Letter(*s.to_owned())),
        Val::Symbol(s) => tokens.push(Token::Symbol(*s.to_owned())),
        Val::Bytes(b) => {
            tokens.push(Token::Bytes(*b.clone()));
        }
        Val::List(l) => {
            list_to_tokens_comfort(&l, tokens);
        }
        Val::Map(m) => {
            map_to_tokens_comfort(&m, tokens);
        }
        Val::Ltree(t) => {
            if matches!(t.root, Val::Infix(_)) {
                tokens.push(Token::Symbol(deep::WRAP_LEFT.to_owned()));
                val_to_tokens_comfort(&t.root, tokens);
                tokens.push(Token::Symbol(deep::WRAP_RIGHT.to_owned()));
            } else {
                val_to_tokens_comfort(&t.root, tokens);
            }
            list_to_tokens_comfort(&t.leaves, tokens);
        }
        Val::Mtree(t) => {
            if matches!(t.root, Val::Infix(_)) {
                tokens.push(Token::Symbol(deep::WRAP_LEFT.to_owned()));
                val_to_tokens_comfort(&t.root, tokens);
                tokens.push(Token::Symbol(deep::WRAP_RIGHT.to_owned()));
            } else {
                val_to_tokens_comfort(&t.root, tokens);
            }
            map_to_tokens_comfort(&t.leaves, tokens);
        }
        Val::Infix(i) => {
            val_to_tokens_comfort(&i.left, tokens);
            tokens.push(Token::Delimeter(" ".to_owned()));
            if matches!(i.infix, Val::List(_) | Val::Map(_) | Val::Infix(_)) {
                tokens.push(Token::Symbol(deep::WRAP_LEFT.to_owned()));
                val_to_tokens_comfort(&i.infix, tokens);
                tokens.push(Token::Symbol(deep::WRAP_RIGHT.to_owned()));
            } else {
                val_to_tokens_comfort(&i.infix, tokens);
            }
            tokens.push(Token::Delimeter(" ".to_owned()));
            if matches!(i.right, Val::List(_) | Val::Map(_) | Val::Infix(_)) {
                tokens.push(Token::Symbol(deep::WRAP_LEFT.to_owned()));
                val_to_tokens_comfort(&i.right, tokens);
                tokens.push(Token::Symbol(deep::WRAP_RIGHT.to_owned()));
            } else {
                val_to_tokens_comfort(&i.right, tokens);
            }
        }
    }
}

fn val_to_tokens_pretty(val: &Val, tokens: &mut Vec<Token>, indent: usize) {
    match val {
        Val::Bool(b) => tokens.push(Token::Bool(*b)),
        Val::Int(i) => tokens.push(Token::Int(*i.to_owned())),
        Val::Float(f) => tokens.push(Token::Float(*f.to_owned())),
        Val::String(s) => tokens.push(Token::String(*s.to_owned())),
        Val::Letter(s) => tokens.push(Token::Letter(*s.to_owned())),
        Val::Symbol(s) => tokens.push(Token::Symbol(*s.to_owned())),
        Val::Bytes(b) => {
            tokens.push(Token::Bytes(*b.clone()));
        }
        Val::List(l) => {
            list_to_tokens_pretty(&l, tokens, indent);
        }
        Val::Map(m) => {
            map_to_tokens_pretty(&m, tokens, indent);
        }
        Val::Ltree(t) => {
            if matches!(t.root, Val::Infix(_)) {
                tokens.push(Token::Symbol(deep::WRAP_LEFT.to_owned()));
                val_to_tokens_pretty(&t.root, tokens, indent);
                tokens.push(Token::Symbol(deep::WRAP_RIGHT.to_owned()));
            } else {
                val_to_tokens_pretty(&t.root, tokens, indent);
            }
            list_to_tokens_pretty(&t.leaves, tokens, indent);
        }
        Val::Mtree(t) => {
            if matches!(t.root, Val::Infix(_)) {
                tokens.push(Token::Symbol(deep::WRAP_LEFT.to_owned()));
                val_to_tokens_pretty(&t.root, tokens, indent);
                tokens.push(Token::Symbol(deep::WRAP_RIGHT.to_owned()));
            } else {
                val_to_tokens_pretty(&t.root, tokens, indent);
            }
            map_to_tokens_pretty(&t.leaves, tokens, indent);
        }
        Val::Infix(i) => {
            val_to_tokens_pretty(&i.left, tokens, indent);
            tokens.push(Token::Delimeter(" ".to_owned()));
            if matches!(i.infix, Val::List(_) | Val::Map(_) | Val::Infix(_)) {
                tokens.push(Token::Symbol(deep::WRAP_LEFT.to_owned()));
                val_to_tokens_pretty(&i.infix, tokens, indent);
                tokens.push(Token::Symbol(deep::WRAP_RIGHT.to_owned()));
            } else {
                val_to_tokens_pretty(&i.infix, tokens, indent);
            }
            tokens.push(Token::Delimeter(" ".to_owned()));
            if matches!(i.right, Val::List(_) | Val::Map(_) | Val::Infix(_)) {
                tokens.push(Token::Symbol(deep::WRAP_LEFT.to_owned()));
                val_to_tokens_pretty(&i.right, tokens, indent);
                tokens.push(Token::Symbol(deep::WRAP_RIGHT.to_owned()));
            } else {
                val_to_tokens_pretty(&i.right, tokens, indent);
            }
        }
    }
}

fn list_to_tokens_compat(list: &List, tokens: &mut Vec<Token>) {
    tokens.push(Token::Symbol(deep::LIST_LEFT.to_owned()));
    for val in list.iter() {
        val_to_tokens_compat(val, tokens);
        tokens.push(Token::Symbol(deep::SEPERATOR.to_owned()));
    }
    if !list.is_empty() {
        tokens.pop();
    }
    tokens.push(Token::Symbol(deep::LIST_RIGHT.to_owned()))
}

fn list_to_tokens_comfort(list: &List, tokens: &mut Vec<Token>) {
    tokens.push(Token::Symbol(deep::LIST_LEFT.to_owned()));
    for val in list.iter() {
        val_to_tokens_comfort(val, tokens);
        tokens.push(Token::Symbol(deep::SEPERATOR.to_owned()));
        tokens.push(Token::Delimeter(" ".to_owned()));
    }
    if !list.is_empty() {
        tokens.pop();
        tokens.pop();
    }
    tokens.push(Token::Symbol(deep::LIST_RIGHT.to_owned()))
}

fn list_to_tokens_pretty(list: &List, tokens: &mut Vec<Token>, indent: usize) {
    tokens.push(Token::Symbol(deep::LIST_LEFT.to_owned()));
    if list.is_empty() {
        tokens.push(Token::Symbol(deep::LIST_RIGHT.to_owned()));
        return;
    }
    tokens.push(Token::Delimeter("\n".to_owned()));
    for val in list.iter() {
        tokens.push(Token::Delimeter("  ".repeat(indent + 1)));
        val_to_tokens_pretty(val, tokens, indent + 1);
        tokens.push(Token::Symbol(deep::SEPERATOR.to_owned()));
        tokens.push(Token::Delimeter("\n".to_owned()));
    }
    tokens.push(Token::Delimeter("  ".repeat(indent)));
    tokens.push(Token::Symbol(deep::LIST_RIGHT.to_owned()))
}

fn map_to_tokens_compat(map: &Map, tokens: &mut Vec<Token>) {
    tokens.push(Token::Symbol(deep::MAP_LEFT.to_owned()));
    for pair in map.iter() {
        val_to_tokens_compat(pair.0, tokens);
        tokens.push(Token::Symbol(deep::MAP_KV_SEPERATOR.to_owned()));
        val_to_tokens_compat(pair.1, tokens);
        tokens.push(Token::Symbol(deep::SEPERATOR.to_owned()));
    }
    if !map.is_empty() {
        tokens.pop();
    }
    tokens.push(Token::Symbol(deep::MAP_RIGHT.to_owned()))
}

fn map_to_tokens_comfort(map: &Map, tokens: &mut Vec<Token>) {
    tokens.push(Token::Symbol(deep::MAP_LEFT.to_owned()));
    for pair in map.iter() {
        val_to_tokens_comfort(pair.0, tokens);
        tokens.push(Token::Symbol(deep::MAP_KV_SEPERATOR.to_owned()));
        tokens.push(Token::Delimeter(" ".to_owned()));
        val_to_tokens_comfort(pair.1, tokens);
        tokens.push(Token::Symbol(deep::SEPERATOR.to_owned()));
        tokens.push(Token::Delimeter(" ".to_owned()));
    }
    if !map.is_empty() {
        tokens.pop();
        tokens.pop();
    }
    tokens.push(Token::Symbol(deep::MAP_RIGHT.to_owned()))
}

fn map_to_tokens_pretty(map: &Map, tokens: &mut Vec<Token>, indent: usize) {
    tokens.push(Token::Symbol(deep::MAP_LEFT.to_owned()));
    if map.is_empty() {
        tokens.push(Token::Symbol(deep::MAP_RIGHT.to_owned()));
        return;
    }
    tokens.push(Token::Delimeter("\n".to_owned()));
    for pair in map.iter() {
        tokens.push(Token::Delimeter("  ".repeat(indent + 1)));
        val_to_tokens_pretty(pair.0, tokens, indent + 1);
        tokens.push(Token::Symbol(deep::MAP_KV_SEPERATOR.to_owned()));
        tokens.push(Token::Delimeter(" ".to_owned()));
        val_to_tokens_pretty(pair.1, tokens, indent + 1);
        tokens.push(Token::Symbol(deep::SEPERATOR.to_owned()));
        tokens.push(Token::Delimeter("\n".to_owned()));
    }
    tokens.push(Token::Delimeter("  ".repeat(indent)));
    tokens.push(Token::Symbol(deep::MAP_RIGHT.to_owned()))
}
