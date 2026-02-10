use std::{env, fs};

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();

    let dir = &args[1];

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let meta = entry.metadata()?;

        println!("{:?} {:?}", meta.permissions(), entry.path());
    }

    Ok(())
}
