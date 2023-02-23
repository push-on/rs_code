use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

// Helper function to read user input
fn get_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}

// Helper function to read the names file into a vector
fn read_names_file(file_path: &str) -> io::Result<Vec<String>> {
    let file = fs::File::open(file_path)?;
    let lines = io::BufReader::new(file).lines();
    let mut names = Vec::new();
    for line in lines {
        let name = line?;
        names.push(name);
    }
    Ok(names)
}

fn main() -> io::Result<()> {
    // Take input for the location of the folder
    let folder_path = get_input("Enter folder path: ")?;

    // Take input for the text file with all the names
    let names_file = get_input("Enter names file path: ")?;

    // Read the contents of the names file into a vector
    let names = read_names_file(&names_file)?;

    // Check if the number of names matches the number of files in the folder
    let folder_files = fs::read_dir(&folder_path)?.count();
    if names.len() != folder_files {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Number of names do not match number of files in folder"));
    }

    // Preview the renamed files
    let files = fs::read_dir(&folder_path)?;
    let mut conflicts = false;
    for (i, file) in files.enumerate() {
        let file = file?.path();
        let ext = file.extension().unwrap_or_default().to_str().unwrap_or("");
        let new_name = format!("{:02}_{}.{}", i+1, names[i], ext);
        println!("{} -> {}", file.display(), new_name);
        if Path::new(&folder_path).join(&new_name).exists() {
            conflicts = true;
        }
    }

    // Show conflicts, if any
    if conflicts {
        let answer = get_input("There are conflicts. Do you want to proceed anyway? (y/n): ")?;
        if answer.trim().to_lowercase() != "y" {
            return Ok(());
        }
    }

    // Rename the files
    let files = fs::read_dir(&folder_path)?;
    for (i, file) in files.enumerate() {
        let file = file?.path();
        let ext = file.extension().unwrap_or_default().to_str().unwrap_or("");
        let new_name = format!("{:02}_{}.{}", i+1, names[i], ext);
        let new_path = Path::new(&folder_path).join(&new_name);
        fs::rename(&file, &new_path)?;
    }

    println!("Files renamed successfully!");
    open::that(&folder_path)?;
    Ok(())
}


