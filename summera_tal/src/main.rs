use std::io;
use std::io::prelude::*;

fn main() {                                                 // Kattis calls main function to run your solution.
    let input = io::stdin();                                // get standard input stream                                                         
    
    let mut numbers: Vec<usize> = input
        .lock()
        .lines()   
        .skip(1) // read only number line
        .next().unwrap().ok().unwrap()
        .split_whitespace()
        .map(|_number| _number.parse::<usize>().ok().unwrap())
        .collect();
    
    let _n = numbers.len();

    if _n == 0 {
        println!("0");
    } else if _n > 0 {
        let to_sum = (_n+1)/2;
        numbers.sort();

        let big_half = &numbers[(_n-to_sum)..];
        let sum: usize = big_half.iter().sum();
        println!("{}", sum);
    } else {
        println!("0");
    }
}
