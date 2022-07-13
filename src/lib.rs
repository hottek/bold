use std::fs::File;
use std::io::{BufRead, BufReader, LineWriter, Write};
use std::path::PathBuf;

pub struct Bolder {}

impl Bolder {
    pub fn bolder(source: PathBuf, target: &mut PathBuf) {
        println!("{:?}:{:?}", &source, &target);
        Bolder::read_text(source);
    }

    fn read_text(source: PathBuf) {
        let mut file = match File::open(source) {
            Ok(file) => file,
            Err(why) => panic!("Unable to open {}", why),
        };
        let reader = BufReader::new(file);
        let mut line = String::new();
        for line in reader.lines() {
            println!("{:?}", line);
        };
    }
}
