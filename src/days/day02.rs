// adding this file causes the reading in of the "day01.txt" to fail???
// able to get it going by removing "data/day01.txt" to "day01.txt"
use std;
static NEWLINE: u8 = 10;
static SPACE: u8 = 32;
pub fn day02answers() {
    let mut filedata = std::fs::read("day02.txt").unwrap();

    filedata.retain(|&x| (x != NEWLINE) ); // remove unneeded symbols
    filedata.retain(|&x| (x != SPACE) ); // remove unneeded symbols

    // A for Rock, B for Paper, and C for Scissors - first column
    // X for Rock, Y for Paper, and Z for Scissors - first column

    if filedata.len() %2 != 0{
        panic!("Number of entries is not even")
    }

    let points = 0;
    
    for i in 0..filedata.len()/2{

        filedata[i]


    }
    println!("Day 02 Part 01: ")
}
