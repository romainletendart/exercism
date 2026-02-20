use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use anyhow::Error;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
#[derive(Debug, Default)]
pub struct Flags {
    line_number: bool,
    files_with_matches: bool,
    ignore_case: bool,
    invert_match: bool,
    line_match: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut instance = Self {
            ..Default::default()
        };
        for &flag in flags {
            match flag {
                "-n" => instance.line_number = true,
                "-l" => instance.files_with_matches = true,
                "-i" => instance.ignore_case = true,
                "-v" => instance.invert_match = true,
                "-x" => instance.line_match = true,
                &_ => panic!("Unsupported flag {flag}"),
            }
        }
        instance
    }
}

fn format_result(
    file_path: &str,
    line_number: usize,
    line: &str,
    flags: &Flags,
    multiple_files: bool,
) -> String {
    if flags.files_with_matches {
        return file_path.to_string();
    }
    let file_path_prefix = {
        if multiple_files {
            format!("{file_path}:")
        } else {
            String::new()
        }
    };
    let line_number_infix = {
        if flags.line_number {
            format!("{line_number}:")
        } else {
            String::new()
        }
    };
    format!("{file_path_prefix}{line_number_infix}{line}")
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let multiple_files = files.len() > 1;
    let mut results = Vec::new();
    let pattern = {
        if flags.ignore_case {
            pattern.to_lowercase()
        } else {
            pattern.to_string()
        }
    };
    for &file_path in files {
        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);
        let mut buf = String::new();
        let mut line_number = 1usize;
        while let Ok(length) = reader.read_line(&mut buf)
            && length != 0
        {
            let line = buf.trim_end().to_string();
            let candidate = {
                if flags.ignore_case {
                    line.to_lowercase()
                } else {
                    line.to_string()
                }
            };
            let is_match = {
                if flags.line_match {
                    candidate == pattern
                } else {
                    candidate.contains(&pattern)
                }
            };
            if flags.invert_match ^ is_match {
                results.push(format_result(
                    file_path,
                    line_number,
                    &line,
                    flags,
                    multiple_files,
                ));
                if flags.files_with_matches {
                    // We already matched this file path, no need to check remaining lines.
                    break;
                }
            }
            buf.clear();
            line_number += 1;
        }
    }
    Ok(results)
}
