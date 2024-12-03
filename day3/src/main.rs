use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use regex::Regex;

fn main() -> io::Result<()> {
    let _ = part1();
    Ok(())
}

fn part2() -> io::Result<()> {
    Ok(())
}

fn part1() -> io::Result<()> {
    let file = File::open("input.in")?;
    //let file = File::open("test.in")?;
    let mut reader = BufReader::new(file);
    let wrap = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;


    let mut total = 0;
    while buffer.len() > 1 {
        let groupiter = wrap.captures_iter(&buffer);

        for s in groupiter {
            let a = &s[1].parse::<i32>().unwrap();
            let b = &s[2].parse::<i32>().unwrap();
            total += a * b;
            println!("{a} {b}");
        }


        buffer = String::new();
        reader.read_line(&mut buffer)?;
    }
    println!("{total}");

    Ok(())
}
