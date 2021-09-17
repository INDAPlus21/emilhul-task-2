use std::io;
use std::io::BufRead;

fn main() {
    let input = io::stdin();                                    // get standard input stream                                                         
    
    let mut numbers = input
        .lock()
        .lines()   
        .skip(1)                                                // skip line telling how many numbers and read only number line
        .next().unwrap().ok().unwrap()
        .split_whitespace()
        .map(|_number| _number.parse::<usize>().ok().unwrap())
        .collect::<Vec<usize>>();                               // collect the numbers into a vector.

    let _n = numbers.len();                                     

    if _n == 0 {
        println!("0");                                          // if we get an empty line len will be 0, we then return the sum of nothing (0)
    } else if _n > 0 {
        let n_sum = (_n+1)/2;                                   // due to how integer division this will be equal to _n/2 for even numbers and (_n+1)/2 for odd ones
        
        numbers.sort();
        let num_to_sum = &numbers[(_n-n_sum)..];                // get only the n_sum largest numbers from the vector

        let sum: usize = num_to_sum.iter().sum();               // iterate over the vecotr containing the largest numbers, adding all the elements together
        println!("{}", sum);
    } else {                                                    // should never execute since the length of numbers can only ever be 0 or >0
        println!("0");
    }
}
