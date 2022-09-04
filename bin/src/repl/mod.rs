use airlang;
use std::io::{self, Write};

pub fn repl() {
    print_version();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut src = String::new();
        io::stdin().read_line(&mut src).unwrap();
        let src = src.trim();

        if matches!(src, "quit" | "exit") {
            break;
        }

        let result = airlang::eval(&src);
        println!("{result}");
    }
}

fn print_version() {
    let version = include_str!("../air/Version.air");
    let version = airlang::eval(version);
    println!("🜁 air {version}");
}