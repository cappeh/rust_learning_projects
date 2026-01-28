use std::{fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let dir = Path::new("/Users/calupric/Downloads/");
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        let ext = match path.extension().and_then(|e| e.to_str()) {
            Some(ext) => ext.to_lowercase(),
            None => "no_extension".to_string(),
        };

        let dest_dir = dir.join(&ext);

        if !dest_dir.exists() {
            fs::create_dir(&dest_dir)?;
        }

        let file_name = path.file_name().unwrap();
        let dest_path = dest_dir.join(file_name);

        fs::rename(&path, &dest_path)?;

    }

    Ok(())
}
