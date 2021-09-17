use std::io;
use std::io::prelude::*;
use std::cmp;
                                                            //
fn convert(num: &usize) -> &str {                           // a function that matches the distance numerical value to a string
    match num {
        1 => "1",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        9 => "9",
        _ => "."
    }
}

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
            print!("{}", convert(&min_dist));                                           // print the converted value
        }
        println!();                                                                     // print empty line, skipping to the next one
    } 
}
