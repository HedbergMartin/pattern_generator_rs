
use std::fs::File;
use std::io::prelude::*;
use std::env;

mod pattern;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].parse::<i32>() {
            Ok(num) => make_patterns(num).unwrap(),
            Err(e) => print!("{}", e),
        }
    } else {
        make_patterns(20).unwrap();
    }

    if args.len() > 2 {
        match args[2].parse::<i32>() {
            Ok(num) => make_subjects(num).unwrap(),
            Err(e) => print!("{}", e),
        }
    } else {
        make_subjects(4).unwrap();
    }

}

fn make_patterns(patterns: i32) -> std::io::Result<()> {
    let mut file = File::create("patterns.txt")?;
    file.set_len(0)?;
    for _ in 0..patterns {
        let s = pattern::generate_pattern();
        file.write_all(s.as_bytes())?;
        file.write_all(b"\n")?;
    }
    Ok(())
}

fn make_subjects(subjects: i32) -> std::io::Result<()> {
    let mut file = File::create("subjects.txt")?;
    file.set_len(0)?;
    for _ in 0..subjects {
        let s = pattern::generate_subject();
        file.write_all(s.as_bytes())?;
        file.write_all(b"\n")?;
    }
    Ok(())
}


