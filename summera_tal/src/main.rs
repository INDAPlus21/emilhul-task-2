/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola Söderlund <violaso@kth.se>
 */

use std::io;
use std::io::prelude::*;

fn main() {                                                 // Kattis calls main function to run your solution.
    let input = io::stdin();                                // get standard input stream                                                         
                                                            
    let mut lines = input                                   // get input lines as strings
        .lock()                                             // see: https://doc.rust-lang.org/std/io/trait.BufRead.html
        .lines()
        .map(|_line| _line.ok().unwrap());
    
    let number_n = lines
        .next().unwrap()
        .parse::<usize>().unwrap();
    
    //eprintln!("The number n is {}", number_n);
    
    let numbers_to_sum = (number_n + 1)/2;

    //eprintln!("Ammount of numbers to sum {}", numbers_to_sum);

    let mut numbers = lines
        .next().unwrap()
        .split(" ")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    numbers.sort();

    let mut sum = 0;

    for number in numbers_to_sum-1..number_n {
        //eprintln!("Sum is: {}", sum);
        sum += numbers[number];
        //eprintln!("Number added: {}", numbers[number]);
    }
    
    println!("{}", sum)
    //eprintln!("The sum is: {}", sum);
}
