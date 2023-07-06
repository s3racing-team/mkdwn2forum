use clipboard::{ClipboardContext, ClipboardProvider};
use std::fs::File;
use std::io::Read;
use std::process::ExitCode;

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

    let mut output = String::new();
    let mut list_depth = 0;

    for line in input.lines() {
        let mut chars = line.chars().peekable();

        let Some(c) = chars.peek() else {
            close_prev_lists(&mut output, &mut list_depth, 0);
            output.push('\n');
            continue;
        };

        match c {
            '#' => {
                chars.next();
                close_prev_lists(&mut output, &mut list_depth, 0);

                let mut num_heading = 1;
                while let Some('#') = chars.peek() {
                    chars.next();
                    num_heading += 1;
                }

                match num_heading {
                    1 => output.push_str("[size=200]"),
                    2 => output.push_str("[size=150]"),
                    _ => output.push_str("[size]"),
                }
                output.push_str(line[num_heading..].trim());
                output.push_str("[/size]\n");
            }
            '-' => {
                chars.next();
                let in_list = list_depth > 0;
                close_prev_lists(&mut output, &mut list_depth, 1);

                if !in_list {
                    output.push_str("[list]\n");
                }

                output.push_str("[*]");
                output.push_str(line[1..].trim());
                output.push('\n');
            }
            ' ' => {
                chars.next();
                let mut indent = 1;
                while let Some(' ') = chars.peek() {
                    chars.next();
                    indent += 1;
                }

                match chars.peek() {
                    Some('-') => {
                        if indent % 4 != 0 {
                            // TODO: warning
                        }
                        let calc_depth = (indent / 4) + 1;
                        let new_list_depth = if calc_depth > list_depth + 1 {
                            // TODO: warning
                            list_depth + 1
                        } else {
                            calc_depth
                        };

                        let start_new_list = new_list_depth > list_depth;
                        close_prev_lists(&mut output, &mut list_depth, new_list_depth);

                        if start_new_list {
                            for _ in 0..(new_list_depth - 1) {
                                output.push_str("    ");
                            }
                            output.push_str("[list]\n");
                        }

                        for _ in 0..(new_list_depth - 1) {
                            output.push_str("    ");
                        }
                        output.push_str("[*]");
                        let text_start = indent as usize + 1;
                        output.push_str(line[text_start..].trim());
                        output.push('\n');
                    }
                    _ => {
                        output.push_str(line);
                        output.push('\n');
                        continue;
                    }
                }
            }
            _ => {
                output.push_str(line);
                output.push('\n');
            }
        }
    }

    close_prev_lists(&mut output, &mut list_depth, 0);

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

fn close_prev_lists(output: &mut String, list_depth: &mut u16, new_list_depth: u16) {
    for i in (new_list_depth..*list_depth).rev() {
        for _ in 0..i {
            output.push_str("    ");
        }
        output.push_str("[/list]\n");
    }
    *list_depth = new_list_depth;
}
