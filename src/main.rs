use clap::Parser;
use std::path::{Path, PathBuf};
use std::option::Option;
use anyhow::{Result, bail, anyhow};
use std::ffi::{OsStr,OsString};
use std::collections::HashSet;
use std::fs;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    jpeg_subdir: PathBuf,
}

fn match_stem(path: &Path, ext: &str) -> Option<OsString> {
    if path.is_dir() { return None; }
    if path.extension() != Some(OsStr::new(ext)) { return None; }
    return Some(OsString::from(path.file_stem()?));
}

fn main() -> Result<()> {
    let args = Args::parse();
    let jpeg_subdir = args.jpeg_subdir;

    let subdir = jpeg_subdir.file_name()
        .ok_or(anyhow!("passed argument has no file name"))?;
    let jpeg_dir = jpeg_subdir.parent()
        .ok_or(anyhow!("jpeg subdirectory doesn't have parent"))?;

    if jpeg_dir.file_name() != Some(OsStr::new("Output")) {
        bail!("passed subdirectory not in folder named Output");
    }
    let parent_dir = jpeg_dir.parent()
        .ok_or(anyhow!("Output directory doesn't have parent"))?;

    let raw_dir = parent_dir.join("Raw");
    let raw_subdir = raw_dir.join(subdir);

    let trash_dir = parent_dir.join("Trash");
    let trash_subdir = trash_dir.join(subdir);

    eprintln!("Synchronizing {} to {}", jpeg_subdir.display(), raw_subdir.display());

    let mut to_keep = HashSet::new();
    for entry in fs::read_dir(jpeg_subdir)? {
        let path = entry?.path();
        if let Some(stem) = match_stem(&path, "jpeg") {
            to_keep.insert(stem);
        };
    }

    // eprintln!("Keeping {:?}", to_keep);

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

    Ok(())
}
