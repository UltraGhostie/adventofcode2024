use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::vec::Vec;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let _ = part1();
    // let _ = part2();
    Ok(())
}

fn part1() -> io::Result<()> {
    //let file = File::open("input.in")?;
    let file = File::open("test.in")?;
    let mut reader = BufReader::new(file);
    let mut order = HashMap::<i32, Vec::<i32>>::new();


    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;


    while buffer.len() > 2 {
        let sepi = match buffer.find('|') {
            Some(n) => n,
            None => panic!("AAA"),
        };
        let key = *(&buffer[..sepi].parse::<i32>().unwrap());
        let rule = *(&buffer[sepi+1..buffer.len()-1].parse::<i32>().unwrap());
        println!("{key}, {rule}");

        let mut val = match order.get(&key) {
            Some(s) => s.clone(),
            None => Vec::<i32>::new(),
        };
        
        val.push(rule);
        
        order.insert(key, val);



        buffer = String::new();
        reader.read_line(&mut buffer)?;
    }

    let mut total = 0;

    while buffer.len() > 0 {
        let mut used = HashMap::<i32, bool>::new();
        let mut vec = Vec::<i32>::new();
        let mut valid = false;

        for si in buffer.to_string().split_terminator(',') {
            let s = si.trim();
            if si == "" { continue; }
            print!("{s} ");
            let val = match s.parse::<i32>() {
                Ok(n) => n,
                //Err(_) => panic!("'{s}' is not a number"),
                Err(_) => continue,
            };

            valid = true;
            match order.get(&val) {
                Some(v) => for i in v {
                    if used.get(&i) != None {
                        valid = false;
                        break;
                    };
                },
                None => print!("a"),
            };

            if !valid { 
                println!("{val} is used");
                break; 
            }
            vec.push(val);
        }
        if valid { 
            let mid = (vec.len() + 1) / 2;
            let len = vec.len();
            println!("{len} {mid}");
            let midval = match vec.get(mid-1) {
                Some(n) => n,
                None => panic!("'{mid}' has no entry"),
            };
            println!("{mid} {midval}");
            total = total + midval;
        }

        buffer = String::new();
        reader.read_line(&mut buffer)?;
    }
    println!("{total}");

    
    Ok(())
}
