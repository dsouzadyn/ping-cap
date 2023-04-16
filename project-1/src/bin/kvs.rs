#![allow(unused)]
use clap::{Parser, Subcommand};
use std::process::exit;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct KvsCli {
    #[command(subcommand)]
    command: Option<KvsCommand>,
}

#[derive(Subcommand, Debug)]
enum KvsCommand {
    /// get the value of a key
    Get { key: String },
    /// set the value of a key
    Set { key: String, value: String },
    /// remove the value of a key
    Rm { key: String },
}

fn main() {
    let args = KvsCli::parse();
    match &args.command {
        Some(KvsCommand::Get { key }) => {
            eprintln!("unimplemented");
            exit(1);
        }
        Some(KvsCommand::Set { key, value }) => {
            eprintln!("unimplemented");
            exit(1);
        }
        Some(KvsCommand::Rm { key }) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => {
            unreachable!();
        }
    }
}
