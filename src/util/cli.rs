use anyhow::{Context, Result};
use clap::Parser;
use std::{fs::read_dir, path::PathBuf, sync::{LazyLock, Mutex}};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to the dictionary storage directory. This directory should contain dictionary files used by the server.
    #[arg(short, long, value_name = "PATH")]
    dict_dir: Option<PathBuf>,

    /// Path to the site root directory. This directory serves as the root for static assets like HTML, CSS, and JavaScript.
    #[arg(short, long, value_name = "PATH")]
    site_root: Option<PathBuf>,
}

static ARGS: LazyLock<Cli> = LazyLock::new(|| Cli::parse());
pub static MDX_FILES: LazyLock<Mutex<Vec<PathBuf>>> = LazyLock::new(|| Mutex::new(get_dicts().unwrap()));

fn get_dicts() -> Result<Vec<PathBuf>> {
    let mut dicts = vec![];
    for entry in read_dir(get_dict_path()?).context("Failed to read dictionary directory")? {
        let path = entry?.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "mdx" {
                    dicts.push(path);
                }
            }
        }
    }
    if dicts.is_empty() {
        Err(anyhow::anyhow!("No dictionary files found"))
    } else {
        Ok(dicts)
    }
}
fn get_dict_path() -> Result<PathBuf> {
    let p = ARGS
        .dict_dir
        .as_ref()
        .filter(|i| i.exists() && i.is_dir())
        .cloned();
    if let Some(p) = p {
        Ok(p)
    } else {
        let p = PathBuf::from("resources/dict");
        if p.exists() && p.is_dir() {
            Ok(p)
        } else {
            Err(anyhow::anyhow!("dictionary directory not found"))
        }
    }
}

pub fn get_site_path() -> Result<PathBuf> {
    let p = ARGS
        .site_root
        .as_ref()
        .filter(|i| i.exists() && i.is_dir())
        .cloned();
    if let Some(p) = p {
        Ok(p)
    } else {
        let p = PathBuf::from("resources/site");
        if p.exists() && p.is_dir() {
            Ok(p)
        } else {
            Err(anyhow::anyhow!("site directory not found"))
        }
    }
}
