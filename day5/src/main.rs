use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::Vec::Vec;

fn main() -> io::Result<()> {
    let _ = part1();
    // let _ = part2();
    Ok(())
}

fn part1() -> io::Result<()> {
    let file = File::open("input.in")?;
    //let file = File::open("test.in")?;
    let mut reader = BufReader::new(file);
    let mut vec = Vec::<String>::new();

    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;


    let len = buffer.len()-1;
    let mut total = 0;

    while buffer.len() > 2 {
        vec.push((&buffer[..len]).to_string());
        total += 1;


        buffer = String::new();
        reader.read_line(&mut buffer)?;
    }

    
    Ok(())
}


fn part2() -> io::Result<()> {
    Ok(());
}
