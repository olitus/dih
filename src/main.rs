
// main.rc
// dih by Oleksandr Litus

use std::io;
use clap::App;
use std::process::Command;
use regex::Regex;
use colored::*;
use inflector::Inflector;
use std::thread;

fn cli() -> App<'static, 'static> {
    App::new("dih")
}

// version=$($cmd -version 2>&1 |& grep -oP -m1 $version_regex | sed -n '1p')
fn version_with_keyword(cmd: &str, keyword: &str) -> io::Result<String> {
    let cmd_out = String::from_utf8(Command::new(cmd).arg(keyword).output()?.stdout).unwrap();
    let re = Regex::new(r"((?:\d+\.)+(?:\d+))(p\d+)?(?:.*?)").unwrap();
    let version = re.find(&cmd_out).unwrap();
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

fn dih(cmd: &str) -> Entry {
    match version(cmd) {
        Some(ver) => Entry::i_have(cmd.into(), ver),
        None => Entry::i_have_not(cmd.into())
    }   
}

fn name_of(cmd: &str) -> String {
    match cmd {
        "cpp" => "C++".into(),
        _ => cmd.to_sentence_case()
    }
}

struct Entry {
    status: ColoredString,
    name: ColoredString,
    command: ColoredString,
    version: ColoredString
}

impl Entry {
    fn i_have(command: String, version: String) -> Entry{
        Entry {
            status: "✓".green(),
            name: name_of(&command).green(),
            command: command.green(),
            version: version.green()
        }
    }
    
    fn i_have_not(command: String) -> Entry {
        Entry {
            status: "X".red(),
            name: name_of(&command).dimmed(),
            command: command.dimmed(),
            version: "---".dimmed()
        }
    }

    fn show(self) {
        println!("{: <2} {: <10} {: <10} {: <10}", self.status, self.name, self.command, self.version);
    }
}

fn main() {

    let commands = vec![
        "cargo",
        "go",
        "racket"
    ];

    let handle = thread::spawn(|| {
        for cmd in commands {
            dih(cmd).show();
        }
    });

    handle.join().unwrap();
}
