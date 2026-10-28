mod quotes;
mod random;
mod session;

use std::env;

const CYAN_BRIGHT: &str = "\x1b[1;96m";
const RESET: &str = "\x1b[0m";

pub fn format_quote(text: &str, max_width: usize) -> String {
    let paragraphs = text.split('\n');
    let mut formatted_lines = Vec::new();

    for p in paragraphs {
        let words: Vec<&str> = p.split_whitespace().collect();
        if words.is_empty() {
            continue;
        }

        let mut cur_line = Vec::new();
        let mut cur_len = 0;

        for w in words {
            let space = if cur_line.is_empty() { 0 } else { 1 };
            if cur_len + w.len() + space > max_width && !cur_line.is_empty() {
                formatted_lines.push(cur_line.join(" "));
                cur_line = vec![w];
                cur_len = w.len();
            } else {
                cur_line.push(w);
                cur_len += w.len() + space;
            }
        }

        if !cur_line.is_empty() {
            formatted_lines.push(cur_line.join(" "));
        }
    }

    if formatted_lines.is_empty() {
        return String::new();
    }

    if formatted_lines.len() == 1 {
        return format!("      \"{}\"", formatted_lines[0]);
    }

    let mut res = Vec::new();
    res.push(format!("      \"{}", formatted_lines[0]));
    for line in &formatted_lines[1..formatted_lines.len() - 1] {
        res.push(format!("       {}", line));
    }
    res.push(format!("       {}\"", formatted_lines.last().unwrap()));
    res.join("\n")
}

fn print_help() {
    println!("nanofetch - A minimal terminal fetch & daily quote tool\n");
    println!("USAGE:");
    println!("    nanofetch [OPTIONS]\n");
    println!("OPTIONS:");
    println!("    -r, --refresh    Refresh quote for today");
    println!("    -h, --help       Print help information");
    println!("    -V, --version    Print version information");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut force_refresh = false;

    if args.len() > 1 {
        match args[1].as_str() {
            "-h" | "--help" => {
                print_help();
                return;
            }
            "-V" | "--version" => {
                println!("nanofetch {}", env!("CARGO_PKG_VERSION"));
                return;
            }
            "-r" | "--refresh" => {
                force_refresh = true;
            }
            other => {
                eprintln!("Unknown option: {}", other);
                eprintln!("Run 'nanofetch --help' for usage.");
                std::process::exit(1);
            }
        }
    }

    let user = env::var("USER").unwrap_or_else(|_| "(unknown)".to_string());
    let quote = session::get_or_select_daily_quote(force_refresh);
    let quote_formatted = format_quote(&quote, 65);

    println!();
    println!("{}  user  {}{}", CYAN_BRIGHT, RESET, user);
    println!();
    println!("{}", quote_formatted);
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_single_line_quote() {
        let q = "Simplicity is prerequisite for reliability.";
        let formatted = format_quote(q, 65);
        assert_eq!(
            formatted,
            "      \"Simplicity is prerequisite for reliability.\""
        );
    }

    #[test]
    fn test_format_multiline_quote() {
        let q = "You do not rise to the level of your goals.\nYou fall to the level of your systems.";
        let formatted = format_quote(q, 65);
        let expected = "      \"You do not rise to the level of your goals.\n       You fall to the level of your systems.\"";
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_format_wrapping() {
        let q = "This is a very long line that should certainly exceed the maximum width specified for line wrapping in our test case.";
        let formatted = format_quote(q, 30);
        let lines: Vec<&str> = formatted.lines().collect();
        assert!(lines.len() > 1);
        assert!(lines[0].starts_with("      \""));
        assert!(lines.last().unwrap().ends_with('"'));
    }
}
