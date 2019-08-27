use std::{env, fs, io, thread};
use std::fs::{DirEntry, FileType};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread::JoinHandle;
use std::time::Instant;

use colored::Colorize;
use git2::{DescribeFormatOptions, DescribeOptions, Repository, StatusOptions};

struct GitStatResult {
    repo_name: String,
    is_dirty: bool,
    branch: String,
    messages: String,
    desc: String,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let base_dir = match &args.get(1) {
        Some(v) => v,
        None => "."
    };

    println!("Base Dir: {}", base_dir);

    let dirs = get_dirs(base_dir);
    let (tx, rx): (Sender<GitStatResult>, Receiver<GitStatResult>) = mpsc::channel();
    let mut children = Vec::new();

    for e in dirs {
        let thread_tx = tx.clone();

        let child = thread::spawn(move || {
            let path_buf = e.path();
            explore_dir(path_buf, thread_tx);
        });

        children.push(child);
    }
    drop(tx);

    let printer = pass_recv_to_printer(rx);

    // Wait for the threads to complete any remaining work
    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    let _ = printer.join();

    Ok(())
}

fn pass_recv_to_printer(rx: Receiver<GitStatResult>) -> JoinHandle<()> {
    return thread::spawn(move || {
        while let Ok(r) = rx.recv() {
            if r.is_dirty {
                print!("{}", "**** ".red());
            }

            let branch = if r.branch != "refs/heads/master" { r.branch.as_str().cyan() } else { r.branch.as_str().green() };
            println!("Received from {} [{} @ {}]: {}", r.repo_name, branch, r.desc.as_str().bright_purple(), r.messages);
        }
    });
}

fn get_dirs(base_dir: &str) -> Vec<DirEntry> {
    let dirs = fs::read_dir(base_dir).unwrap()
        .filter_map(Result::ok)
        .filter({
            |e| -> bool {
                let file_type: FileType = e.file_type().unwrap();
                file_type.is_dir() || file_type.is_symlink()
            }
        });

    return dirs.collect();
}

fn explore_dir(dir: PathBuf, tx: Sender<GitStatResult>) {
    let dir_path = dir.to_str().unwrap();
    let git_path = format!("{}/.git", dir_path);
    if !Path::new(&git_path).exists() {
        drop(tx);
        return;
    }

    let repo = match Repository::open(dir_path) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open: {}", e)
    };

    let head = repo.head().unwrap();
    let name = head.name().unwrap();
    let mut messages = String::new();
    let mut is_dirty = true;
    if is_clean(&repo) {
        is_dirty = false;
    }

    let desc = desc(&repo);

    let result = GitStatResult {
        repo_name: dir_path.to_string(),
        is_dirty,
        branch: name.to_string(),
        messages,
        desc,
    };

    tx.send(result).unwrap();
    drop(tx);
}

fn desc(repo: &Repository) -> String {
    let mut descOpts = DescribeOptions::new();
    descOpts.describe_tags();
    let describe = match repo.describe(&descOpts) {
        Ok(describe) => describe,
        Err(e) => return "N/A".to_string(),
    };

    let mut descFmtOpts = DescribeFormatOptions::new();
    descFmtOpts.abbreviated_size(0);
    return describe.format(Some(&descFmtOpts)).unwrap();
}

fn is_clean(repo: &Repository) -> bool {
    let mut status_options: StatusOptions = StatusOptions::new();
    let statuses = repo.statuses(Some(&mut status_options)).unwrap();
    return statuses.is_empty();
}
