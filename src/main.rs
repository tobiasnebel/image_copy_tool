use std::{fs, path::Path};

use walkdir::WalkDir;

const USAGE: &str = "usage: imgtool[.exe] <source_root_folder> <target_folder>";

fn main() {
    // let source_root_folder = Path::new("C:\\tmp\\img\\in");
    // let target_folder = Path::new("C:\\tmp\\img\\out");

    let args: Vec<String> = std::env::args().collect();
    let sut = (args.get(1), args.get(2));
    let args_res = match sut {
        (Some(source), Some(target)) => Ok((source, target)),
        _ => Err("invalid arguments"),
    }
    .map(|sut| (Path::new(sut.0), Path::new(sut.1)));

    match args_res {
        Ok((source_root_folder, target_folder)) => {
            if let Err(e) = rename_jpegs_in_folder(source_root_folder, target_folder) {
                eprintln!("Error: {}", e);
            }
        }
        Err(e) => {
            println!("Error: {e}");
            print_usage();
        }
    }
}

fn print_usage() {
    println!("{USAGE}");
}

fn rename_jpegs_in_folder(source_root_folder: &Path, target_folder: &Path) -> std::io::Result<()> {
    let mut highest_index = 0;
    let mut to_rename = Vec::new();

    // Traverse the directory recursively
    for entry in WalkDir::new(source_root_folder)
        .max_depth(3)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();

        // Skip if it's a directory
        if path.is_dir() {
            continue;
        }

        // Get the file name as a string
        if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
            if file_name.to_lowercase().ends_with(".jpeg")
                || file_name.to_lowercase().ends_with(".jpg")
                || file_name.to_lowercase().ends_with(".png")
                || file_name.to_lowercase().ends_with(".bmp")
                || file_name.to_lowercase().ends_with(".gif")
            {
                // Queue files that match any supported image extension (not yet renamed)
                println!(" > found file '{file_name}'");
                to_rename.push(path.to_path_buf());
            }
        }
    }
    println!("Found {} files to rename and move", to_rename.len());

    // Ensure "out" directory exists
    let out_dir = target_folder;
    if !out_dir.exists() {
        fs::create_dir(out_dir)?;
    }

    // Rename and move files to the "out" folder
    for path in to_rename {
        highest_index += 1;
        let ext = path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        let new_name = format!("img_{}.{}", highest_index, ext);
        let new_path = out_dir.join(new_name);

        fs::rename(&path, &new_path)?;
        println!("  > Renamed and moved {:?} to {:?}", path, new_path);
    }

    Ok(())
}
