use crate::parser::lexer::Token::{self, *};

pub fn expected() -> Vec<Token> {
    vec![
        Symbol("`".to_owned()),
        Symbol("~".to_owned()),
        Symbol("!".to_owned()),
        Symbol("@".to_owned()),
        Symbol("#".to_owned()),
        Symbol("$".to_owned()),
        Symbol("%".to_owned()),
        Symbol("^".to_owned()),
        Symbol("&".to_owned()),
        Symbol("*".to_owned()),
        Symbol("(".to_owned()),
        Symbol(")".to_owned()),
        Symbol("-".to_owned()),
        Symbol("+".to_owned()),
        Symbol("_".to_owned()),
        Symbol("=".to_owned()),
        Symbol("[".to_owned()),
        Symbol("]".to_owned()),
        Symbol("{".to_owned()),
        Symbol("}".to_owned()),
        Symbol("\\".to_owned()),
        Symbol("|".to_owned()),
        Symbol(";".to_owned()),
        Symbol(":".to_owned()),
        Bytes(vec![]),
        Symbol("\"".to_owned()),
        Symbol(",".to_owned()),
        Symbol(".".to_owned()),
        Symbol("<".to_owned()),
        Symbol(">".to_owned()),
        Symbol("/".to_owned()),
        Symbol("?".to_owned()),
        Symbol("'_".to_owned()),
        Symbol("'`".to_owned()),
        Symbol("'_a".to_owned()),
        Symbol("'`_".to_owned()),
        Symbol("'`_a1_B__2_".to_owned()),
        Symbol("'`__a".to_owned()),
        Symbol("'`~".to_owned()),
        Symbol("'`~_".to_owned()),
        Symbol("'`~_a".to_owned()),
        Symbol("'`~__a".to_owned()),
        Symbol("'__".to_owned()),
        Symbol("'___".to_owned()),
        Symbol("'_`".to_owned()),
        Symbol("'__`_".to_owned()),
        Symbol("'__`_a".to_owned()),
        Symbol("'_a".to_owned()),
        Symbol("'__a".to_owned()),
        Symbol("'___a".to_owned()),
    ]
}
