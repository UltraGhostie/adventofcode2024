use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::collections::BinaryHeap;
use std::collections::HashMap;

fn main() {
    let _ = part1();
    let _ = part2();
    Ok(())
}

fn part2() {
    let file = File::open("input.in")?;
    //let file = File::open("test.in")?;
    let mut reader = BufReader::new(file);
    let mut existshmap = HashMap::<i32, bool>::new();
    let mut counthmap = HashMap::<i32, i32>::new();

    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    //Seperate the two numbers, parse into i32s, add to existsmap the first number, increment the
    //number in counthmap of the second number
    while buffer.len() > 1 {
        let sep = buffer.find(' ');
        let sep = match sep {
            Some(n) => n,
            _ => panic!("None: {buffer}"),
        };

        let num1 = buffer[..sep].parse::<i32>().unwrap();
        existshmap.insert(num1, true);

        let num2 = buffer[sep+1..&buffer.len()-1].parse::<i32>().unwrap();
        let cnt = match counthmap.get(&num2) {
            Some(n) => n,
            None => &0,
        };
        counthmap.insert(num2, cnt+1);


        buffer = String::new();
        reader.read_line(&mut buffer)?;
    }


    // Calculate similarity score
    let mut sim = 0;

    for (k, _) in existshmap.drain() {
        let a = match counthmap.get(&k) {
            Some(n) => n,
            None => &0,
        };
        if a != &0 { println!("{k} / {a}"); }
        sim = sim + (k*a)
    }

    println!("\n\n{sim}");


    Ok(())
}

fn part1() {
    let file = File::open("input.in")?;
    //let file = File::open("test.in")?;
    let mut reader = BufReader::new(file);
    let mut bheap1 = BinaryHeap::<i32>::new();
    let mut bheap2 = BinaryHeap::<i32>::new();

    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    //Seperate the two numbers, parse into i32s, add to bheaps to sort
    while buffer.len() > 1 {
        let sep = buffer.find(' ');
        let sep = match sep {
            Some(n) => n,
            _ => panic!("None: {buffer}"),
        };
        let num1 = buffer[..sep].parse::<i32>().unwrap();
        bheap1.push(num1);

        let num2 = buffer[sep+1..&buffer.len()-1].parse::<i32>().unwrap();
        bheap2.push(num2);


        buffer = String::new();
        reader.read_line(&mut buffer)?;
    }

    // Calculate total distance by popping the largest items, calculating the distance between them
    // and then summing to tot
    let mut tot = 0;

    loop {
        let n1 = bheap1.pop();
        let n2 = bheap2.pop();
        let n1 = match n1 {
            Some(n) => n,
            _ => break,
        };
        let n2 = match n2 {
            Some(n) => n,
            _ => panic!("??"),
        };
        let dist = n1 - n2;
        let dist = if dist < 0 { -dist } else { dist };
        tot = tot + dist;
    }

    println!("{tot}");

    Ok(())
}
