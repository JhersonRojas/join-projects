use dialoguer::Input;
use ignore::WalkBuilder;
use std::collections::HashSet;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

fn main() -> io::Result<()> {

    let target_path: String = Input::new()
        .with_prompt("1. Target path files")
        .interact_text()
        .unwrap();

    let output_file: String = Input::new()
        .with_prompt("2. Name result")
        .default("complete_project.txt".into())
        .interact_text()
        .unwrap();

    let ignored_input: String = Input::new()
        .with_prompt("3. Files/folders will be ignored (optional)")
        .default("".into())
        .interact_text()
        .unwrap();

    let ignored_paths: HashSet<PathBuf> = ignored_input
        .split(',')
        .filter_map(|s| {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(PathBuf::from(trimmed))
            }
        })
        .collect();

    let exceptions_file = "exceptions.txt";

    let target_dir = Path::new(&target_path);
    if !target_dir.is_dir() {
        eprintln!("❌ '{}' is not a valid directory", target_path);
        std::process::exit(1);
    }

    let mut output = File::create(&output_file)?;
    let mut exceptions = OpenOptions::new()
        .create(true)
        .append(true)
        .open(exceptions_file)?;

    println!("📦 Compilyng files from: {}\n", target_path);

    let start = Instant::now();

    if !ignored_paths.is_empty() {
        println!("🚫 Ignoring manual paths:");
        for path in &ignored_paths {
            println!(" - {}", path.display());
        }
    }

    for result in WalkBuilder::new(target_dir)
        .standard_filters(true)
        .build()
    {
        let dir_entry = match result {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("Error to read input: {}", err);
                continue;
            }
        };

        let path = dir_entry.path();

        // Ignorar si el path contiene alguna ruta personalizada ignorada
        if ignored_paths.iter().any(|ignore| path.components().any(|c| c.as_os_str() == ignore.as_os_str())) {
            continue;
        }

        if path.is_file() {
            let path_str = path.to_string_lossy();

            match fs::read_to_string(path) {
                Ok(content) => {
                    writeln!(output, "\n/*\n* {}\n*/", path_str)?;
                    writeln!(output, "{}", content)?;
                }
                Err(_) => {
                    writeln!(exceptions, "\n/*\n* {}\n*/", path_str)?;
                    writeln!(exceptions, "Content UTF-8 was invalid")?;
                }
            }
        }
    }

    println!("✅ Files generated: {}", output_file);
    println!("⚠️ Catch exceptions in: {}", exceptions_file);

    let duration = start.elapsed();
    println!("⏱️ Total time: {:.2?}", duration);

    Ok(())
}
