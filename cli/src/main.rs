use std::fs::File;
use std::io::Read;
use std::process::ExitCode;

use clipboard::{ClipboardContext, ClipboardProvider};

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let mut stdin = std::io::stdin();

    let mut input = String::new();
    if let Some(path) = args.next() {
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(e) => {
                println!("Error opening file {path}:\n{e}");
                return ExitCode::FAILURE;
            }
        };

        if let Err(e) = file.read_to_string(&mut input) {
            println!("Error reading file {path}:\n{e}");
            return ExitCode::FAILURE;
        }
    } else {
        if let Err(e) = stdin.read_to_string(&mut input) {
            println!("Error reading from stdin:\n{e}");
            return ExitCode::FAILURE;
        }
    }

    let output = mkdwn2forum::convert(&input);

    println!("{output}");

    match ClipboardContext::new() {
        Ok(mut clipboard) => {
            if let Err(e) = clipboard.set_contents(output) {
                println!("Error setting clipboard content:\n{e}");
            }
        }
        Err(e) => {
            println!("Error getting clipboard provider:\n{e}");
        }
    }

    // keep the program alive so the cliboard stays valid
    let mut line = String::new();
    let _ = stdin.read_line(&mut line);

    ExitCode::SUCCESS
}
