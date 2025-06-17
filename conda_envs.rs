use std::fs;
use std::path::PathBuf;
use std::env;

fn main() {
    let home_dir = env::var("HOME").unwrap_or_default();

    let mut conda_dirs = vec![];

    // Check default miniconda/anaconda locations
    for prefix in &[".conda",] {
        let envs_path = PathBuf::from(&home_dir).join(prefix).join("envs");
        if envs_path.exists() {
            conda_dirs.push(envs_path);
        }
    }

    // Check CONDA_PREFIX if available
    if let Ok(conda_prefix) = env::var("CONDA_PREFIX") {
        let envs_path = PathBuf::from(conda_prefix).join("envs");
        if envs_path.exists() {
            conda_dirs.push(envs_path);
        }
    }

    let mut envs = vec![];

    for dir in conda_dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    if let Some(name) = entry.file_name().to_str() {
                        envs.push(name.to_string());
                    }
                }
            }
        }
    }

    // Always include "base"
    envs.push("base".to_string());

    envs.sort();
    envs.dedup();

    for env in envs {
        println!("{}", env);
    }
}

