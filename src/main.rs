use std::{env, process};

enum ArgOption {
    Theme,
    Dir,
    Help,
}

impl ArgOption {
    fn value(&self) -> &'static str {
        match *self {
            ArgOption::Theme => { "--theme" }
            ArgOption::Dir => { "--dir" }
            ArgOption::Help => { "--help" }
        }
    }

    fn shorten_value(&self) -> &'static str {
        match *self {
            ArgOption::Theme => { "-t" }
            ArgOption::Dir => { "-d" }
            ArgOption::Help => { "-h" }
        }
    }

    fn is_theme(a: &String) -> bool {
        return a.as_str() == ArgOption::Theme.value() || a.as_str() == ArgOption::Theme.shorten_value();
    }

    fn is_dir(a: &String) -> bool {
        return a.as_str() == ArgOption::Dir.value() || a.as_str() == ArgOption::Dir.shorten_value();
    }

    fn is_help(a: &String) -> bool {
        return a.as_str() == ArgOption::Help.value() || a.as_str() == ArgOption::Help.shorten_value();
    }
}

fn parse_cli_args(args: &Vec<String>, theme: &mut String, dir_name: &mut String) {
    for i in 0..args.len() {
        if let Some(arg) = args.get(i) {
            if ArgOption::is_theme(arg) {
                *theme = args.get(i + 1)
                    .filter(|theme| { !(**theme).is_empty() && !(**theme).starts_with("-") })
                    .expect("Theme name expected")
                    .clone()
            } else if ArgOption::is_dir(arg) {
                *dir_name = args.get(i + 1)
                    .filter(|dn| { !(**dn).is_empty() && !(**dn).starts_with("-") })
                    .expect("Dir name expected")
                    .clone()
            } else if ArgOption::is_help(arg) {
                print_help();
                process::exit(0);
            } else {
                continue;
            }
        }
    }

    if theme.is_empty() || dir_name.is_empty() {
        panic!("Arguments --dir and --theme required");
    }
}

fn print_help() {
    todo!()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut theme = String::new();
    let mut dir_name = String::new();

    parse_cli_args(&args, &mut theme, &mut dir_name);
    println!("Chosen theme {} and dir name {}", theme, dir_name);
}
