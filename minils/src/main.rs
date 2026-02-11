use std::{env, fs, os::unix::fs::PermissionsExt};
use std::fs::FileType;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {

    let args: Vec<String> = env::args().collect();

    let cmd = parse_args(&args);

    for entry in fs::read_dir(cmd.directory)? {
        let entry = entry?;
        let meta = entry.metadata()?;

        let mode = meta.permissions().mode();
        let ft = meta.file_type();

        println!(
            "{}{}  {}",
            filetype(ft),
            mode_to_rwx(mode),
            entry.file_name().to_string_lossy()
        );

    }

    Ok(())
}

struct Cmd {
    directory: PathBuf,
}

fn parse_args(args: &[String]) -> Cmd {
   let directory = args.get(1)
       .map(PathBuf::from)
       .unwrap_or_else(|| PathBuf::from("."));

    Cmd { directory }
}

fn filetype(ft: FileType) -> char {
    if ft.is_dir() { 'd' } else { '-' }
}

fn mode_to_rwx(mode: u32) -> String {
    let mut perm = String::new();

    // User
    perm.push(if mode & modes::USER_READ != 0 { 'r' } else { '-' } );
    perm.push(if mode & modes::USER_WRITE != 0 { 'w' } else { '-' } );
    perm.push(if mode & modes::USER_EXECUTE != 0 { 'x' } else { '-' } );

    // Group
    perm.push(if mode & modes::GROUP_READ != 0 { 'r' } else { '-' } );
    perm.push(if mode & modes::GROUP_WRITE != 0 { 'w' } else { '-' } );
    perm.push(if mode & modes::GROUP_EXECUTE != 0 { 'x' } else { '-' } );

    // Other
    perm.push(if mode & modes::OTHER_READ != 0 { 'r' } else { '-' } );
    perm.push(if mode & modes::OTHER_WRITE != 0 { 'w' } else { '-' } );
    perm.push(if mode & modes::OTHER_EXECUTE != 0 { 'x' } else { '-' } );

    perm
}
mod modes {
    pub type Mode = u32;

    pub const USER_READ: Mode = libc::S_IRUSR as Mode;
    pub const USER_WRITE: Mode = libc::S_IWUSR as Mode;
    pub const USER_EXECUTE: Mode = libc::S_IXUSR as Mode;

    pub const GROUP_READ: Mode = libc::S_IRGRP as Mode;
    pub const GROUP_WRITE: Mode = libc::S_IWGRP as Mode;
    pub const GROUP_EXECUTE: Mode = libc::S_IXGRP as Mode;

    pub const OTHER_READ: Mode = libc::S_IROTH as Mode;
    pub const OTHER_WRITE: Mode = libc::S_IWOTH as Mode;
    pub const OTHER_EXECUTE: Mode = libc::S_IXOTH as Mode;

    //pub const STICKY: Mode = libc::S_ISVTX as Mode;
    //pub const SETGID: Mode = libc::S_ISGID as Mode;
    //pub const SETUID: Mode = libc::S_ISUID as Mode;
}
