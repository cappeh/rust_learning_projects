use std::{env, fs};

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();

    let dir_arg = &args[1];

    for entry in fs::read_dir(dir_arg)? {
        let dir = entry?;
        println!("{:?}", dir.path());
    }

    Ok(())
}
