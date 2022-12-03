use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

use crate::day1::Expedition;

pub mod day1;

fn main() {
    
    if let Ok(lines) = read_lines("./src/day1/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let input: Vec<String> = lines.map(|x| x.unwrap()).collect();
        let exp = Expedition::try_from(input).unwrap();
        println!("highest: {}", exp.highest_calories());
        println!("highest top 3: {}", exp.highest_3_calories());
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
