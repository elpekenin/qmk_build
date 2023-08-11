use std::{collections::hash_map::DefaultHasher, hash::{Hasher, Hash}, process::exit};

use crate::{logging::*, sh};

fn hash(repo: &String) -> String{
    let mut hasher = DefaultHasher::new();
    repo.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

pub fn clone(repo: &String, branch: &String, path: &String) {
    let remote = hash(repo);

    let _ = sh::run_strict(format!("git clone {repo} -b {branch} -o {remote} {path}"));
}

pub fn remote_add(repo: &String, path: &String) {
    let remote = hash(repo);
    
    let output = sh::run_at(path, format!("git remote add {remote} {repo}"));
    
    if output.status.code() != Some(0) {
        let stderr = String::from_utf8(output.stderr).unwrap();
        if !stderr.contains("already exists") { 
            error!("Adding remote failed with\n\t<red>{stderr}</>");
            exit(1);
        }
    }
}

pub fn fetch(repo: &String, path: &String) {
    let remote = hash(repo);
    let _ = sh::run_strict_at(path, format!("git fetch {remote}"));
}

pub fn checkout(repo: &String, branch: &String, files: Option<&Vec<String>>, path: &String) {
    let remote = hash(repo);

    let mut command = format!("git checkout {remote}/{branch}");
    if let Some(files) = files {
        command.push_str(" --");

        for file in files {
            command.push_str(&format!(" {file}"));
        }
    }

    let _ = sh::run_strict_at(path, command);
}

pub fn apply(file: &String, path: &String) {
    let _ = sh::run_strict_at(path, format!("git apply {file}"));
}

pub fn restore_staged(path: &String) {
    let _ = sh::run_strict_at(path, "git restore --staged .".to_owned());
}

pub fn restore(path: &String) {
    let _ = sh::run_strict_at(path, "git restore .".to_owned());
}

pub fn clean(path: &String) {
    let _ = sh::run_strict_at(path, "git clean -f -x".to_owned());
}