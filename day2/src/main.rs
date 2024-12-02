use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::collections::VecDeque;

fn main() -> io::Result<()> {
    let _ = part1();
    let _ = part2();
    Ok(())
}

fn part2() -> io::Result<()> {
    let file = File::open("input.in")?;
    //let file = File::open("test.in")?;
    let mut reader = BufReader::new(file);
    let mut vec = VecDeque::new();

    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    let mut safecount = 0;


    while buffer.len() > 1 {
        for s in buffer.split_whitespace() {
            let n = match s.parse::<i32>() {
                Ok(n) => n,
                _ => break,
            };
            vec.push_back(n);
        };


        let mut prev = vec.pop_front().unwrap();
        let mut next = vec.pop_front().unwrap();

        let mut inc = false;
        let mut dec = false;
        let mut safe = true;
        let mut ig = false;

        loop {
            if !inc && !dec {
                if prev < next { inc = true } else { dec = true }
            };

            let mut dist = prev - next;
            dist = if dist < 0 { -dist } else { dist };

            if dist > 3 { safe = false };

            if inc && next < prev { safe = false };
            if dec && next > prev { safe = false };
            if prev == next { safe = false };

            if safe == false && ig == false { ig = true; safe = true }
            if safe == false { break };


            prev = next;
            next = match vec.pop_front() {
                Some(n) => n,
                _ => break,
            };
        }
        if safe { safecount += 1 };


        vec = VecDeque::new();
        buffer = String::new();
        reader.read_line(&mut buffer)?;
    }

    println!("{safecount}");

    Ok(())
}

fn part1() -> io::Result<()> {
    let file = File::open("input.in")?;
    //let file = File::open("test.in")?;
    let mut reader = BufReader::new(file);
    let mut vec = VecDeque::new();

    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    let mut safecount = 0;


    while buffer.len() > 1 {
        for s in buffer.split_whitespace() {
            let n = match s.parse::<i32>() {
                Ok(n) => n,
                _ => break,
            };
            vec.push_back(n);
        };


        let mut prev = vec.pop_front().unwrap();
        let mut next = vec.pop_front().unwrap();

        let mut inc = false;
        let mut dec = false;
        let mut safe = true;

        loop {
            if !inc && !dec {
                if prev < next { inc = true } else { dec = true }
            };

            let mut dist = prev - next;
            dist = if dist < 0 { -dist } else { dist };

            if dist > 3 { safe = false };

            if inc && next < prev { safe = false };
            if dec && next > prev { safe = false };
            if prev == next { safe = false };

            if safe == false { break };

            prev = next;
            next = match vec.pop_front() {
                Some(n) => n,
                _ => break,
            };
        }
        if safe { safecount += 1 };


        vec = VecDeque::new();
        buffer = String::new();
        reader.read_line(&mut buffer)?;
    }

    println!("{safecount}");
    Ok(())
}
