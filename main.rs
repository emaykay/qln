use std::env;
use std::fs::{OpenOptions, create_dir_all};
use std::io::{Write, Error};
use chrono::Utc;
use whoami;

fn main() -> Result<(), Error> {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: qln notebook_name \"text to save\"");
        std::process::exit(1);
    }

    let notebook_name = &args[1];
    let text_to_save = &args[2];

    // Define the directory and create it if necessary
    let default_dir = dirs::document_dir().unwrap().join("qln");
    create_dir_all(&default_dir)?;

    // Define the notebook file path
    let notebook_path = default_dir.join(format!("{}.txt", notebook_name));

    // Open or create the notebook file for appending
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&notebook_path)?;

    // Write header if the file is newly created
    if file.metadata()?.len() == 0 {
        writeln!(file, "{}'s {} notebook\n", whoami::username(), notebook_name)?;
    }

    // Write the note with timestamp
    writeln!(file, "{}Z - {}", Utc::now().format("%Y-%m-%d %H:%M:%S"), text_to_save)?;

    // Print confirmation message
    println!("Your note has been saved to '{}'. Timestamp: {}Z", notebook_path.display(), Utc::now().format("%Y-%m-%d %H:%M:%S"));

    Ok(())
}
