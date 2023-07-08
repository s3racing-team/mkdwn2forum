use std::fmt::Write as _;

pub fn convert(input: &str) -> String {
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
                push_text(&mut output, line[1..].trim());
                output.push('\n');
            }
            ' ' => {
                chars.next();
                let mut indent = 1;
                while let Some(' ') = chars.peek() {
                    chars.next();
                    indent += 1;
                }

                let Some('-') = chars.peek() else {
                    push_text(&mut output, line);
                    output.push('\n');
                    continue;
                };

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
                push_text(&mut output, line[text_start..].trim());
                output.push('\n');
            }
            _ => {
                push_text(&mut output, line);
                output.push('\n');
            }
        }
    }

    close_prev_lists(&mut output, &mut list_depth, 0);

    output
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

fn push_text(output: &mut String, line: &str) {
    let mut chars = line.char_indices().peekable();
    let mut pos = 0;
    'outer: loop {
        let Some((i, c)) = chars.next() else { break };

        match c {
            '[' => {
                let link_text_start = i + 1;
                let link_text_end = loop {
                    let Some((i, c)) = chars.next() else {
                        break 'outer;
                    };

                    if c == ']' {
                        break i;
                    }
                };

                // eat spaces
                while let Some((_, ' ')) = chars.peek() {
                    chars.next();
                }

                let link_url_start = match chars.peek() {
                    Some(&(i, '(')) => {
                        chars.next();
                        i + 1
                    }
                    Some(&(i, _)) => {
                        output.push_str(&line[pos..i]);
                        pos = i;
                        continue;
                    }
                    None => break,
                };

                let link_url_end = loop {
                    let Some((i, c)) = chars.next() else { break 'outer };

                    if c == ')' {
                        break i;
                    }
                };

                let link_text = line[link_text_start..link_text_end].trim();
                let link_url = line[link_url_start..link_url_end].trim();
                write!(output, "[url={link_url}]{link_text}[/url]").ok();

                pos = link_url_end + 1;
            }
            _ => {
                output.push(c);
                pos = i + 1;
            }
        }
    }

    output.push_str(&line[pos..]);
}