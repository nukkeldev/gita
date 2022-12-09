use std::{
    env,
    path::Path,
    process::{exit, Command},
};

use colored::Colorize;
use gita::{
    error,
    logging::{self, log},
    success, warning,
};

fn main() {
    let mut args = env::args();

    let inited = Path::exists(Path::new("./.git/"));

    if !inited {
        error!("No git repo initialized! [use `git init`]");
        exit(0);
    }

    loop {
        let arg = args.next();
        match arg {
            Some(arg_str) => match arg_str.as_str() {
                "link" => {
                    link(args.next().unwrap_or_else(|| {
                        error!("No URL Provided");
                        exit(0)
                    }));
                }
                "pusha" => {
                    pusha(args.next().unwrap_or_else(|| {
                        error!("No Commit Message Provided");
                        exit(0)
                    }));
                }
                _ => {}
            },
            None => break,
        }
    }
}

fn link(path: String) {
    let mut args = ["remote", "add", "origin", path.as_str()];
    let mut completed = false;

    if git(&args) {
        success!(format!("Remote {} set to {}\n", "origin".purple(), path.yellow()).as_str());
        completed = true;
    }

    args[1] = "set-url";

    if git(&args) && !completed {
        success!(format!("Remote {} set to {}\n", "origin".purple(), path.yellow()).as_str());
    }

    git(&["config", "push.autoSetupRemote", "true"]);
    warning!("Upstream remote will setup on first push");
}

fn pusha(msg: String) {
    let args = [
        vec!["add", "."],
        vec!["commit", "-m", msg.as_str()],
        vec!["push"],
    ];
    log(""); // TODO: Log each step
}

fn git(args: &[&str]) -> bool {
    let output = Command::new("git").args(args).output().unwrap();

    println!("{:?}", output);

    output.status.success()
}

fn str_string_vec(input: Vec<&str>) -> Vec<String> {
    input.iter().map(|&f| f.to_owned()).collect::<Vec<String>>()
}
