extern crate chrono;
use chrono::{DateTime, Utc};
use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub mod colors;

fn info(text: &str, settings: &mut Vec<String>) {
    if settings[1] == "$NORMAL!" {
        print!(
            "{}${}[ {}{}{} ]{} {}{}",
            colors::RED,
            colors::GREEN,
            colors::CYAN,
            settings[0],
            colors::GREEN,
            colors::BLUE,
            text,
            colors::RESET
        );
    } else if settings[1] == "$ABSTRACT!" {
        print!(
            "{} ≣≣≣[{} {}{} ]≣≣≣{} {}{}{}",
            colors::GREEN,
            colors::CYAN,
            settings[0],
            colors::GREEN,
            colors::RED,
            colors::CYAN,
            text,
            colors::RESET
        );
    }
}

fn parse_settings(settings_array: &mut Vec<String>) -> &mut Vec<String> {
    match settings_array[0].as_str() {
        "$DEFAULT!" => {
            settings_array[0] = "nitroTools".to_string();
        }

        "$PATH!" => {
            let current_dir = match env::current_dir() {
                Ok(path) => format!("{}", path.display()),
                Err(e) => format!("Error getting current directory: {}\n", e),
            };
            settings_array[0] = current_dir;
        }

        &_ => {}
    }
    settings_array
}

fn error(text: &str, settings: &mut Vec<String>) {
    if settings[1] == "$NORMAL!" {
        print!(
            "{}${}[ {}{}{} ]{} {}{}",
            colors::RED,
            colors::GREEN,
            colors::CYAN,
            settings[0],
            colors::GREEN,
            colors::RED,
            text,
            colors::RESET
        );
    } else if settings[1] == "$ABSTRACT!" {
        print!(
            "{} ≣≣≣[{} {}{} ]≣≣≣{} {}{}{}",
            colors::GREEN,
            colors::CYAN,
            settings[0],
            colors::GREEN,
            colors::RED,
            colors::RED,
            text,
            colors::RESET
        );
    }
}

fn format_system_time(system_time: SystemTime) -> String {
    let datetime: DateTime<Utc> = DateTime::from(system_time);
    datetime.format("%d-%m-%Y %H:%M").to_string()
}

fn main() -> io::Result<()> {
    let mut line_number_to_modify;
    let mut file = fs::File::open("src/settings.ninfo")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();
    let mut _settings = parse_settings(&mut lines);

    let mut running = true;

    while running {
        let mut user_input = String::new();
        if _settings[1] == "$NORMAL!" {
            print!(
                "{}${}[ {}{}{} ] >>{} ",
                colors::RED,
                colors::GREEN,
                colors::CYAN,
                _settings[0],
                colors::GREEN,
                colors::RESET
            );
        } else if _settings[1] == "$ABSTRACT!" {
            print!(
                "{}↱{}≣≣≣[{} {}{} ]≣≣≣ \n{}↳ {}",
                colors::RED,
                colors::GREEN,
                colors::CYAN,
                _settings[0],
                colors::GREEN,
                colors::RED,
                colors::RESET
            );
        }
        io::stdout().flush()?;
        io::stdin().read_line(&mut user_input)?;
        let user_input = user_input.trim();

        let words: Vec<&str> = user_input.split(' ').collect();
        let formatted_message: String;

        match words.as_slice() {
            ["exit"] => {
                running = false;
                info("Exiting", &mut _settings);
            }
            ["echo", ..] => {
                let echo_sentence = user_input.strip_prefix("echo ");
                let casted_echo_sentence = echo_sentence.unwrap_or(&"").to_string();
                info(&(casted_echo_sentence + "\n"), &mut _settings);
            }
            ["dir"] => {
                let formatted_message = match env::current_dir() {
                    Ok(path) => format!("{}\n", path.display()),
                    Err(e) => format!("Error getting current directory: {}\n", e),
                };
                info(&formatted_message, &mut _settings);
            }
            ["set", "model", model_type] => {
                if model_type == &"path" {
                    line_number_to_modify = 0;
                    let mut lines: Vec<&str> = contents.lines().collect();
                    if line_number_to_modify < lines.len() {
                        lines[line_number_to_modify] = "$PATH!";
                        let current_dir = match env::current_dir() {
                            Ok(path) => path.display().to_string(),
                            Err(e) => format!("Error getting current directory: {}\n", e),
                        };
                        _settings[0] = current_dir;
                    } else {
                        eprintln!("Line number {} is out of range.", line_number_to_modify);
                    }
                    let modified_contents = lines.join("\n");
                    let mut file = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open("src/settings.ninfo")?;
                    file.write_all(modified_contents.as_bytes())?;
                } else if model_type == &"default" {
                    line_number_to_modify = 0;
                    let mut lines: Vec<&str> = contents.lines().collect();
                    if line_number_to_modify < lines.len() {
                        lines[line_number_to_modify] = "$DEFAULT!";
                        _settings[0] = "nitroTools".to_string();
                    } else {
                        eprintln!("Line number {} is out of range.", line_number_to_modify);
                    }
                    let modified_contents = lines.join("\n");
                    let mut file = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open("src/settings.ninfo")?;
                    file.write_all(modified_contents.as_bytes())?;
                } else {
                    let error_text = format!("Model '{}' does not exist!\n", model_type);
                    error(&error_text, &mut _settings);
                }
            }
            ["set", "aspect", aspect_type] => {
                if aspect_type == &"normal" {
                    line_number_to_modify = 1;
                    let mut lines: Vec<&str> = contents.lines().collect();
                    if line_number_to_modify < lines.len() {
                        lines[line_number_to_modify] = "$NORMAL!";
                        _settings[1] = "$NORMAL!".to_string();
                    } else {
                        eprintln!("Line number {} is out of range.", line_number_to_modify);
                    }
                    let modified_contents = lines.join("\n");
                    let mut file = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open("src/settings.ninfo")?;
                    file.write_all(modified_contents.as_bytes())?;
                    info("Aspect set to 'normal'\n", &mut _settings);
                } else if aspect_type == &"abstract" {
                    line_number_to_modify = 1;
                    let mut lines: Vec<&str> = contents.lines().collect();
                    if line_number_to_modify < lines.len() {
                        lines[line_number_to_modify] = "$ABSTRACT!";
                        _settings[1] = "$ABSTRACT!".to_string();
                    } else {
                        eprintln!("Line number {} is out of range.", line_number_to_modify);
                    }
                    let modified_contents = lines.join("\n");
                    let mut file = OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open("src/settings.ninfo")?;
                    file.write_all(modified_contents.as_bytes())?;
                    info("Aspect set to 'abstract'\n", &mut _settings);
                } else {
                    let error_text = format!("Aspect '{}' does not exist!\n", aspect_type);
                    error(&error_text, &mut _settings);
                }
            }
            ["show", show_type] => {
                if show_type == &"dir" {
                    let mut files: Vec<PathBuf> = Vec::new();
                    let mut directories: Vec<PathBuf> = Vec::new();

                    let paths = fs::read_dir("./")?;

                    for path in paths {
                        let path = path?;
                        let metadata = fs::metadata(&path.path())?;
                        if metadata.is_file() {
                            files.push(path.path());
                        } else if metadata.is_dir() {
                            directories.push(path.path());
                        }
                    }

                    files.sort();
                    directories.sort();

                    // Calculate maximum lengths for formatting
                    let max_dir_length = directories
                        .iter()
                        .map(|d| d.display().to_string().len())
                        .max()
                        .unwrap_or(0);

                    let max_file_length = files
                        .iter()
                        .map(|f| f.display().to_string().len())
                        .max()
                        .unwrap_or(0);

                    let max_size_length = files
                        .iter()
                        .map(|f| {
                            let size_kb = round_to_two_decimal_places(
                                fs::metadata(f).unwrap().len() as f64 / 1024.0,
                            );
                            format!("{:.2} KB", size_kb).len()
                        })
                        .max()
                        .unwrap_or(0);

                    info("Directories: \n", _settings); // Placeholder for settings
                    for dir in directories.iter() {
                        let size_bytes = calculate_dir_size(dir)?;
                        let mut size_kb = round_to_two_decimal_places(size_bytes as f64 / 1024.0);
                        let time_created = match fs::metadata(dir)?.modified() {
                            Ok(created) => format_system_time(created),
                            Err(_) => "Created time not available".to_string(),
                        };
                        let size_type;
                        if size_kb > 999.99 {
                            size_type = "MB";
                            size_kb = size_kb / 1000.0;
                        }
                        else {
                            size_type = "KB";
                        }
                        println!(
                            "                            {:<dir_width$} | {:<size_width$} | {}",
                            dir.display(),
                            format!("{:.2} {}", size_kb, size_type),
                            time_created,
                            dir_width = max_dir_length,
                            size_width = max_size_length
                        );
                    }

                    info("Files: \n", _settings); // Placeholder for settings
                    for file in files.iter() {
                        let metadata = fs::metadata(file)?;
                        let mut size_kb = round_to_two_decimal_places(metadata.len() as f64 / 1024.0);
                        let time_created = match metadata.modified() {
                            Ok(created) => format_system_time(created),
                            Err(_) => "Created time not available".to_string(),
                        };
                        let size_type;
                        if size_kb > 999.99 {
                            size_type = "MB";
                            size_kb = size_kb / 1000.0;
                        }
                        else {
                            size_type = "KB";
                        }
                        println!(
                            "                            {:<file_width$} | {:<size_width$} | {}",
                            file.display(),
                            format!("{:.2} {}", size_kb, size_type),
                            time_created,
                            file_width = max_file_length,
                            size_width = max_size_length
                        );
                    }
                }
            }
            [_command, ..] => {
                formatted_message = format!("'{}' is not a command!\n", user_input);
                error(&formatted_message, &mut _settings);
            }
            [] => {}
        }
    }
    Ok(())
}

fn calculate_dir_size(path: &Path) -> io::Result<u64> {
    let mut total_size = 0;

    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let entry_path = entry.path();
        let metadata = fs::metadata(&entry_path)?;

        if metadata.is_file() {
            total_size += metadata.len();
        } else if metadata.is_dir() {
            total_size += calculate_dir_size(&entry_path)?; // Recursive call
        }
    }

    Ok(total_size)
}

fn round_to_two_decimal_places(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}
