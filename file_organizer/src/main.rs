use std::{collections::HashMap, fs, path::Path};

fn extensions() -> HashMap<&'static str, &'static str> {
    let mut ext_map = HashMap::new();
    ext_map.insert("jpg", "images");
    ext_map.insert("png", "images");
    ext_map.insert("pdf", "docs");
    ext_map.insert("txt", "docs");
    ext_map.insert("zip", "dart_bundles");
    ext_map.insert("pcapng", "packet_captures");
    ext_map.insert("pcap", "packet_captures");
    ext_map.insert("json", "json");
    ext_map
}

fn main() -> Result<(), Box<dyn std::error::Error>> { 
    let ext_map = extensions();
    let dir = Path::new("/Users/calupric/Downloads/");
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("no_ext").to_lowercase();

        if let Some(folder) = ext_map.get(ext.as_str()) {
            let dst_folder = dir.join(folder);
            if !dst_folder.exists() {
                fs::create_dir(&dst_folder)?;
            }

            if let Some(filename) = path.file_name() {
                let dst_path = dst_folder.join(filename);
                fs::rename(&path, &dst_path)?;
            }
        }
    }

    Ok(())
}
