use std::collections::HashSet;
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

use dialoguer::{Input, Select};
use ignore::WalkBuilder;

mod utils;

fn main() -> io::Result<()> {
    // Capturing the current dir of user
    let current_dir = env::current_dir()?;

    println!(
        "\nCurrent directory:\n{icon} {dir:?}\n",
        icon = utils::icons::FOLDER_OPEN,
        dir = current_dir
    );

    // Creating a folders list for options to user
    let mut folders: Vec<String> = Vec::new();

    for entry in fs::read_dir(&current_dir)? {
        let entry = entry?;
        let path: PathBuf = entry.path();

        // Set each dir in current path to list
        if path.is_dir() {
            if let Some(folder) = path.file_name().and_then(|n| n.to_str()) {
                folders.push(folder.to_string());
            }
        }
    }

    // Copy original folders with icon for best UI
    let playable_folders: Vec<String> = folders
        .iter()
        .map(|f| format!("{} {}", utils::icons::FOLDER, f))
        .collect();

    let selection = Select::new()
        .with_prompt("1. Folders aviables")
        .items(&playable_folders)
        .interact()
        .unwrap();

    let target_path: String = folders[selection].to_owned();

    let output_file: String = Input::new()
        .with_prompt("2. Name result")
        .default("content.txt".into())
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

    let target_dir = Path::new(&target_path);
    if !target_dir.is_dir() {
        eprintln!("❌'{}' is not a valid directory", target_path);
        std::process::exit(1);
    }

    println!("\n📦 Compilyng files from: {}\n", &target_path);

    // Creating necesary folders and files
    let output_dir = PathBuf::from("results");

    fs::create_dir_all(&output_dir)?;

    let output_path = output_dir.join(&output_file);
    let exceptions_path = output_dir.join("exceptions.txt");

    let mut output = File::create(&output_path)?;
    let mut exceptions = File::create(&exceptions_path)?;

    // Elapsed time counter
    let start = Instant::now();

    if !ignored_paths.is_empty() {
        println!("🚫 Ignoring manual paths:");
        for path in &ignored_paths {
            println!(" - {}", path.display());
        }
    }

    // Here get the collection tree entries (folder/files) to join
    let tree_path_collection = WalkBuilder::new(target_dir).standard_filters(true).build();

    for result in tree_path_collection {
        let dir_entry = match result {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("❌Error to read input: {}", err);
                continue;
            }
        };

        let path = dir_entry.path();

        if ignored_paths.iter().any(|ignore| {
            path.components()
                .any(|c| c.as_os_str() == ignore.as_os_str())
        }) {
            continue;
        }

        // Write in files about correct or error capture content
        if path.is_file() {
            let path_str = path.to_string_lossy();

            match fs::read_to_string(path) {
                Ok(content) => {
                    writeln!(output, "/*\n* {}\n*/\n", path_str)?;
                    writeln!(output, "{}", content)?;
                }
                Err(_) => {
                    writeln!(exceptions, "/*\n* {}\n*/\n", path_str)?;
                    writeln!(exceptions, "Content UTF-8 was invalid")?;
                }
            }
        }
    }

    println!("✅ Files generated: {:?}", &output_path);
    println!("⚠️ Catch exceptions in: {:?}", &exceptions_path);

    let duration = start.elapsed();
    println!("⏱️ Total time: {:.2?}", duration);

    let _: String = Input::new()
        .with_prompt("\n🚪For quit, press Enter")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    Ok(())
}
