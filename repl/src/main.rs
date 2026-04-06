use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

const DUALIS_HISTORY_PATH: &str = ".dualis_history";

fn main() -> rustyline::Result<()> {
    println!("Dualis REPL (use .q or .quit to exit)");
    let mut rl = DefaultEditor::new()?;
    if rl.load_history(DUALIS_HISTORY_PATH).is_err() {}

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
    rl.save_history(DUALIS_HISTORY_PATH)?;
    Ok(())
}

fn handle_repl_line(line: String) -> ReplAction {
    match line.trim() {
        ".q" | ".quit" => return ReplAction::Break,
        input => {
            println!("{input}");
        }
    }
    ReplAction::Continue
}

enum ReplAction {
    Continue,
    Break,
}
