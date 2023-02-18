use std::fs::DirEntry;
use std::path::Path;
use std::{env, fs, io, process};

enum ArgOption {
    Theme,
    Dir,
    Help,
}

impl ArgOption {
    fn value(&self) -> &'static str {
        match *self {
            ArgOption::Theme => "--theme",
            ArgOption::Dir => "--dir",
            ArgOption::Help => "--help",
        }
    }

    fn shorten_value(&self) -> &'static str {
        match *self {
            ArgOption::Theme => "-t",
            ArgOption::Dir => "-d",
            ArgOption::Help => "-h",
        }
    }

    fn is_theme(a: &String) -> bool {
        return a.as_str() == ArgOption::Theme.value()
            || a.as_str() == ArgOption::Theme.shorten_value();
    }

    fn is_dir(a: &String) -> bool {
        return a.as_str() == ArgOption::Dir.value()
            || a.as_str() == ArgOption::Dir.shorten_value();
    }

    fn is_help(a: &String) -> bool {
        return a.as_str() == ArgOption::Help.value()
            || a.as_str() == ArgOption::Help.shorten_value();
    }
}

fn parse_cli_args(args: &Vec<String>, theme: &mut String, dir_name: &mut String) {
    for i in 0..args.len() {
        if let Some(arg) = args.get(i) {
            if ArgOption::is_theme(arg) {
                *theme = args
                    .get(i + 1)
                    .filter(|theme| !(**theme).is_empty() && !(**theme).starts_with("-"))
                    .expect("Theme name expected")
                    .clone()
            } else if ArgOption::is_dir(arg) {
                *dir_name = args
                    .get(i + 1)
                    .filter(|dn| !(**dn).is_empty() && !(**dn).starts_with("-"))
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

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut theme = String::new();
    let mut dir_name = String::new();

    parse_cli_args(&args, &mut theme, &mut dir_name);
    println!("Chosen theme {} and dir name {}", theme, dir_name);

    let path = Path::new(&dir_name);
    visit_dirs(path, &|entry| {
        let file_path = entry.path();
        let os_string = entry.file_name();
        if None == os_string.to_str() {
            return;
        }
        let file_name = os_string.to_str().unwrap();
        let format = format!(".{}", theme);
        if !file_name.contains(format.as_str()) {
            return;
        }
        let dest_file_name = file_name.replace(format.as_str(), "");
        if None == file_path.parent() {
            return;
        }
        let parent_path = file_path.parent().unwrap();
        let destination = parent_path.join(dest_file_name);
        fs::copy(file_path.clone(), destination.clone()).expect(
            format!(
                "Failed to copy {} to {}",
                file_path.to_str().unwrap_or("some file"),
                destination.to_str().unwrap_or("some file")
            )
            .as_str(),
        );
    })
    .expect("Something sent wrong while processing");

    println!("Successful!");
}
