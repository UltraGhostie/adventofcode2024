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

fn part1() -> io::Result<()> {
    let file = File::open("input.in")?;
    //let file = File::open("test.in")?;
    let mut reader = BufReader::new(file);
    let mut vec = VecDeque::<String>::new();

    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;


    let len = buffer.len()-1;
    let mut total = 0;

    while buffer.len() > 2 {
        vec.push_back((&buffer[..len]).to_string());
        total += 1;


        buffer = String::new();
        reader.read_line(&mut buffer)?;
    }

    let mut appearances = 0;
    for i in 0..total {
        // Attempt to construct string from chars at position i+a, j+b where i+a > 0 and j+b > 0
        for j in 0..len {
            // Check left to right
            if xmascheck(&vec, &i, &j, 0, 1) { appearances += 1 };
            // Check to down right
            if xmascheck(&vec, &i, &j, 1, 1) { appearances += 1 };
            // Check up to down
            if xmascheck(&vec, &i, &j, 1, 0) { appearances += 1 };
            // Check to down left
            if xmascheck(&vec, &i, &j, 1, -1) { appearances += 1 };
            // Check right to left
            if xmascheck(&vec, &i, &j, 0, -1) { appearances += 1 };
            // Check to up left
            if xmascheck(&vec, &i, &j, -1, -1) { appearances += 1 };
            // Check down to up
            if xmascheck(&vec, &i, &j, -1, 0) { appearances += 1 };
            // Check to up right
            if xmascheck(&vec, &i, &j, -1, 1) { appearances += 1 };
        }
    }
    println!("{appearances}");
    Ok(())
}

fn xmascheck(vec: &VecDeque<String>, i: &usize, j: &usize, a: isize , b: isize) -> bool {
    let i:isize = isize::try_from(i.clone()).unwrap();
    let j:isize = isize::try_from(j.clone()).unwrap();
    let ix = i+a*3;
    let jx = j+b*3;
    //println!("{ix} {jx}"); 
    if jx < 0 || ix < 0 { return false };
    let i1:usize = match (i+a*0).try_into() {
        Ok(n) => n,
        Err(s) => { println!("{i}+{a}*0"); panic!("{s}") }
    };
    let j1:usize = match (j+b*0).try_into() {
        Ok(n) => n,
        Err(s) => { println!("{j}+{b}*0"); panic!("{s}") }
    };
    let i2:usize = match (i+a*1).try_into() {
        Ok(n) => n,
        Err(s) => { println!("{i}+{a}*1"); panic!("{s}") }
    };

    let j2:usize = match (j+b*1).try_into() {
        Ok(n) => n,
        Err(s) => { println!("{j}+{b}*1"); panic!("{s}") }
    };
    let i3:usize = match (i+a*2).try_into() {
        Ok(n) => n,
        Err(s) => { println!("{i}+{a}*2"); panic!("{s}") }
    };
    let j3:usize = match (j+b*2).try_into() {
        Ok(n) => n,
        Err(s) => { println!("{j}+{b}*2"); panic!("{s}") }
    };
    let i4:usize = match (i+a*3).try_into() {
        Ok(n) => n,
        Err(s) => { println!("{i}+{a}*3"); panic!("{s}") }
    };
    let j4:usize = match (j+b*3).try_into() {
        Ok(n) => n,
        Err(s) => { println!("{j}+{b}*3"); panic!("{s}") }
    };
    let x = match vec.get(i1) {
        Some(s) => match s.chars().nth(j1) {
            Some(c) => c,
            None => return false,
        },
        None => return false,
    };
    if x != 'X' { return false; }
    let x = match vec.get(i2) {
        Some(s) => match s.chars().nth(j2) {
            Some(c) => c,
            None => return false,
        },
        None => return false,
    };
    if x != 'M' { return false; }
    let x = match vec.get(i3) {
        Some(s) => match s.chars().nth(j3) {
            Some(c) => c,
            None => return false,
        },
        None => return false,
    };
    if x != 'A' { return false; }
    let x = match vec.get(i4) {
        Some(s) => match s.chars().nth(j4) {
            Some(c) => c,
            None => return false,
        },
        None => return false,
    };
    if x != 'S' { return false; }
    true
}


fn part2() -> io::Result<()> {
    //let file = File::open("input2.in")?;
    let file = File::open("test2.in")?;
    let mut reader = BufReader::new(file);
    let mut vec = VecDeque::<String>::new();

    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;


    let len = buffer.len()-1;
    let mut total = 0;

    while buffer.len() > 2 {
        vec.push_back((&buffer[..len]).to_string());
        total += 1;

        buffer = String::new();
        reader.read_line(&mut buffer)?;
    }

    let mut appearances = 0;
    for i in 0..total {
        println!();
        for j in 0..len {
            if x_mascheck(&vec, &i, &j) { appearances += 1 };
        }
    }
    println!("{appearances}");
    Ok(())
}

fn x_mascheck(vec: &VecDeque<String>, i: &usize, j: &usize) -> bool {
    let i:isize = isize::try_from(i.clone()).unwrap();
    let j:isize = isize::try_from(j.clone()).unwrap();

    if j < 1 || i < 1 { return false };

    let i0:usize = match (i).try_into() {
        Ok(n) => n,
        Err(s) => { println!("{i}+0"); panic!("{s}") }
    };
    let j0:usize = match (i).try_into() {
        Ok(n) => n,
        Err(s) => { println!("{i}+0"); panic!("{s}") }
    };

    let i1:usize = match (i+1).try_into() {
        Ok(n) => n,
        Err(s) => { println!("{i}+1"); panic!("{s}") }
    };
    let j1:usize = match (i+1).try_into() {
        Ok(n) => n,
        Err(s) => { println!("{i}+1"); panic!("{s}") }
    };

    let i2:usize = match (i-1).try_into() {
        Ok(n) => n,
        Err(s) => { println!("{i}-1"); panic!("{s}") }
    };
    let j2:usize = match (i-1).try_into() {
        Ok(n) => n,
        Err(s) => { println!("{i}-1"); panic!("{s}") }
    };

    // Middle is always A
    let x = match vec.get(i0) {
        Some(s) => match s.chars().nth(j0) {
            Some(c) => c,
            None => return false,
        },
        None => return false,
    };
    println!("{i}, {j}; {i0}, {j0}: {x}");
    if x != 'A' { return false; }



    // Check top right/bot left
    let x = match vec.get(i2) {
        Some(s) => match s.chars().nth(j1) {
            Some(c) => c,
            None => return false,
        },
        None => return false,
    };
    let y = match vec.get(i1) {
        Some(s) => match s.chars().nth(j2) {
            Some(c) => c,
            None => return false,
        },
        None => return false,
    };
    let mas = (x == 'M' && y == 'S') || (y == 'M' && x == 'S');
    if !mas { return false; }


    // Check top left/bot right
    let x = match vec.get(i2) {
        Some(s) => match s.chars().nth(j2) {
            Some(c) => c,
            None => return false,
        },
        None => return false,
    };
    let y = match vec.get(i1) {
        Some(s) => match s.chars().nth(j1) {
            Some(c) => c,
            None => return false,
        },
        None => return false,
    };
    let mas = (x == 'M' && y == 'S') || (y == 'M' && x == 'S');
    if !mas { return false; }


    true
}
