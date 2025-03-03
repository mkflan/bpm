#![allow(unused)]

mod cli;
pub mod crypto;
mod db;

use clap::Parser;
use cli::{Cli, Command, handle_create, handle_open};

fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Create(args) => handle_create(args),
        Command::Open(args) => handle_open(args),
        _ => todo!(),
    }
}
