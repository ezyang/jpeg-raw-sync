use clap::Parser;
use std::path::{Path, PathBuf};
use std::option::Option;
use anyhow::{Result, bail, anyhow};
use std::ffi::{OsStr,OsString};
use std::collections::HashSet;
use std::fs;
use std::io::stdin;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    jpg_subdir: PathBuf,
}

fn match_stem(path: &Path, ext: &str) -> Option<OsString> {
    if path.is_dir() { return None; }
    if !path.extension().map_or(false, |s| s.to_ascii_lowercase() == OsStr::new(ext)) { return None; }
    return Some(OsString::from(path.file_stem()?));
}

fn main() -> Result<()> {
    let args = Args::parse();
    let jpg_subdir = args.jpg_subdir;

    let subdir = jpg_subdir.file_name()
        .ok_or(anyhow!("passed argument has no file name"))?;
    let jpg_dir = jpg_subdir.parent()
        .ok_or(anyhow!("jpg subdirectory doesn't have parent"))?;

    if jpg_dir.file_name() != Some(OsStr::new("Output")) {
        bail!("passed subdirectory not in folder named Output");
    }
    let parent_dir = jpg_dir.parent()
        .ok_or(anyhow!("Output directory doesn't have parent"))?;

    let raw_dir = parent_dir.join("Raw");
    let raw_subdir = raw_dir.join(subdir);

    let trash_dir = parent_dir.join("Trash");
    let trash_subdir = trash_dir.join(subdir);

    eprintln!("Synchronizing {} to {}", jpg_subdir.display(), raw_subdir.display());

    let mut to_keep = HashSet::new();
    for entry in fs::read_dir(jpg_subdir)? {
        let path = entry?.path();
        eprintln!("Scanning {:?}", path);
        if let Some(stem) = match_stem(&path, "jpg") {
            to_keep.insert(stem);
        };
    }

    eprintln!("Keeping {:?}", to_keep);

    fs::create_dir_all(&trash_subdir)?;

    for entry in fs::read_dir(raw_subdir)? {
        let path = entry?.path();
        if let Some(stem) = match_stem(&path, "arw") {
            if !to_keep.contains(&stem) {
                eprintln!("Moving to trash {:?}", path);
                let filename = path.file_name().expect("impossible!, has stem but not path");
                let trash_path = trash_subdir.join(filename);
                fs::rename(&path, trash_path)?;
            }
        };
    }

    println!("Press any key to continue");
    stdin().read_line(&mut String::new()).unwrap();

    Ok(())
}
