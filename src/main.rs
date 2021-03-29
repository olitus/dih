
// main.rc
// dih by Oleksandr Litus

use std::io;
use std::thread;
use std::time::Duration;
// use clap::App;
use std::process::Command;
use regex::Regex;
use colored::*;
// use inflector::Inflector;
// use rayon::prelude::*;
use indicatif::{ProgressBar, MultiProgress, ProgressStyle};

type Result<T> = std::result::Result<T, std::io::Error>;

// version=$($cmd -version 2>&1 |& grep -oP -m1 $version_regex | sed -n '1p')
fn version_with_keyword(cmd: &str, keyword: &str) -> Result<String> {
    let cmd_out = String::from_utf8(Command::new(cmd).arg(keyword).output()?.stdout).unwrap();
    let re = Regex::new(r"((?:\d+\.)+(?:\d+))(p\d+)?(?:.*?)").unwrap();

    let version = re.find(&cmd_out).ok_or(io::Error::new(io::ErrorKind::Other, "no match"))?;
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

// fn dih(cmd: &str) -> Entry {
//     match version(cmd) {
//         Some(ver) => Entry::i_have(cmd.into(), ver.as_str()),
//         None => Entry::i_have_not(cmd.into())
//     }   
// }

fn in_path(cmd: &str) -> bool {
    match Command::new("command").arg("-v").arg(cmd).output() {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(out) => match out.as_str() {
                "" => false,
                _ => true
            },
            _ => false
        },
        _ => false
    }
}

// fn name_of(cmd: &str) -> String {
//     match cmd {
//         "cpp" => "C++".into(),
//         _ => cmd.to_sentence_case()
//     }
// }

// struct Entry {
//     status: ColoredString,
//     name: ColoredString,
//     command: ColoredString,
//     version: ColoredString
// }

// impl Entry {
//     fn i_have(command: &str, version: &str) -> Entry{
//         Entry {
//             status: "✓".green(),
//             name: name_of(&command).cyan(),
//             command: command.blue(),
//             version: version.bright_magenta()
//         }
//     }
    
//     fn i_have_not(command: &str) -> Entry {
//         Entry {
//             status: "X".red(),
//             name: name_of(&command).dimmed(),
//             command: command.dimmed(),
//             version: "---".dimmed()
//         }
//     }

//     fn msg(self) -> String {
//         format!("{: <2} {: <10} {: <10} {: <10}", self.status, self.name, self.command, self.version)
//     }
// }

struct Language<'a> {
    name: &'a str,
    cmds: Vec<&'a str>
}

fn main() {

    let langs = vec![
        Language {
            name: "Rust",
            cmds: vec!["cargo", "rustc"]
        },
        Language {
            name: "Perl",
            cmds: vec!["perl"]
        },
        Language {
            name: "Ruby",
            cmds: vec!["ruby", "irb"]
        },
        Language {
            name: "Racket",
            cmds: vec!["racket", "raco", "drracket"]
        },
        Language {
            name: "Go",
            cmds: vec!["go"]
        },
        Language {
            name: "Zig",
            cmds: vec!["zig"]
        },
        Language {
            name: "Crystal",
            cmds: vec!["crystal", "shards"]
        },
    ];

    let spinner_style = ProgressStyle::default_spinner()
        .template("{spinner:.blue} {msg}");

    let m = MultiProgress::new();

    for lang in langs {
        let name = lang.name;
        for cmd in lang.cmds {
            let pb = m.add(ProgressBar::new_spinner());
            pb.enable_steady_tick(60);
            pb.set_style(spinner_style.clone());

            thread::spawn(move || {
                pb.set_message(format!("{: <2} {: <10} {: <10} {: <10}", "", name.cyan(), cmd.blue(), "").as_str());
                if in_path(cmd) {
                    pb.set_message(format!("{: <2} {: <10} {: <10} {: <10}", "✓".green(), name.cyan(), cmd.blue(), "").as_str());
                    // thread::sleep(Duration::from_millis(800));

                    if let Some(ver) = version(cmd) {
                        pb.finish_with_message(format!("{: <2} {: <10} {: <10} {: <10}", "✓".green(), name.cyan(), cmd.blue(), ver.magenta()).as_str());
                    } else {
                        pb.finish_with_message(format!("{: <2} {: <10} {: <10} {: <10}", "✓".green(), name.cyan(), cmd.blue(), "error".magenta()).as_str());
                    }
                } else {
                    pb.finish_with_message(format!("{: <2} {: <10} {: <10} {: <10}", "X".red(), name.dimmed(), cmd.dimmed(), "").as_str());
                }
            });
        }
    }

    m.join().unwrap();
}
