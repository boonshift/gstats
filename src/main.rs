use std::{env, fs, io, thread};
use std::fs::{DirEntry, FileType};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread::JoinHandle;

use colored::Colorize;
use colored::control::set_override;
use git2::{DescribeFormatOptions, DescribeOptions, Repository, StatusOptions};

struct GitStatResult {
    repo_name: String,
    is_dirty: bool,
    in_sync: bool,
    branch: String,
    messages: String,
    desc: String,
}

struct ChildProcessingThread {
    dir_name: String,
    child: JoinHandle<()>,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Make sure it colors regardless of console or pipe etc.
    set_override(true);

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
        let dir_name = e.path().to_str().unwrap().to_string();

        let child = thread::spawn(move || {
            let path_buf = e.path();
            explore_dir(path_buf, thread_tx);
        });

        children.push(ChildProcessingThread { dir_name, child });
    }
    drop(tx);

    let printer = pass_recv_to_printer(rx);

    // Wait for the threads to complete any remaining work
    for child in children {
        child.child.join().expect(format!("oops! the child[{}] thread panicked", child.dir_name).as_str());
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
            if !r.in_sync {
                print!("{}", "?!?! ".bright_magenta());
            }

            let branch = if r.branch != "refs/heads/master".to_string() { r.branch.as_str().cyan() } else { r.branch.as_str().green() };
            println!("Stats of {} [{} @ {}]: {}", r.repo_name, branch, r.desc.as_str().bright_purple(), r.messages);
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

    let head = match repo.head() {
        Ok(head) => head,
        Err(_e) => {
            drop(tx);
            return;
        }
    };

    let name = head.name().unwrap().replace("refs/heads/", "");
    let mut messages = String::new();
    let mut is_dirty = true;
    if is_clean(&repo) {
        is_dirty = false;
    }

    let in_sync = is_sync(&repo, &name, &mut messages);

    let desc = describe(&repo);

    let result = GitStatResult {
        repo_name: dir_path.to_string(),
        is_dirty,
        in_sync,
        branch: name.to_string(),
        messages,
        desc,
    };

    tx.send(result).unwrap();
    drop(tx);
}

fn describe(repo: &Repository) -> String {
    let mut desc_opts = DescribeOptions::new();
    desc_opts.describe_tags();
    let describe = match repo.describe(&desc_opts) {
        Ok(describe) => describe,
        Err(_) => return "N/A".to_string(),
    };

    let mut desc_fmt_opts = DescribeFormatOptions::new();
    desc_fmt_opts.abbreviated_size(0);
    return describe.format(Some(&desc_fmt_opts)).unwrap();
}

fn is_clean(repo: &Repository) -> bool {
    let mut status_options: StatusOptions = StatusOptions::new();
    let statuses = repo.statuses(Some(&mut status_options)).unwrap();
    return statuses.is_empty();
}

fn is_sync(repo: &Repository, branch_name: &String, messages: &mut String) -> bool {
    let origin_oid = match repo.revparse(format!("origin/{}", branch_name).as_str()) {
        Ok(r) => r.from().unwrap().id(),
        Err(_) => {
            messages.push_str(format!("origin/{} not found", branch_name).as_str());
            return false;
        }
    };

    let cur_oid = match repo.revparse(branch_name) {
        Ok(r) => r.from().unwrap().id(),
        Err(_) => {
            messages.push_str(format!("{} not found", branch_name).as_str());
            return false;
        }
    };

    return origin_oid.eq(&cur_oid);
}
