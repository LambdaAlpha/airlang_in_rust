#[cfg(test)]
mod test;

use regex::Regex;

use crate::parser::lexer::LexResult;

use super::super::LexError;
use super::super::Token;
use super::super::UnitLexer;

pub struct StringLexer {
    pattern: Regex,
    delimeter_pattern: Regex,
}

impl StringLexer {
    pub fn new() -> StringLexer {
        StringLexer {
            pattern: Regex::new(
                "(?x)
        \"
        (?P<body>(?:
            [^\"\\\\] # everything other than \" or \\
            |
            \\\\[\\ strn\\\\\"] # escape space, tab, newline and \"
            |
            \\\\[uU]\\{[0-9a-fA-F]{1,6}\\} # unicode
        )*)
        \"
        ",
            )
            .unwrap(),
            delimeter_pattern: Regex::new("[ \\t\\r\\n]*[\\t\\r\\n]+[ \\t\\r\\n]*").unwrap(),
        }
    }
}

impl UnitLexer for StringLexer {
    fn pattern(&self) -> &Regex {
        &self.pattern
    }
    fn lexing(&self, captures: &regex::Captures) -> LexResult<Token> {
        let m = captures.name("body").unwrap();
        let s = self.delimeter_pattern.replace_all(m.as_str(), "");
        let mut ns = String::new();
        let mut escape = false;
        let mut iter = s.chars().into_iter();
        while let Some(c) = iter.next() {
            if escape {
                let escaped = match c {
                    ' ' | 's' => ' ',
                    't' => '\t',
                    'r' => '\r',
                    'n' => '\n',
                    'u' | 'U' => {
                        let mut codepoint = String::new();
                        while let Some(hex) = iter.next() {
                            if hex == '{' {
                                continue;
                            }
                            if hex == '}' {
                                break;
                            }
                            codepoint.push(hex);
                        }
                        let i = u32::from_str_radix(codepoint.as_str(), 16).unwrap();
                        let i = char::from_u32(i);
                        if i.is_none() {
                            return LexError::err(format!("invalid unicode {codepoint}"));
                        }
                        i.unwrap()
                    }
                    _ => c,
                };
                ns.push(escaped);
                escape = false
            } else {
                if c == '\\' {
                    escape = true
                } else {
                    ns.push(c)
                }
            }
        }
        Ok(Token::String(ns))
    }
    fn stringify(&self, token: &Token, s: &mut String) {
        match token {
            Token::String(s1) => {
                s.push('"');
                for c in s1.chars() {
                    let escaped = match c {
                        '\\' => "\\\\".to_owned(),
                        '\n' => "\\n".to_owned(),
                        '\r' => "\\r".to_owned(),
                        '\t' => "\\t".to_owned(),
                        '"' => "\\\"".to_owned(),
                        _ => c.to_string(),
                    };
                    s.push_str(&escaped);
                }
                s.push('"');
            }
            _ => {}
        }
    }
}
