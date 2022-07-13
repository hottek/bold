use clap::{App, Arg};
use std::env;
use std::path::PathBuf;
use bold::Bolder;

fn main() {
    let matches = App::new("Bold")
        .args(&vec![
            Arg::with_name("source")
                .short("s")
                .long("source")
                .help("source file")
                .takes_value(true),
                Arg::with_name("target")
                .short("t")
                .long("target")
                .help("target file")
                .takes_value(true),
        ])
        .get_matches();

    let source_dir = if let Some(source_dir) = matches.value_of("source") {
        PathBuf::from(source_dir)
    } else {
        env::current_dir().unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        })
    };

    let mut target_dir = if let Some(target_dir) = matches.value_of("target") {
        PathBuf::from(target_dir)
    } else {
        env::current_dir().unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        })
    };

    Bolder::bolder(source_dir, &mut target_dir);
    std::process::exit(0);
}