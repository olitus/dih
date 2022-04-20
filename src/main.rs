use colored::*;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use regex::Regex;
use std::io;
use std::process::Command;
use std::thread;

pub mod data;
pub mod language;
pub mod tool;

// dih --ext rb
// dih --pl

type Result<T> = std::result::Result<T, std::io::Error>;

fn version_with_keyword(cmd: &str, keyword: &str) -> Result<String> {
    let cmd_out = String::from_utf8(Command::new(cmd).arg(keyword).output()?.stdout).unwrap();
    let re = Regex::new(r"((?:\d+\.)+(?:\d+))(p\d+)?(?:.*?)").unwrap();

    let version = re
        .find(&cmd_out)
        .ok_or(io::Error::new(io::ErrorKind::Other, "no match"))?;
    Ok(version.as_str().into())
}

fn version(cmd: &str) -> Option<String> {
    if let Ok(version) = version_with_keyword(cmd, "--version") {
        Some(version)
    } else if let Ok(version) = version_with_keyword(cmd, "-version") {
        Some(version)
    } else if let Ok(version) = version_with_keyword(cmd, "version") {
        Some(version)
    } else if let Ok(version) = version_with_keyword(cmd, "--help") {
        Some(version)
    } else if let Ok(version) = version_with_keyword(cmd, "-help") {
        Some(version)
    } else if let Ok(version) = version_with_keyword(cmd, "help") {
        Some(version)
    } else {
        None
    }
}

fn in_path(cmd: &str) -> bool {
    match Command::new("command").arg("-v").arg(cmd).output() {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(out) => match out.as_str() {
                "" => false,
                _ => true,
            },
            _ => false,
        },
        _ => false,
    }
}


/// ...
fn main() {
    
}


fn old_main() {
    let spinner_style = ProgressStyle::default_spinner().template("{spinner:.blue} {msg}");

    let m = MultiProgress::new();

    for lang in data::LANGS {
        let name = lang.name;
        for cmd in lang.cmds {
            let pb = m.add(ProgressBar::new_spinner());
            pb.enable_steady_tick(60);
            pb.set_style(spinner_style.clone());

            thread::spawn(move || {
                pb.set_message(
                    format!(
                        "{: <2} {: <10} {: <10} {: <10}",
                        "",
                        name.cyan(),
                        cmd.blue(),
                        ""
                    )
                    .as_str(),
                );
                if in_path(cmd) {
                    pb.set_message(
                        format!(
                            "{: <2} {: <10} {: <10} {: <10}",
                            "✓".green(),
                            name.cyan(),
                            cmd.blue(),
                            ""
                        )
                        .as_str(),
                    );

                    if let Some(ver) = version(cmd) {
                        pb.finish_with_message(
                            format!(
                                "{: <2} {: <10} {: <10} {: <10}",
                                "✓".green(),
                                name.cyan(),
                                cmd.blue(),
                                ver.magenta()
                            )
                            .as_str(),
                        );
                    } else {
                        pb.finish_with_message(
                            format!(
                                "{: <2} {: <10} {: <10} {: <10}",
                                "✓".green(),
                                name.cyan(),
                                cmd.blue(),
                                "error".magenta()
                            )
                            .as_str(),
                        );
                    }
                } else {
                    pb.finish_with_message(
                        format!(
                            "{: <2} {: <10} {: <10} {: <10}",
                            "X".red(),
                            name.dimmed(),
                            cmd.dimmed(),
                            ""
                        )
                        .as_str(),
                    );
                }
            });
        }
    }

    m.join().unwrap();
}
