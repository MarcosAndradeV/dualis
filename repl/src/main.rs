use lex_just_parse::lexer::{Lexer, TokenKind};
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

// const DUALIS_HISTORY_PATH: &str = ".dualis_history";

fn main() -> rustyline::Result<()> {
    println!("Dualis REPL (use .q or .quit to exit)");
    let mut rl = DefaultEditor::new()?;
    // if rl.load_history(DUALIS_HISTORY_PATH).is_err() {}

    loop {
        let readline = rl.readline("dualis> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                match handle_repl_line(line) {
                    ReplAction::Continue => continue,
                    ReplAction::Break => break,
                }
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
    // rl.save_history(DUALIS_HISTORY_PATH)?;
    Ok(())
}

fn handle_repl_line(line: String) -> ReplAction {
    match line.trim() {
        ".q" | ".quit" => return ReplAction::Break,
        input => {
            let mut l = Lexer::new(input);
            match eval(&mut l) {
                Ok(Value::NoReturn) => return ReplAction::Break,
                Ok(value) => println!("{value}"),
                Err(err) => println!("{err}"),
            }
        }
    }
    ReplAction::Continue
}

enum ReplAction {
    Continue,
    Break,
}

fn eval<'lex>(l: &mut Lexer<'lex>) -> Result<Value, String> {
    loop {
        let t = l.next();
        if t.is_eof() {
            break;
        }
        match t.kind {
            TokenKind::Int(base) => {
                let n = i32::from_str_radix(t.source(), base.radix()).unwrap();
                return Ok(Value::Int32(n));
            }
            _ => todo!()
        }
    }
    Ok(Value::Nil)
}

#[derive(Debug)]
enum Value {
    Nil,
    #[allow(unused)]
    NoReturn,
    Int32(i32)
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::NoReturn => write!(f, "noreturn"),
            Value::Int32(n) => write!(f, "{n}"),
        }
    }
}
