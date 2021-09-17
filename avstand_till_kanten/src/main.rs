use std::io;
use std::io::prelude::*;
use std::cmp;

fn main() {
    let input = io::stdin();

    let values = input                                                                  // get standard input stream
        .lock()
        .lines()
        .next().unwrap().ok().unwrap()
        .split_whitespace()
        .map(|_value| _value.parse::<usize>().ok().unwrap())
        .collect::<Vec<usize>>();
    
    let rows: usize = values[0];                                                        // split the vector into two values, rows and cols
    let cols: usize = values[1];

    for x in 1..=rows {                                                                 // for every row and every col
        for y in 1..=cols {
            let min_dist = cmp::min(cmp::min(x, rows-x+1), cmp::min(y, cols-y+1));      // get the smallest distance
            match min_dist {
                1 => print!("1"),
                2 => print!("2"),
                3 => print!("3"),
                4 => print!("4"),
                5 => print!("5"),
                6 => print!("6"),
                7 => print!("7"),
                8 => print!("8"),
                9 => print!("9"),
                _ => print!(".")
            }
        }
        println!();                                                                     // print empty line, skipping to the next one
    } 
}
