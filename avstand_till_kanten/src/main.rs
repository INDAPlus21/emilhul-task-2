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
        let mut row: String = String::with_capacity(cols);                              // create a string for the row
        for y in 1..=cols {
            let min_dist = cmp::min(cmp::min(x, rows-x+1), cmp::min(y, cols-y+1));      // get the smallest distance
            match min_dist {                                                            // match the distane to the char and push it to the row string
                1 => row.push('1'),
                2 => row.push('2'),
                3 => row.push('3'),
                4 => row.push('4'),
                5 => row.push('5'),
                6 => row.push('6'),
                7 => row.push('7'),
                8 => row.push('8'),
                9 => row.push('9'),
                _ => row.push('.')
            }
        }
        println!("{}", row);                                                           // print the row string, and continue on a new line
    } 
}
