#![feature(iter_array_chunks)]
#![feature(exclusive_range_pattern)]
#![feature(get_many_mut)]
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

use crate::day1::Expedition;
use crate::day10::*;
use crate::day11::*;
use crate::day2::*;
use crate::day3::*;
use crate::day4::*;
use crate::day5::*;
use crate::day6::*;
use crate::day7::*;
use crate::day8::*;
use crate::day9::*;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
fn main() -> Result<(), Box<dyn Error>> {
    // if let Ok(lines) = read_lines("./src/day1/input.txt") {
    //     // Consumes the iterator, returns an (Optional) String
    //     let input: Vec<String> = lines.map(|x| x.unwrap()).collect();
    //     let exp = Expedition::try_from(input).unwrap();
    //     println!("highest: {}", exp.highest_calories());
    //     println!("highest top 3: {}", exp.highest_3_calories());
    // }
    let mut file = File::open("./src/day11/input.txt")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    // let day2_part1_result = day2::part1(&data);
    // println!("total score: {}", day2_part1_result);

    // let day2_part2_result = day2::part2(data);
    // println!("total score: {}", day2_part2_result);

    let day11_part2_result = day11::part2(data);
    println!("total priority: {}", day11_part2_result);
    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
