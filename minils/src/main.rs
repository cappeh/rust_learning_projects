use std::{env, fs, os::unix::fs::PermissionsExt};

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();

    let dir = &args[1];

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let meta = entry.metadata()?;

        println!("{:#o} {:?}", meta.permissions().mode(), entry.path());
    }

    Ok(())
}
