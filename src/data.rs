use crate::language::Language;
use crate::tool::Tool;

pub const LANGS: Vec<Language> = vec![
    Language {
        name: "Rust".into(),
        tools: vec![
            Tool {
                name: "Cargo".into(),
                cmd: "cargo".into(),
                typ: "Package Manager".into(),
                default_version_option: "version".into(),
            },
            Tool {
                name: "Rust Compiler".into(),
                cmd: "rustc".into(),
                typ: "Compiler".into(),
                default_version_option: "--version".into(),
            },
        ],
    },
    Language {
        name: "Perl".into(),
        cmds: vec!["perl"],
    },
    Language {
        name: "Ruby".into(),
        cmds: vec!["ruby", "irb"],
    },
    Language {
        name: "Racket".into(),
        cmds: vec!["racket", "raco", "drracket"],
    },
    Language {
        name: "Go".into(),
        cmds: vec!["go"],
    },
    Language {
        name: "Zig".into(),
        cmds: vec!["zig"],
    },
    Language {
        name: "Crystal".into(),
        cmds: vec!["crystal", "shards"],
    },
    Language {
        name: "Python".into(),
        cmds: vec!["python3", "pip3", "python", "pip"],
    },
    Language {
        name: "JavaScript".into(),
        cmds: vec!["npm", "deno"],
    },
];
